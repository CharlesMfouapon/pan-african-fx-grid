use crate::currency::{Currency, ExchangeRate};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::algo;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::ops::Neg;

/// An arbitrage opportunity: a cycle of currency exchanges that yields profit.
#[derive(Debug, Clone)]
pub struct ArbitrageOpportunity {
    /// The cycle of currencies: c1 -> c2 -> ... -> cn -> c1
    pub cycle: Vec<Currency>,
    /// The product of exchange rates along the cycle.
    /// Value > 1.0 indicates profitable arbitrage.
    pub profitability: Decimal,
    /// Estimated profit in USD after transaction costs.
    pub estimated_profit_usd: Decimal,
    /// Total transaction cost along the cycle.
    pub transaction_cost: Decimal,
}

/// Detects arbitrage opportunities in a currency exchange graph.
///
/// Uses the Bellman-Ford algorithm on the negative log of exchange rates.
/// In the log space, an arbitrage cycle corresponds to a negative-weight cycle.
pub struct ArbitrageDetector {
    graph: DiGraph<Currency, Decimal>,
    currency_to_node: HashMap<Currency, NodeIndex>,
    transaction_fees: HashMap<(Currency, Currency), Decimal>,
}

impl ArbitrageDetector {
    /// Create a new detector with known transaction fees.
    /// Fees are expressed as a percentage (e.g., 0.02 = 2% fee).
    pub fn new(fees: HashMap<(Currency, Currency), Decimal>) -> Self {
        Self {
            graph: DiGraph::new(),
            currency_to_node: HashMap::new(),
            transaction_fees: fees,
        }
    }

    /// Add or update an exchange rate in the graph.
    pub fn update_rate(&mut self, rate: &ExchangeRate) {
        let from_idx = self.get_or_create_node(rate.from);
        let to_idx = self.get_or_create_node(rate.to);

        // Remove existing edge if present
        if let Some(edge) = self.graph.find_edge(from_idx, to_idx) {
            self.graph.remove_edge(edge);
        }

        self.graph.add_edge(from_idx, to_idx, rate.rate);
    }

    /// Find all arbitrage opportunities in the current graph.
    ///
    /// # Returns
    /// A vector of opportunities sorted by profitability (most profitable first).
    pub fn find_opportunities(&self) -> Vec<ArbitrageOpportunity> {
        let mut opportunities = Vec::new();

        // Build the negative log graph for Bellman-Ford
        let mut log_graph = DiGraph::<Currency, f64>::new();
        let mut log_nodes = HashMap::new();

        for edge in self.graph.edge_references() {
            let from = *self.graph.node_weight(edge.source()).unwrap();
            let to = *self.graph.node_weight(edge.target()).unwrap();
            let rate = *edge.weight();

            let from_idx = *log_nodes
                .entry(from)
                .or_insert_with(|| log_graph.add_node(from));
            let to_idx = *log_nodes
                .entry(to)
                .or_insert_with(|| log_graph.add_node(to));

            // Convert rate to negative log for Bellman-Ford
            let rate_f64: f64 = rate
                .to_string()
                .parse()
                .unwrap_or(0.0);

            if rate_f64 > 0.0 {
                let neg_log = -rate_f64.ln();
                log_graph.add_edge(from_idx, to_idx, neg_log);
            }
        }

        // For each node, try to find negative cycles
        for start_node in log_graph.node_indices() {
            if let Some(cycle) = self.find_negative_cycle(&log_graph, start_node) {
                if cycle.len() >= 3 {
                    // Convert cycle back to currencies and calculate profitability
                    let currencies: Vec<Currency> = cycle
                        .iter()
                        .map(|&idx| *log_graph.node_weight(idx).unwrap())
                        .collect();

                    if let Some(opportunity) = self.calculate_opportunity(&currencies) {
                        opportunities.push(opportunity);
                    }
                }
            }
        }

        // Sort by profitability descending
        opportunities.sort_by(|a, b| b.profitability.cmp(&a.profitability));
        opportunities.dedup_by(|a, b| a.cycle == b.cycle);

        opportunities
    }

