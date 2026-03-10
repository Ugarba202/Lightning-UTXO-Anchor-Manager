use crate::utxo::Utxo;

/// Minimum value for a UTXO to be considered anchor-capable.
/// In practice Lightning implementations prefer reasonably sized coins.
pub const MIN_ANCHOR_UTXO: u64 = 40_000;

/// Filter UTXOs that can support anchor channels
pub fn anchor_capable_utxos(
    utxos: &[Utxo],
    min_confirmations: u32,
    min_value: u64,
) -> Vec<Utxo> {
    utxos
        .iter()
        .filter(|u| {
            u.confirmations >= min_confirmations &&
            u.value >= min_value
        })
        .cloned()
        .collect()
}

/// Estimate the maximum safe channel size that can be opened
/// while respecting wallet reserve policy.
pub fn max_safe_channel_size(
    utxos: &[Utxo],
    min_confirmations: u32,
    reserve: u64,
    channel_buffer: u64,
) -> u64 {
    let spendable: Vec<&Utxo> = utxos
        .iter()
        .filter(|u| u.confirmations >= min_confirmations)
        .collect();

    let total: u64 = spendable.iter().map(|u| u.value).sum();

    if total <= reserve + channel_buffer {
        return 0;
    }

    total - reserve - channel_buffer
}

/// Determine whether wallet meets minimum anchor reserve requirements
pub fn anchor_reserve_required(utxos: &[Utxo]) -> bool {
    let total: u64 = utxos.iter().map(|u| u.value).sum();
    total >= MIN_ANCHOR_UTXO
}