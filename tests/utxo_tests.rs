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
