use std::process::Command;
use serde::Deserialize;
use crate::utxo::Utxo;

#[derive(Deserialize)]
struct RpcUtxo {
    txid: String,
    vout: u32,
    amount: f64,
    confirmations: u32,
}

pub fn fetch_utxos() -> Vec<Utxo> {
    let output = Command::new("bitcoin-cli")
        .args(["-signet", "-rpcwallet=lnwallet", "listunspent"])
        .output()
        .expect("Failed to run bitcoin-cli");

    let json = String::from_utf8(output.stdout)
        .expect("Invalid UTF8 from bitcoin-cli");

    let rpc_utxos: Vec<RpcUtxo> =
        serde_json::from_str(&json).expect("Invalid JSON");

    rpc_utxos
        .into_iter()
        .map(|u| Utxo {
            txid: u.txid,
            vout: u.vout,
            value: (u.amount * 100_000_000.0) as u64,
            confirmations: u.confirmations,
        })
        .collect()
}