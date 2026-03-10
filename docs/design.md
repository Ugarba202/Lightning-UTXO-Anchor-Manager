# System Design – Lightning UTXO & Anchor Manager

## Overview

The Lightning UTXO & Anchor Manager is a Rust library and CLI tool designed to analyze Bitcoin wallet UTXOs and determine whether they are suitable for operating a Lightning node using **anchor channels**.

Lightning nodes rely on on-chain funds to open channels, close channels, and bump transaction fees during congestion. Poor UTXO management can cause failures such as:

* inability to open channels
* stuck force-close transactions
* inability to CPFP bump anchor commitments
* loss of operational liquidity

This project acts as a **wallet policy engine** that evaluates whether a wallet is safe for Lightning operations.

---

## Design Goals

The project was designed with the following goals:

1. Provide **UTXO health analysis** for Lightning node operators.
2. Simulate **fee spike scenarios** and CPFP fee bumping capability.
3. Detect **UTXO fragmentation** that may prevent channel funding.
4. Provide a **modular Rust library** usable by Lightning implementations.
5. Offer a **CLI reporting tool** for quick wallet diagnostics.

---

## High-Level Architecture

The project is structured as a modular Rust library with a CLI interface.

```
Bitcoin Core RPC
        │
        ▼
     rpc.rs
(fetch wallet UTXOs)
        │
        ▼
      utxo.rs
(UTXO data model)
        │
        ▼
 ┌───────────────┬───────────────┬───────────────┐
 ▼               ▼               ▼
reserve.rs     anchor.rs      selection.rs
(wallet policy) (anchor rules) (UTXO selection)
        │
        ▼
   simulation.rs
(fee environment analysis)
        │
        ▼
     main.rs
(CLI reporting interface)
```

---

## Module Responsibilities

### utxo.rs

Defines the core `Utxo` data structure used throughout the project.

Responsibilities:

* represent wallet coins
* store value and confirmation state
* act as the input for all wallet policy checks

---

### reserve.rs

Implements Lightning wallet reserve policies.

Responsibilities:

* calculate wallet balance
* determine spendable UTXOs
* enforce emergency reserve requirements

Lightning nodes must maintain on-chain liquidity to handle unexpected channel closures and fee spikes.

---

### anchor.rs

Implements anchor-channel-specific rules.

Responsibilities:

* identify anchor-capable UTXOs
* enforce minimum UTXO size requirements
* estimate maximum safe channel funding amount

Anchor channels require additional liquidity to support fee bumping.

---

### selection.rs

Implements UTXO selection and wallet health checks.

Responsibilities:

* detect wallet fragmentation
* recommend consolidation candidates
* select UTXOs suitable for channel funding

Wallet fragmentation is a common operational issue for Lightning nodes.

---

### simulation.rs

Simulates mempool congestion environments.

Responsibilities:

* estimate CPFP fee bump capacity
* classify fee safety risk
* simulate different feerates

The simulation models how the wallet would behave during high fee environments.

---

### rpc.rs

Provides integration with Bitcoin Core.

Responsibilities:

* call `bitcoin-cli`
* retrieve wallet UTXOs
* convert RPC responses into the internal `Utxo` structure

---

### error.rs

Defines custom error types used across the project.

Responsibilities:

* handle RPC failures
* handle JSON parsing errors
* enforce safe error propagation

---

## CLI Tool

The CLI tool provides a high-level report describing the wallet's Lightning readiness.

Example workflow:

1. Fetch UTXOs from Bitcoin Core
2. Analyze wallet reserves
3. Identify anchor-capable coins
4. Simulate fee spike scenarios
5. Detect wallet fragmentation
6. Recommend channel funding coins

The output is designed to resemble a **Lightning node health diagnostic report**.

---

## Future Improvements

Potential future improvements include:

* Lightning Development Kit (LDK) integration
* automated UTXO consolidation suggestions
* advanced channel funding strategies
* JSON API output for monitoring tools
* mempool-aware fee prediction

---

## Summary

The Lightning UTXO & Anchor Manager acts as a **wallet policy intelligence layer** for Lightning nodes. By analyzing wallet UTXOs and simulating fee conditions, it helps operators maintain safe and reliable Lightning infrastructure.
