use clap::Parser;

use lighting_utxo_anchor_manager::reserve::{
    anchor_capable_utxos, available_after_reserve, fee_bump_capacity, fee_risk_status,
    max_safe_channel_size, spendable_utxos, total_balance,
};
use lighting_utxo_anchor_manager::utxo::Utxo;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = 150)]
    feerate: u64,

    #[arg(long, default_value_t = 200)]
    tx_size: u64,

    #[arg(long, default_value_t = 20_000)]
    reserve: u64,

    #[arg(long, default_value_t = 5_000)]
    channel_buffer: u64,
}

fn main() {
    let args = Args::parse();

    let utxos = vec![
        Utxo {
            txid: "abc123".to_string(),
            vout: 0,
            value: 50_000,
            confirmations: 6,
        },
        Utxo {
            txid: "def456".to_string(),
            vout: 1,
            value: 30_000,
            confirmations: 1,
        },
    ];

    let balance = total_balance(&utxos);

    let spendable = spendable_utxos(&utxos, 3);
    let spendable_balance: u64 = spendable.iter().map(|u| u.value).sum();

    let usable = available_after_reserve(&utxos, 3, args.reserve);

    let anchor_utxos = anchor_capable_utxos(&utxos, 3, 40_000);
    let anchor_balance: u64 = anchor_utxos.iter().map(|u| u.value).sum();

    let max_channel = max_safe_channel_size(&utxos, 3, args.reserve, args.channel_buffer);

    let fee_capacity = fee_bump_capacity(&anchor_utxos, args.feerate, args.tx_size);

    let risk = fee_risk_status(&anchor_utxos, args.feerate, args.tx_size);

    println!("\n⚡ Lightning Node UTXO Health Report");
    println!("------------------------------------");

    println!("Total balance: {} sats", balance);
    println!("Spendable balance (>=3 conf): {} sats", spendable_balance);
    println!("Usable balance (after reserve): {} sats", usable);

    println!("\nAnchor-capable balance: {} sats", anchor_balance);
    println!("Max safe channel size: {} sats", max_channel);

    println!("\nFee bump simulation ({} sat/vB):", args.feerate);
    println!("Remaining after fee bump: {} sats", fee_capacity);
    println!("Risk status: {}", risk);
}
