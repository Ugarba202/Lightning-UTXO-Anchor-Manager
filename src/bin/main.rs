use clap::Parser;
use serde::Serialize;
use lighting_utxo_anchor_manager::anchor::{anchor_capable_utxos, max_safe_channel_size};
use lighting_utxo_anchor_manager::policy::{
    anchor_fee_safety_score, lightning_risk_score, render_health_bar,
    suggest_channel_strategy, ChannelStrategy,
};
use lighting_utxo_anchor_manager::reserve::{
    available_after_reserve, fee_bump_capacity, fee_risk_status, spendable_utxos, total_balance,
};
use lighting_utxo_anchor_manager::rpc::fetch_utxos;
use lighting_utxo_anchor_manager::selection::{
    consolidation_candidates, detect_fragmentation, select_utxos_for_channel,
};
use lighting_utxo_anchor_manager::simulation::simulate_fee_levels;

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

    #[arg(long)]
    json: bool,
}

#[derive(Serialize)]
struct WalletReport {
    total_balance: u64,
    spendable_balance: u64,
    usable_balance: u64,
    anchor_balance: u64,
    max_channel_size: u64,
    risk_score: u8,
}

fn main() {
    let args = Args::parse();
    let utxos = match fetch_utxos() {
        Ok(u) => u,
        Err(e) => {
            println!("RPC error: {}", e);
            return;
        }
    };

    let balance = total_balance(&utxos);
    let spendable = spendable_utxos(&utxos, 3);
    let spendable_balance: u64 = spendable.iter().map(|u| u.value).sum();
    let usable = available_after_reserve(&utxos, 3, args.reserve);
    let anchor_utxos = anchor_capable_utxos(&utxos, 3, 40_000);
    let anchor_balance: u64 = anchor_utxos.iter().map(|u| u.value).sum();
    let max_channel = max_safe_channel_size(&utxos, 3, args.reserve, args.channel_buffer);
    let fee_capacity = fee_bump_capacity(&anchor_utxos, args.feerate, args.tx_size);
    let risk = fee_risk_status(&anchor_utxos, args.feerate, args.tx_size);
    let risk_score = lightning_risk_score(&utxos, 40_000);

    if args.json {
        let report = WalletReport {
            total_balance: balance,
            spendable_balance,
            usable_balance: usable,
            anchor_balance,
            max_channel_size: max_channel,
            risk_score,
        };
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        return;
    }

    println!("\n⚡ Lightning Wallet Health Dashboard");
    println!("-----------------------------------");
    render_health_bar("Liquidity Health", 85); 
    render_health_bar("Anchor Safety", anchor_fee_safety_score(&utxos, args.feerate, args.tx_size));
    render_health_bar("Operational Risk", risk_score);

    println!("\nFragmentation Analysis");
    println!("----------------------");
    if detect_fragmentation(&utxos) {
        println!("⚠ Wallet fragmentation detected");
        let candidates = consolidation_candidates(&utxos);
        for u in candidates { println!("{} sats", u.value); }
    } else {
        println!("Wallet UTXO distribution looks healthy");
    }

    println!("\nChannel Funding Recommendation");
    println!("------------------------------");
    let recommended = select_utxos_for_channel(&utxos, max_channel);
    if recommended.is_empty() {
        println!("No suitable UTXOs available.");
    } else {
        for u in recommended { println!("{} sats", u.value); }
    }

    println!("\nStrategy Suggestion");
    println!("-------------------");
    match suggest_channel_strategy(&utxos, args.reserve) {
        ChannelStrategy::SingleLargeChannel(val) => println!("Strategy: OPEN SINGLE LARGE CHANNEL ({} sats)", val),
        ChannelStrategy::MultipleChannels(vec) => println!("Strategy: OPEN MULTIPLE SMALL CHANNELS ({} total channels)", vec.len()),
        ChannelStrategy::InsufficientLiquidity => println!("Strategy: CONSOLIDATE OR DEPOSIT FUNDS"),
    }

    println!("\n⚡ Lightning Node UTXO Health Report");
    println!("------------------------------------");
    println!("Total balance: {} sats", balance);
    println!("Spendable balance (>=3 conf): {} sats", spendable_balance);
    println!("Usable balance (after reserve): {} sats", usable);
    println!("Anchor-capable balance: {} sats", anchor_balance);
    println!("Max safe channel size: {} sats", max_channel);
    println!("\nFee bump simulation ({} sat/vB):", args.feerate);
    println!("Remaining after fee bump: {} sats", fee_capacity);
    println!("Risk status: {}", risk);

    simulate_fee_levels(&utxos);
}