    /// Find a negative cycle reachable from the given node using Bellman-Ford.
    fn find_negative_cycle(
        &self,
        graph: &DiGraph<Currency, f64>,
        start: NodeIndex,
    ) -> Option<Vec<NodeIndex>> {
        let node_count = graph.node_count();

        // Initialize distances
        let mut distances = vec![f64::INFINITY; node_count];
        let mut predecessors = vec![None; node_count];
        distances[start.index()] = 0.0;

        // Relax edges |V| - 1 times
        for _ in 0..node_count - 1 {
            for edge in graph.edge_references() {
                let u = edge.source().index();
                let v = edge.target().index();
                let weight = *edge.weight();

                if distances[u] < f64::INFINITY {
                    let new_dist = distances[u] + weight;
                    if new_dist < distances[v] - 1e-12 {
                        distances[v] = new_dist;
                        predecessors[v] = Some(edge.source());
                    }
                }
            }
        }

        // Check for negative cycles
        for edge in graph.edge_references() {
            let u = edge.source().index();
            let v = edge.target().index();
            let weight = *edge.weight();

            if distances[u] < f64::INFINITY && distances[u] + weight < distances[v] - 1e-12 {
                // Negative cycle detected. Reconstruct the cycle.
                let mut cycle = Vec::new();
                let mut visited = vec![false; node_count];
                let mut current = edge.source();

                while !visited[current.index()] {
                    visited[current.index()] = true;
                    cycle.push(current);
                    if let Some(pred) = predecessors[current.index()] {
                        current = pred;
                    } else {
                        break;
                    }
                }

                // Add the node that closes the cycle
                cycle.push(edge.target());

                if cycle.len() >= 3 {
                    return Some(cycle);
                }
            }
        }

        None
    }

    /// Calculate the profitability of a currency cycle.
    fn calculate_opportunity(&self, cycle: &[Currency]) -> Option<ArbitrageOpportunity> {
        if cycle.len() < 3 {
            return None;
        }

        let mut product = Decimal::ONE;
        let mut total_fee_pct = Decimal::ZERO;

        for i in 0..cycle.len() {
            let from = cycle[i];
            let to = cycle[(i + 1) % cycle.len()];

            // Find the exchange rate for this edge
            let from_idx = self.currency_to_node.get(&from)?;
            let to_idx = self.currency_to_node.get(&to)?;
            let edge = self.graph.find_edge(*from_idx, *to_idx)?;
            let rate = *self.graph.edge_weight(edge)?;

            product *= rate;

            // Accumulate transaction fees
            let fee = self
                .transaction_fees
                .get(&(from, to))
                .copied()
                .unwrap_or(Decimal::from_f64(0.01).unwrap()); // Default 1% fee
            total_fee_pct += fee;
        }

        let profitability = product;
        let transaction_cost = product * total_fee_pct;
        let net_profit = product - Decimal::ONE - transaction_cost;
        let estimated_profit_usd = net_profit * Decimal::from_f64(1000.0).unwrap(); // Assuming $1000 notional

        if profitability > Decimal::ONE + transaction_cost {
            Some(ArbitrageOpportunity {
                cycle: cycle.to_vec(),
                profitability,
                estimated_profit_usd,
                transaction_cost,
            })
        } else {
            None
        }
    }

    fn get_or_create_node(&mut self, currency: Currency) -> NodeIndex {
        *self
            .currency_to_node
            .entry(currency)
            .or_insert_with(|| self.graph.add_node(currency))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_triangular_arbitrage_detection() {
        let mut fees = HashMap::new();
        fees.insert((Currency::XOF, Currency::NGN), dec!(0.02));
        fees.insert((Currency::NGN, Currency::USD), dec!(0.01));
        fees.insert((Currency::USD, Currency::XOF), dec!(0.02));

        let mut detector = ArbitrageDetector::new(fees);

        // Set up rates that create an arbitrage:
        // 1 XOF -> 1.5 NGN -> 0.0015 USD -> 1.02 XOF (profit of 0.02 XOF per XOF)
        detector.update_rate(&ExchangeRate {
            from: Currency::XOF,
            to: Currency::NGN,
            rate: dec!(1.5),
            source: crate::currency::RateSource::CentralBank {
                country: "Nigeria".into(),
            },
            timestamp_ns: 0,
            confidence: 1.0,
        });

        detector.update_rate(&ExchangeRate {
            from: Currency::NGN,
            to: Currency::USD,
            rate: dec!(0.001),
            source: crate::currency::RateSource::CentralBank {
                country: "Nigeria".into(),
            },
            timestamp_ns: 0,
            confidence: 1.0,
        });

        detector.update_rate(&ExchangeRate {
            from: Currency::USD,
            to: Currency::XOF,
            rate: dec!(680.0), // This creates the arbitrage: 1.5 * 0.001 * 680 = 1.02
            source: crate::currency::RateSource::Parallel {
                market: "Wuse".into(),
                verified_by: vec![],
            },
            timestamp_ns: 0,
            confidence: 0.8,
        });

        let opportunities = detector.find_opportunities();
        // We should detect at least one opportunity
        // (The exact cycle depends on graph structure)
        assert!(!opportunities.is_empty());
    }
}
