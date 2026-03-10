use crate::error::ManagerError;
use crate::utxo::Utxo;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct RpcUtxo {
    txid: String,
    vout: u32,
    amount: f64,
    confirmations: u32,
}

pub fn fetch_utxos() -> Result<Vec<Utxo>, ManagerError> {
    let output = Command::new("bitcoin-cli")
        .args(["-signet", "-rpcwallet=lnwallet", "listunspent"])
        .output()
        .map_err(|e| ManagerError::RpcError(e.to_string()))?;

    if !output.status.success() {
        return Err(ManagerError::RpcError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let json = String::from_utf8(output.stdout)
        .map_err(|e| ManagerError::JsonParseError(e.to_string()))?;

    if json.trim().is_empty() {
        return Ok(Vec::new());
    }

    let rpc_utxos: Vec<RpcUtxo> =
        serde_json::from_str(&json).map_err(|e| ManagerError::JsonParseError(e.to_string()))?;

    let utxos = rpc_utxos
        .into_iter()
        .map(|u| Utxo {
            txid: u.txid,
            vout: u.vout,
            value: (u.amount * 100_000_000.0) as u64,
            confirmations: u.confirmations,
        })
        .collect();

    Ok(utxos)
}
