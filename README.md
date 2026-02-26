# ⚡ Lightning UTXO & Anchor Manager (Rust)

A Rust-based CLI tool and library for analyzing and managing UTXOs for Lightning nodes using anchor channels.

Built as part of my transition into Lightning infrastructure engineering during the BOSS Bitcoin Developer Challenge.

---

## 📌 Overview

Lightning nodes rely on on-chain Bitcoin transactions to:

- Open channels
- Close channels
- Handle force closes
- Perform anchor fee bumping (CPFP)

However, poor UTXO management can lead to:

- Failed channel opens
- Inability to bump fees
- Stuck force-close transactions
- Unsafe reserve conditions

This project provides a structured way to:

- Analyze Lightning wallet UTXOs
- Model anchor channel reserve requirements
- Simulate mempool fee spikes
- Evaluate fee bump safety
- Estimate safe channel capacity

---

## 🎯 Project Goals

- Build a reusable Rust library for Lightning-aware UTXO management
- Create a CLI tool for Lightning node operators
- Model anchor channel reserve and CPFP safety
- Simulate fee bump scenarios under congestion
- Bridge wallet-level logic with Lightning infrastructure needs

---

## 🧠 Why This Matters

Lightning nodes do not manage on-chain funds automatically — wallet logic must ensure:

- Adequate anchor reserves
- Emergency liquidity
- Fee bump capacity under congestion
- Safe UTXO selection for channel funding

This tool acts as a **UTXO health intelligence layer** for Lightning nodes.

---

## 🛠 Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| Bitcoin primitives | `rust-bitcoin` |
| CLI framework | `clap` |
| Error handling | `thiserror` |
| Serialization | `serde` |
| Testing | `cargo test` |

---

## 📁 Project Structure
ln-utxo-manager/
├── Cargo.toml
├── README.md
├── src/
│ ├── lib.rs
│ ├── utxo.rs
│ ├── reserve.rs
│ ├── anchor.rs
│ ├── selection.rs
│ ├── simulation.rs
│ ├── error.rs
│ └── bin/
│ └── main.rs
├── docs/
│ ├── anchor_channel_notes.md
│ └── design.md
└── tests/
└── utxo_tests.rs

---

---

## 🚀 Installation

### 1️⃣ Install Rust

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
2️⃣ Clone the repository
git clone https://github.com/Ugarba202/Lightning-UTXO-Anchor-Manager
cd ln-utxo-manager
3️⃣ Build
cargo build --release
📦 Usage
Analyze UTXO health
cargo run -- analyze
Simulate mempool congestion
cargo run -- simulate --feerate 150
🖥 Example Output
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
⚡ Core Features

✔ UTXO classification (spendable, anchor-capable, reserved)
✔ Channel reserve modeling
✔ CPFP carve-out safety simulation
✔ Anchor fee bump capacity estimation
✔ CLI-based reporting
✔ Modular Rust library design

📚 Learning & Research Foundation

This project is built upon:

Mastering Bitcoin (Andreas M. Antonopoulos)

BOLT 2 – Peer Protocol

BOLT 3 – Commitment & Anchor Transactions

BOLT 5 – On-chain Handling

CPFP Carve-Out Rule (Bitcoin mempool policy)

LDK wallet integration architecture

📈 Roadmap

 Integrate Bitcoin RPC input

 LDK-compatible wallet interface

 JSON export mode

 Configurable reserve policies

 Regtest-based simulation

 Anchor channel stress testing suite

🧪 Testing
cargo test
🤝 Contributing

Contributions are welcome:

Improved fee modeling

Better anchor simulation

LDK integration improvements

Documentation updates

Open an issue to discuss ideas.

🧭 Author

Built by Usman Umar Garba
https://www.linkedin.com/in/usman-umar-garba/
https://x.com/dev_useee
https://github.com/Ugarba202
Bitcoin &  Lightning  Engineer in progress.