# Anchor Channels – Technical Notes

## Introduction

Anchor channels are a Lightning Network channel type designed to improve fee bumping reliability during force closes.

Traditional Lightning commitment transactions often became stuck when network fees increased unexpectedly. Anchor outputs were introduced to allow nodes to attach Child-Pays-For-Parent (CPFP) transactions to increase the effective fee rate.

---

## The Problem Anchor Channels Solve

When a Lightning channel is force-closed, a commitment transaction must confirm on-chain.

If the commitment transaction has a **low fee rate**, miners may ignore it during periods of congestion.

Without a way to increase the fee, funds can remain stuck in the mempool.

This creates several operational risks:

* delayed channel settlement
* HTLC timeout failures
* liquidity lockup

---

## Anchor Output Design

Anchor channels introduce small additional outputs called **anchor outputs**.

These outputs allow the channel participant to create a CPFP transaction that increases the effective fee of the commitment transaction.

The process works like this:

1. Commitment transaction is broadcast
2. Anchor output becomes spendable
3. Node creates a child transaction spending the anchor
4. The child pays a higher fee
5. Miners include both transactions

This mechanism ensures the commitment transaction can confirm even under high fee conditions.

---

## CPFP Carve-Out Rule

Bitcoin mempool policy includes a special rule called the **CPFP carve-out**.

This rule allows a small child transaction to attach to a low-fee parent transaction without being rejected by mempool limits.

This rule is essential for Lightning anchor channels because it guarantees that the fee bump transaction will be accepted by nodes.

---

## Why UTXO Management Matters

Even with anchor outputs, Lightning nodes still require sufficient wallet funds to pay for fee bumps.

If the wallet contains only small or fragmented UTXOs, the node may not be able to construct an effective CPFP transaction.

Good Lightning wallet management requires:

* sufficiently large UTXOs
* emergency reserve liquidity
* protection against fragmentation
* ability to handle fee spikes

---

## Relevance to This Project

The Lightning UTXO & Anchor Manager analyzes wallet UTXOs to determine whether a Lightning node can safely operate anchor channels.

The tool evaluates:

* anchor-capable UTXOs
* wallet reserve safety
* fee bump capacity
* UTXO fragmentation

These checks help Lightning operators avoid operational failures caused by poor wallet liquidity management.

---

## Further Reading

* BOLT 3 – Commitment Transactions
* BOLT 5 – On-Chain Handling
* Bitcoin CPFP policy rules
* Lightning Development Kit documentation
