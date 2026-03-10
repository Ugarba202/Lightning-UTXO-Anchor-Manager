use crate::policy::lightning_risk_score;
use crate::selection::select_utxos_for_channel;
use crate::utxo::Utxo;

/// Lightning wallet abstraction
///
/// This struct acts as a simplified wallet interface similar to
/// the wallet layers used by Lightning implementations.
pub struct LightningWallet {
    utxos: Vec<Utxo>,
}

impl LightningWallet {
    /// Create a new wallet instance
    pub fn new(utxos: Vec<Utxo>) -> Self {
        Self { utxos }
    }

    /// Return wallet balance in sats
    pub fn balance(&self) -> u64 {
        self.utxos.iter().map(|u| u.value).sum()
    }

    /// Return all wallet UTXOs
    pub fn utxos(&self) -> &Vec<Utxo> {
        &self.utxos
    }

    /// Select coins for channel funding
    pub fn select_channel_coins(&self, amount: u64) -> Vec<Utxo> {
        select_utxos_for_channel(&self.utxos, amount)
    }

    /// Compute Lightning operational risk score
    pub fn risk_score(&self) -> u8 {
        lightning_risk_score(&self.utxos, 40_000)
    }
}
