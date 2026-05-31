<!-- Custom badges for Pan-African FX Grid -->
![built by: Charles Mfouapon](https://img.shields.io/badge/built%20by-Charles%20Mfouapon-1A1A1A?labelColor=D4A017&style=for-the-badge)
![region: Pan-African](https://img.shields.io/badge/region-Pan--African-006B3F?labelColor=E8B80D&style=for-the-badge)
![settlement: Mobile Money Rails](https://img.shields.io/badge/settlement-Mobile%20Money%20Rails-C8102E?labelColor=F5F5F0&style=for-the-badge)
![consensus: Byzantine Fault Tolerant](https://img.shields.io/badge/consensus-Byzantine%20Fault%20Tolerant-1A1A1A?labelColor=D4A017&style=for-the-badge)
![arbitrage: Real-time Detection](https://img.shields.io/badge/arbitrage-Real--time%20Detection-006B3F?labelColor=F5F5F0&style=for-the-badge)
![currencies: XOF XAF NGN GHS KES ZAR](https://img.shields.io/badge/currencies-XOF%20XAF%20NGN%20GHS%20KES%20ZAR-1E3A5F?labelColor=E8B80D&style=for-the-badge)
![rust: Core Engine](https://img.shields.io/badge/rust-Core%20Engine-1A1A1A?labelColor=F5F5F0&logo=rust&style=for-the-badge)
![python: Quant Models](https://img.shields.io/badge/python-Quant%20Models-1E3A5F?labelColor=E8B80D&logo=python&style=for-the-badge)
![typescript: Dashboard](https://img.shields.io/badge/typescript-Dashboard-1A1A1A?labelColor=D4A017&logo=typescript&style=for-the-badge)

---

# Pan-African FX Grid

**Decentralized foreign exchange liquidity mesh for African currency markets.**

African FX markets are structurally different from Western markets.
Fragmented liquidity. Mobile money rails. Parallel market dynamics.
This protocol is purpose-built for those realities.

## The Problem

| Western FX | African FX |
|---|---|
| Centralized ECNs | Mobile money fragmentation |
| SWIFT settlement | Provider-specific APIs |
| Deep liquidity pools | Thin, fragmented markets |
| Sub-microsecond latency | Variable connectivity |
| Single exchange rate | Official/parallel spread |

## Architecture
```

UEMOA Fragment ←→ WAMZ Fragment ←→ EAC Fragment
↑                ↑                ↑
└────────────────┴────────────────┘
│
Arbitrage Detection
(Bellman-Ford cycles)
│
Mobile Money Settlement
(MTN | Orange | M-Pesa | Wave)
│
BFT Consensus Layer
(Rate attestation + settlement proof)

```

## Key Components

| Component | Language | Description |
|---|---|---|
| Arbitrage Engine | Rust | Bellman-Ford negative cycle detection across currency digraph |
| Rate Oracle Network | Rust | Median-based aggregation with modified z-score outlier rejection |
| Quant Models | Python | Market simulation, volatility surfaces, regime detection |
| Settlement Adapter | Rust | Mobile money API integration layer |
| Dashboard | TypeScript | Real-time visualization of liquidity mesh |

## Supported Currencies

XOF XAF NGN GHS KES UGX TZS RWF ZAR ETB EGP MAD

## Supported Mobile Money Providers

MTN Mobile Money | Orange Money | M-Pesa | Wave | Moov Money

## Quick Start

```bash
# Build the arbitrage engine
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate personalized badges
python scripts/generate_badges.py
```

## Research

Read the [Protocol Specification](SPECIFICATION.md) for the complete
architectural design, threat model, and performance targets.

Author: Charles Mfouapon

Building financial infrastructure for African markets.

