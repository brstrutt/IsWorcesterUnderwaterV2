

pub fn get_flood_percentage(current_level: f64, normal_level: f64, risky_level: f64) -> f64 {
	let quarter = risky_level - normal_level; // Calculate what 25% of the range is
	let base = normal_level - quarter; // Calculate what value 0% would be, aka the base of the range

	let scaled_level = current_level - base;
	let percentage_multiplier = 1.0 / (4.0 * quarter);
	let percentage = scaled_level * percentage_multiplier;
	percentage.min(1.0).max(0.0)
}
