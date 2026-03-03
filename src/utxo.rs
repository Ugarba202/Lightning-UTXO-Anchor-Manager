#[derive(Debug, Clone)]
pub struct Utxo {
    pub txid: String, // Transaction ID where the coins are Created
    pub vout: u32, // Output index inside the transactions (Because one transaction can create many outputs)
    pub value: u64, // Amount in satoshi (we use satoshi because 1- integers are safe 2-> no floating point errors
    pub confirmations: u32, // how many blokcs deeps
}
