# Pan-African FX Grid: Protocol Specification

- **Version:** 0.1.0-draft
- **Author:** Charles Mfouapon
- **Status:** Research & Development

## Abstract

The Pan-African FX Grid is a decentralized foreign exchange liquidity mesh
designed for the unique structural characteristics of African currency markets.
Unlike Western FX markets with centralized ECNs and deep liquidity pools,
African FX markets are characterized by:

1. **Fragmented liquidity** across multiple mobile money providers (MTN Mobile Money, 
   Orange Money, Moov Money, Wave, M-Pesa)
2. **Significant spreads** between official and parallel market rates
3. **Cross-border friction** due to limited correspondent banking relationships
4. **Settlement latency** measured in days rather than microseconds
5. **Currency basket effects** from CFA franc zones, ECOWAS, SADC, EAC regions

This protocol addresses these challenges with a novel architecture that
combines Byzantine fault-tolerant consensus, statistical arbitrage detection,
and mobile money settlement rails.

## Core Innovations

### 1. Regional Liquidity Fragments
Rather than a global order book, liquidity is organized into **regional
fragments** corresponding to monetary zones:
- **UEMOA Fragment:** XOF-pegged liquidity (8 West African nations)
- **CEMAC Fragment:** XAF-pegged liquidity (6 Central African nations)
- **WAMZ Fragment:** NGN, GHS, GMD, SLL, LRD
- **EAC Fragment:** KES, UGX, TZS, RWF, BIF
- **SADC Fragment:** ZAR, BWP, NAD, LSL, SZL

Each fragment maintains its own order book and connects to others through
a mesh of liquidity providers.

### 2. Mobile Money Settlement Layer
Settlement does not use SWIFT or correspondent banking. Instead, it uses
**mobile money rails** directly:
- MTN Mobile Money API (available in 16 African countries)
- Orange Money API (available in 17 African countries)
- M-Pesa API (available in 7 African countries)
- Wave API (available in 5 West African countries)

Settlement finality is achieved through provider-specific confirmation
semantics, with cryptographic proofs of settlement posted to the mesh.

### 3. Arbitrage Detection Engine
Identifies cross-border arbitrage opportunities by monitoring:
- Official central bank rates
- Parallel market rates (crowdsourced and verified)
- Mobile money provider exchange rates
- Crypto-fiat on/off ramps (Binance P2P, Paxful, Yellow Card)

The engine computes **arbitrage cycles** across directed graphs of
currency pairs, flagging opportunities that exceed threshold spreads
after accounting for mobile money transaction fees.

### 4. Byzantine Fault-Tolerant Mesh
Liquidity providers form a BFT consensus group per regional fragment.
Providers stake native tokens (or reputational collateral) and attest
to the validity of exchange rates they observe in their local markets.

The consensus protocol is a simplified Practical Byzantine Fault Tolerance
(PBFT) variant optimized for high-latency, low-bandwidth environments
typical of African internet infrastructure.

### 5. Rate Oracle Network
Market rates are sourced from a hybrid oracle network:
- **Primary feeds:** Central bank APIs where available
- **Secondary feeds:** Mobile money provider exchange APIs
- **Tertiary feeds:** Verified crowd-sourced submissions from licensed
  bureau de change operators
- **Validation:** Median-based aggregation with outlier rejection using
  modified z-score thresholding

## System Architecture
