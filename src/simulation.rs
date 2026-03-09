use crate::utxo::Utxo;
use crate::reserve::anchor_capable_utxos;
use crate::reserve::fee_bump_capacity;
use crate::reserve::fee_risk_status;

pub fn simulate_fee_levels(utxos: &[Utxo]) {
    let fee_levels = [50, 150, 300, 500];

    println!("\nFee Simulation Table");
    println!("--------------------");
    println!("Feerate    Remaining      Status");

    let anchor_utxos = anchor_capable_utxos(utxos, 3, 40_000);

    for feerate in fee_levels {
        let remaining = fee_bump_capacity(&anchor_utxos, feerate, 200);

        let status = fee_risk_status(&anchor_utxos, feerate, 200);

        println!(
            "{} sat/vB   {} sats      {}",
            feerate, remaining, status
        );
    }
}