/// Render a simple ASCII health bar
pub fn health_bar(score: u8) -> String {

    let total_slots = 10;
    let filled = (score as usize * total_slots) / 100;
    let empty = total_slots - filled;

    let filled_bar = "█".repeat(filled);
    let empty_bar = "░".repeat(empty);

    format!("{}{} {}%", filled_bar, empty_bar, score)
}