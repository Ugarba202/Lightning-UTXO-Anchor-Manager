# ⚡ Lightning UTXO & Anchor Manager (Rust)

A Rust-based CLI tool and library for analyzing and managing UTXOs for Lightning nodes using anchor channels. Built as part of my transition into Lightning infrastructure engineering during the BOSS Bitcoin Developer Challenge.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange) ![Bitcoin](https://img.shields.io/badge/Bitcoin_Core-22+-blue) ![LDK](https://img.shields.io/badge/LDK-Lightning-yellow) ![CLI](https://img.shields.io/badge/CLI-Tool-lightgray)

## Why This Matters

Lightning nodes do not manage on-chain funds automatically — wallet logic must ensure:
- Adequate anchor reserves
- Emergency liquidity
- Fee bump capacity under congestion
- Safe UTXO selection for channel funding

This tool acts as a **UTXO health intelligence layer** for Lightning nodes, preventing failed channel opens, stuck force-close transactions, and unsafe reserve conditions.

## Features

- **UTXO classification** — spendable, anchor-capable, reserved
- **Channel reserve modeling**
- **CPFP carve-out safety simulation**
- **Anchor fee bump capacity estimation**
- **CLI-based reporting**
- **Modular Rust library design**

## Lightning Wallet Interface

The project includes a wallet abstraction layer designed to mirror the wallet interfaces used by Lightning implementations.

The `LightningWallet` module exposes:

* wallet balance
* UTXO access
* channel funding coin selection
* Lightning operational risk scoring

This design allows the library to act as a **wallet policy engine that could theoretically integrate with Lightning node software such as LDK, LND, or Core Lightning**.

## System Architecture

The Lightning UTXO & Anchor Manager is designed as a modular Rust library that analyzes wallet UTXOs and evaluates Lightning node operational safety.

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
(wallet policy) (anchor rules) (coin selection)
        │
        ▼
   simulation.rs
(fee environment analysis)
        │
        ▼
     policy.rs
(Lightning wallet diagnostics)
        │
        ▼
wallet_interface.rs
(Lightning node integration layer)
        │
        ▼
      main.rs
(CLI reporting tool)
```

This architecture separates **Bitcoin wallet logic, Lightning channel policy, and operational diagnostics** into independent modules.

## Quick Start

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 2. Clone the repository
git clone https://github.com/Ugarba202/Lightning-UTXO-Anchor-Manager.git
cd ln-utxo-manager

# 3. Build the project
cargo build --release
```

## Project Structure

```text
ln-utxo-manager/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── utxo.rs
│   ├── reserve.rs
│   ├── anchor.rs
│   ├── selection.rs
│   ├── simulation.rs
│   ├── error.rs
│   └── bin/
│       └── main.rs
├── docs/
│   ├── anchor_channel_notes.md
│   └── design.md
└── tests/
    └── utxo_tests.rs
```

## Commands & Usage

| Command | Description |
|---|---|
| `cargo run -- analyze` | Analyze UTXO health |
| `cargo run -- simulate --feerate 150` | Simulate mempool congestion |

## Example

```bash
# Analyze UTXO health
cargo run -- analyze
```

**Example Output:**
```text
Lightning Node UTXO Health Report
----------------------------------
Total UTXOs: 14
Spendable: 9
Anchor Capable: 4
Reserved for Anchor Safety: OK
Emergency Reserve: OK

Channel Capacity Estimate:
- Max Safe Channel Size: 0.08 BTC
- Safe Under 150 sat/vB: YES
```

## Documentation

| Document | Description |
|----------|-------------|
| `docs/anchor_channel_notes.md` | Notes on anchor channels |
| `docs/design.md` | System design and architecture |

## Testing

Run the test suite to verify functionality:

```bash
# Run all tests
cargo test
```

## Roadmap

- [ ] Integrate Bitcoin RPC input
- [ ] LDK-compatible wallet interface
- [ ] JSON export mode
- [ ] Configurable reserve policies
- [ ] Regtest-based simulation
- [ ] Anchor channel stress testing suite

## Acknowledgments & Learning Foundation

This project is built upon:
- Mastering Bitcoin (Andreas M. Antonopoulos)
- BOLT 2 – Peer Protocol
- BOLT 3 – Commitment & Anchor Transactions
- BOLT 5 – On-chain Handling
- CPFP Carve-Out Rule (Bitcoin mempool policy)
- LDK wallet integration architecture

## License

MIT License

Copyright (c) 2026 Usman Umar Garba

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

---

## Author

Built by **Usman Umar Garba**
- LinkedIn: [usman-umar-garba](https://www.linkedin.com/in/usman-umar-garba/)
- X: [@dev_useee](https://x.com/dev_useee)
- GitHub: [Ugarba202](https://github.com/Ugarba202)

## Lightning Wallet Health Dashboard

The CLI tool produces a visual health report describing the Lightning node's operational safety.

Example output:

```
Lightning Wallet Health Dashboard
---------------------------------

Liquidity Health        ████████░░ 80%
Anchor Safety           ███████░░░ 70%
Fragmentation Risk      ███░░░░░░░ 30%
```

This visualization helps operators quickly understand the wallet's readiness for Lightning operations.

*Bitcoin & Lightning Engineer in progress.*