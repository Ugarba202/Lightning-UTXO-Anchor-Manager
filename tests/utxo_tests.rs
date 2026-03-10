use lighting_utxo_anchor_manager::reserve::total_balance;
use lighting_utxo_anchor_manager::utxo::Utxo;

#[test]
fn test_total_balance() {
    let utxos = vec![
        Utxo {
            txid: "tx1".to_string(),
            vout: 0,
            value: 50_000,
            confirmations: 6,
        },
        Utxo {
            txid: "tx2".to_string(),
            vout: 1,
            value: 30_000,
            confirmations: 3,
        },
    ];

    let balance = total_balance(&utxos);

    assert_eq!(balance, 80_000);
}

#[test]
fn test_anchor_capable_utxos() {
    use lighting_utxo_anchor_manager::anchor::anchor_capable_utxos;

    let utxos = vec![
        Utxo {
            txid: "tx1".to_string(),
            vout: 0,
            value: 50_000,
            confirmations: 6,
        },
        Utxo {
            txid: "tx2".to_string(),
            vout: 1,
            value: 30_000,
            confirmations: 6,
        },
        Utxo {
            txid: "tx3".to_string(),
            vout: 2,
            value: 100_000,
            confirmations: 1,
        },
    ];

    // min_conf = 3, min_val = 40,000 -> Only tx1 matches
    let anchor_capable = anchor_capable_utxos(&utxos, 3, 40_000);
    assert_eq!(anchor_capable.len(), 1);
    assert_eq!(anchor_capable[0].txid, "tx1");
}

#[test]
fn test_max_safe_channel_size() {
    use lighting_utxo_anchor_manager::anchor::max_safe_channel_size;

    let utxos = vec![
        Utxo {
            txid: "tx1".to_string(),
            vout: 0,
            value: 100_000,
            confirmations: 6,
        },
        Utxo {
            txid: "tx2".to_string(),
            vout: 1,
            value: 50_000,
            confirmations: 6,
        },
    ];

    // total = 150,000. reserve = 20,000, buffer = 5,000.
    // expected = 150k - 20k - 5k = 125k
    let max_size = max_safe_channel_size(&utxos, 3, 20_000, 5000);
    assert_eq!(max_size, 125_000);
}
