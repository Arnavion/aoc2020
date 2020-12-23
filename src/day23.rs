pub(super) fn run() -> Result<(), super::Error> {
	let cups = parse(super::read_input_lines::<String>("day23")?)?;

	{
		let result = part1(&cups)?;

		println!("23a: {}", result);

		assert_eq!(result, "52864379");
	}

	{
		let result = part2(&cups)?;

		println!("23b: {}", result);

		assert_eq!(result, 11591415792);
	}

	Ok(())
}

fn parse(mut input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Vec<usize>, super::Error> {
	let line = input.next().ok_or("EOF")?;
	let line = line?;
	let line = line.as_ref();

	let cups: Vec<_> =
		line.chars()
		.map(|c| c as usize - '0' as usize)
		.collect();

	{
		let mut cups = cups.clone();
		cups.sort_unstable();
		if cups != [1, 2, 3, 4, 5, 6, 7, 8, 9] {
			return Err("expected input to contain the cups 1..=9".into());
		}
	}

	Ok(cups)
}

fn part1(cups: &[usize]) -> Result<String, super::Error> {
	let right_neighbors = play(cups, 100)?;

	let mut result = String::new();
	let mut cup = right_neighbors[1];
	while cup != 1 {
		use std::fmt::Write;
		write!(result, "{}", cup)?;
		cup = right_neighbors[cup];
	}

	Ok(result)
}

fn part2(cups: &[usize]) -> Result<usize, super::Error> {
	let cups: Vec<_> =
		cups.iter()
		.copied()
		.chain((cups.len() + 1)..=1_000_000)
		.collect();

	let right_neighbors = play(&cups, 10_000_000)?;

	let cup_one = right_neighbors[1];
	let cup_two = right_neighbors[cup_one];
	Ok(cup_one * cup_two)
}

/// Returns the right neighbors map after playing `num_moves` moves. The map is a Vec where the index is cup
/// and the element is the cup to its right.
///
/// Eg, given cups = "389125467", the initial map is "_258647391". Index 0 in the map is unused and should be ignored,
/// and the last cup's right neighbor is the first cup.
///
/// To convert the map back to the list, start at cup "1", then its right neighbor is `right_neighbors[1]`,
/// its right neighbor is `right_neighbors[right_neighbors[1]]`, and so on.
fn play(cups: &[usize], num_moves: usize) -> Result<Vec<usize>, super::Error> {
	let mut right_neighbors = vec![0; cups.len() + 1];
	for (&cup, &cup_right_neighbor) in cups.iter().zip(cups[1..].iter().chain(std::iter::once(&cups[0]))) {
		right_neighbors[cup] = cup_right_neighbor;
	}

	let mut current_cup = cups[0];

	for _ in 0..num_moves {
		let removed_cup_one = right_neighbors[current_cup];
		let removed_cup_two = right_neighbors[removed_cup_one];
		let removed_cup_three = right_neighbors[removed_cup_two];

		right_neighbors[current_cup] = right_neighbors[removed_cup_three];

		let destination_cup =
			(1..current_cup).rev()
			.chain(((current_cup + 1)..=(cups.len())).rev())
			.find(|&destination_cup| destination_cup != removed_cup_one && destination_cup != removed_cup_two && destination_cup != removed_cup_three)
			.ok_or("no solution")?;

		right_neighbors[removed_cup_three] = right_neighbors[destination_cup];
		right_neighbors[destination_cup] = removed_cup_one;

		current_cup = right_neighbors[current_cup];
	}

	Ok(right_neighbors)
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "389125467";

	#[test]
	fn part1() {
		let cups = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&cups).unwrap(), "67384529");
	}

	#[test]
	fn part2() {
		let cups = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&cups).unwrap(), 934001 * 159792);
	}
}
