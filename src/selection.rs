use crate::utxo::Utxo;

/// Threshold below which UTXOs are considered too small
pub const SMALL_UTXO_THRESHOLD: u64 = 20_000;

/// Detect wallet fragmentation by counting small UTXOs
pub fn detect_fragmentation(utxos: &[Utxo]) -> bool {
    let small_count = utxos
        .iter()
        .filter(|u| u.value < SMALL_UTXO_THRESHOLD)
        .count();

    small_count >= 3
}

/// Suggest UTXOs that should be consolidated
pub fn consolidation_candidates(utxos: &[Utxo]) -> Vec<Utxo> {
    utxos
        .iter()
        .filter(|u| u.value < SMALL_UTXO_THRESHOLD)
        .cloned()
        .collect()
}

/// Select UTXOs to fund a Lightning channel
/// Uses a simple greedy algorithm
pub fn select_utxos_for_channel(
    utxos: &[Utxo],
    target_amount: u64,
) -> Vec<Utxo> {

    let mut selected = Vec::new();
    let mut total = 0;

    let mut sorted = utxos.to_vec();

    // sort largest → smallest
    sorted.sort_by(|a, b| b.value.cmp(&a.value));

    for u in sorted {
        selected.push(u.clone());
        total += u.value;

        if total >= target_amount {
            break;
        }
    }

    selected
}