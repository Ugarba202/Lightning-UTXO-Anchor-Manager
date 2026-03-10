use clap::Parser;
use lighting_utxo_anchor_manager::anchor::{anchor_capable_utxos, max_safe_channel_size};
use lighting_utxo_anchor_manager::policy::{
    ChannelStrategy, anchor_fee_safety_score, classify_wallet_liquidity, lightning_risk_score,
    suggest_channel_strategy,
};
use lighting_utxo_anchor_manager::rpc::fetch_utxos;
use lighting_utxo_anchor_manager::selection::{
    consolidation_candidates, detect_fragmentation, select_utxos_for_channel,
};

use lighting_utxo_anchor_manager::simulation::simulate_fee_levels;

use lighting_utxo_anchor_manager::reserve::{
    available_after_reserve, fee_bump_capacity, fee_risk_status, spendable_utxos, total_balance,
};
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
    println!("\nFragmentation Analysis");
    println!("----------------------");

    if detect_fragmentation(&utxos) {
        println!("⚠ Wallet fragmentation detected");

        let candidates = consolidation_candidates(&utxos);

        println!("Small UTXOs recommended for consolidation:");

        for u in candidates {
            println!("{} sats", u.value);
        }
    } else {
        println!("Wallet UTXO distribution looks healthy");
    }
    println!("\nChannel Funding Recommendation");
    println!("------------------------------");

    let recommended = select_utxos_for_channel(&utxos, max_channel);

    if recommended.is_empty() {
        println!("No suitable UTXOs available for channel funding.");
    } else {
        println!("Suggested UTXOs for channel funding:");

        for u in recommended {
            println!("{} sats", u.value);
        }
    }
    println!("\nLightning Wallet Policy Analysis");
    println!("--------------------------------");

    println!("\nChannel Funding Strategy");
    println!("------------------------");

    let strategy = suggest_channel_strategy(&utxos, args.reserve);

    match strategy {
        ChannelStrategy::SingleLargeChannel(size) => {
            println!("Recommended strategy: Single large channel");
            println!("Suggested channel size: {} sats", size);
        }

        ChannelStrategy::MultipleChannels(channels) => {
            println!("Recommended strategy: Multiple smaller channels");

            for (i, size) in channels.iter().enumerate() {
                println!("Channel {} → {} sats", i + 1, size);
            }
        }

        ChannelStrategy::InsufficientLiquidity => {
            println!("Wallet liquidity insufficient for safe channel funding");
        }
    }

    let liquidity = classify_wallet_liquidity(&utxos, 40_000);

    match liquidity {
        lighting_utxo_anchor_manager::policy::LiquidityStatus::Healthy => {
            println!("Wallet liquidity: HEALTHY")
        }

        lighting_utxo_anchor_manager::policy::LiquidityStatus::LowAnchorLiquidity => {
            println!("Wallet liquidity: LOW ANCHOR LIQUIDITY")
        }

        lighting_utxo_anchor_manager::policy::LiquidityStatus::Fragmented => {
            println!("Wallet liquidity: FRAGMENTED")
        }

        lighting_utxo_anchor_manager::policy::LiquidityStatus::Unsafe => {
            println!("Wallet liquidity: UNSAFE")
        }
    }
    println!("\nLightning Node Risk Score");
    println!("-------------------------");

    let risk_score = lightning_risk_score(&utxos, 40_000);

    println!("Operational Safety Score: {}/100", risk_score);
    let score = anchor_fee_safety_score(&utxos, args.feerate, args.tx_size);

    println!("Anchor Fee Safety Score: {}/100", score);

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
    simulate_fee_levels(&utxos);
}
