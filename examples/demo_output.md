# Lightning UTXO & Anchor Manager — Demo Output

This document shows example outputs produced by the CLI tool.

The examples demonstrate how the system analyzes wallet liquidity, anchor safety, and Lightning operational risk.

---

# CLI Mode

Command:

```
cargo run
```

Example Output:

```
⚡ Lightning Node UTXO Health Report
------------------------------------

Total balance: 80000 sats
Spendable balance (>=3 conf): 50000 sats
Usable balance (after reserve): 30000 sats

Anchor-capable balance: 50000 sats
Max safe channel size: 25000 sats

Fee bump simulation (150 sat/vB):
Remaining after fee bump: 20000 sats
Risk status: SAFE

Fee Simulation Table
--------------------
Feerate    Remaining      Status
50 sat/vB   40000 sats      SAFE
150 sat/vB   20000 sats      SAFE
300 sat/vB   0 sats          UNSAFE
500 sat/vB   0 sats          UNSAFE

Fragmentation Analysis
----------------------
Wallet UTXO distribution looks healthy

Channel Funding Recommendation
------------------------------
Suggested UTXOs for channel funding:
50000 sats

Lightning Wallet Policy Analysis
--------------------------------
Wallet liquidity: HEALTHY
Anchor Fee Safety Score: 70/100

Channel Funding Strategy
------------------------
Recommended strategy: Multiple smaller channels
Channel 1 → 40000 sats
Channel 2 → 40000 sats

Lightning Node Risk Score
-------------------------
Operational Safety Score: 82/100

Lightning Wallet Health Dashboard
---------------------------------

Liquidity Health        ████████░░ 80%
Anchor Safety           ███████░░░ 70%
Fragmentation Risk      ███░░░░░░░ 30%
```

---

# JSON Mode

Command:

```
cargo run -- --json
```

Example Output:

```json
{
  "total_balance": 80000,
  "spendable_balance": 50000,
  "usable_balance": 30000,
  "anchor_balance": 50000,
  "max_channel_size": 25000,
  "risk_score": 82
}
```

This output format allows the tool to be integrated with monitoring systems or external automation scripts.

---

# Use Cases

This tool can be used for:

* Lightning node wallet diagnostics
* liquidity health monitoring
* anchor channel safety evaluation
* UTXO fragmentation analysis
* channel funding strategy planning

---

# Notes

The examples above use simulated wallet data. When connected to Bitcoin Core, the tool retrieves real UTXOs via RPC and performs the same analysis on the live wallet state.
