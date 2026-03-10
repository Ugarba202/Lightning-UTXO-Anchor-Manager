use crate::utxo::Utxo;

/// Wallet liquidity state classification
#[derive(Debug, PartialEq)]
pub enum LiquidityStatus {
    Healthy,
    LowAnchorLiquidity,
    Fragmented,
    Unsafe,
}

/// Evaluate wallet liquidity health
pub fn classify_wallet_liquidity(utxos: &[Utxo], anchor_threshold: u64) -> LiquidityStatus {
    if utxos.is_empty() {
        return LiquidityStatus::Unsafe;
    }

    let total: u64 = utxos.iter().map(|u| u.value).sum();
    let anchor_count = utxos.iter().filter(|u| u.value >= anchor_threshold).count();

    if anchor_count == 0 {
        return LiquidityStatus::LowAnchorLiquidity;
    }

    let small_utxos = utxos.iter().filter(|u| u.value < 20_000).count();
    if small_utxos >= 3 {
        return LiquidityStatus::Fragmented;
    }

    if total < 50_000 {
        return LiquidityStatus::Unsafe;
    }

    LiquidityStatus::Healthy
}

/// Anchor fee safety score (0-100)
pub fn anchor_fee_safety_score(utxos: &[Utxo], feerate: u64, tx_size: u64) -> u8 {
    let total: u64 = utxos.iter().map(|u| u.value).sum();
    let required_fee = feerate * tx_size;

    if total >= required_fee * 5 {
        return 100;
    }
    if total >= required_fee * 3 {
        return 70;
    }
    if total >= required_fee {
        return 40;
    }
    10
}

/// Simple ASCII bar renderer for CLI dashboard
pub fn render_health_bar(label: &str, score: u8) {
    let filled = (score / 10) as usize;
    let empty = 10 - filled;
    
    let bar = format!(
        "{: <20} {}{} {}%",
        label,
        "█".repeat(filled),
        "░".repeat(empty),
        score
    );
    println!("{}", bar);
}

/// Compute an overall Lightning node operational risk score (0-100)
pub fn lightning_risk_score(utxos: &[Utxo], anchor_threshold: u64) -> u8 {
    let total_balance: u64 = utxos.iter().map(|u| u.value).sum();
    let anchor_count = utxos.iter().filter(|u| u.value >= anchor_threshold).count();
    let small_utxos = utxos.iter().filter(|u| u.value < 20_000).count();

    let mut score = 100;
    if total_balance < 50_000 { score -= 40; }
    if anchor_count == 0 { score -= 30; }
    if small_utxos >= 3 { score -= 20; }
    
    score.max(10)
}

#[derive(Debug, PartialEq)]
pub enum ChannelStrategy {
    SingleLargeChannel(u64),
    MultipleChannels(Vec<u64>),
    InsufficientLiquidity,
}

/// Suggest a channel funding strategy based on wallet liquidity
pub fn suggest_channel_strategy(utxos: &[Utxo], reserve: u64) -> ChannelStrategy {
    let total: u64 = utxos.iter().map(|u| u.value).sum();

    if total <= reserve {
        return ChannelStrategy::InsufficientLiquidity;
    }

    let available = total - reserve;
    if available >= 150_000 {
        return ChannelStrategy::SingleLargeChannel(available);
    }

    let mut channels = Vec::new();
    let mut remaining = available;
    while remaining >= 40_000 {
        channels.push(40_000);
        remaining -= 40_000;
    }
    
    if channels.is_empty() {
        ChannelStrategy::InsufficientLiquidity
    } else {
        ChannelStrategy::MultipleChannels(channels)
    }
}
