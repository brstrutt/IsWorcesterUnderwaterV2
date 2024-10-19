
#[derive(PartialEq, Clone, Copy)]
pub struct HeightRange {
    pub normal_level: f64,
    pub risky_level: f64
}

// Converts the specified river level into a screen height percentage
// HeightRange.normal_level will sit 25% above the bottom of the screen
// HeightRange.risky_level will sit 50% above the bottom of the screen
pub fn get_flood_percentage(current_level: f64, range: HeightRange) -> f64 {
	let quarter = range.risky_level - range.normal_level; // Calculate what 25% of the range is
	let base = range.normal_level - quarter; // Calculate what value 0% would be, aka the base of the range

	let scaled_level = current_level - base;
	let percentage_multiplier = 1.0 / (4.0 * quarter);
	let percentage = scaled_level * percentage_multiplier;
	percentage
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_risky_level() {
		let ranges = [
			HeightRange{normal_level: 1.3, risky_level: 4.6},
			HeightRange{normal_level: 0.0, risky_level: 103.5},
			HeightRange{normal_level: 12.0, risky_level: 53.5},
			HeightRange{normal_level: 0.0, risky_level: 0.1}
		];

		for range in ranges {
			let result = get_flood_percentage(range.risky_level, range);
			assert_eq!(result, 0.5);
		};
	}

	#[test]
	fn test_normal_level() {
		let ranges = [
			HeightRange{normal_level: 1.3, risky_level: 4.6},
			HeightRange{normal_level: 0.0, risky_level: 103.5},
			HeightRange{normal_level: 12.0, risky_level: 53.5},
			HeightRange{normal_level: 0.0, risky_level: 0.1}
		];

		for range in ranges {
			let result = get_flood_percentage(range.normal_level, range);
			assert_eq!(result, 0.25);
		};
	}

	#[test]
	fn test_values_above_risky_level() {
		assert_eq!(get_flood_percentage(
			3.0, 
			HeightRange{normal_level: 1.0, risky_level: 2.0}), 
			0.75
		);
		assert_eq!(get_flood_percentage(
			26.0, 
			HeightRange{normal_level: 20.0, risky_level: 25.0}), 
			0.55
		);
		assert_eq!(get_flood_percentage(
			30.0, 
			HeightRange{normal_level: 10.0, risky_level: 15.0}), 
			1.25
		);
	}

	#[test]
	fn test_values_below_normal_level() {
		assert_eq!(get_flood_percentage(
			0.0, 
			HeightRange{normal_level: 1.0, risky_level: 2.0}), 
			0.0
		);
		assert_eq!(get_flood_percentage(
			19.0, 
			HeightRange{normal_level: 20.0, risky_level: 25.0}), 
			0.2
		);
		assert_eq!(get_flood_percentage(
			0.0, 
			HeightRange{normal_level: 10.0, risky_level: 15.0}), 
			-0.25
		);
	}
}
