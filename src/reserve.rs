use crate::utxo::Utxo;

pub fn total_balance(utxos: &[Utxo]) -> u64 {
    utxos.iter().map(|u| u.value).sum()
}

pub fn spendable_utxos(utxos: &[Utxo], min_confirmations: u32) -> Vec<Utxo> {
    utxos
        .iter()
        .filter(|u| u.confirmations >= min_confirmations)
        .cloned()
        .collect()
}
pub fn available_after_reserve(
    utxos: &[Utxo],
    min_confirmations: u32,
    emergency_reserve: u64,
) -> u64 {
    let spendable = spendable_utxos(utxos, min_confirmations);
    let spendable_balance: u64 = spendable.iter().map(|u| u.value).sum();

    if spendable_balance > emergency_reserve {
        spendable_balance - emergency_reserve
    } else {
        0
    }
}
pub fn anchor_capable_utxos(
    utxos: &[Utxo],
    min_confirmations: u32,
    minimum_value: u64,
) -> Vec<Utxo> {
    utxos
        .iter()
        .filter(|u| u.confirmations >= min_confirmations && u.value >= minimum_value)
        .cloned()
        .collect()
}

pub fn max_safe_channel_size(
    utxos: &[Utxo],
    min_confirmations: u32,
    emergency_reserve: u64,
    channel_buffer: u64,
) -> u64 {
    let usable = available_after_reserve(utxos, min_confirmations, emergency_reserve);

    if usable > channel_buffer {
        usable - channel_buffer
    } else {
        0
    }
}

pub fn fee_bump_capacity(anchor_utxos: &[Utxo], target_feerate: u64, assumed_tx_size: u64) -> u64 {
    let total_anchor_balance: u64 = anchor_utxos.iter().map(|u| u.value).sum();

    let required_fee = target_feerate * assumed_tx_size;

    if total_anchor_balance > required_fee {
        total_anchor_balance - required_fee
    } else {
        0
    }
}
pub fn fee_risk_status(
    anchor_utxos: &[Utxo],
    target_feerate: u64,
    assumed_tx_size: u64,
) -> &'static str {
    let capacity = fee_bump_capacity(anchor_utxos, target_feerate, assumed_tx_size);

    if capacity > 10_000 {
        "SAFE"
    } else if capacity > 0 {
        "LOW BUFFER"
    } else {
        "UNSAFE"
    }
}
