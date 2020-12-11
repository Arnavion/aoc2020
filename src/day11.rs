pub(super) fn run() -> Result<(), super::Error> {
	let (seats, num_rows, num_cols) = parse_seats(super::read_input_lines::<String>("day11")?)?;

	{
		let result = part1(seats.clone(), num_rows, num_cols);

		println!("11a: {}", result);

		assert_eq!(result, 2361);
	}

	{
		let result = part2(seats, num_rows, num_cols);

		println!("11b: {}", result);

		assert_eq!(result, 2119);
	}

	Ok(())
}

fn parse_seats(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) ->
	Result<(std::collections::BTreeMap<(usize, usize), Seat>, usize, usize), super::Error>
{
	let mut seats: std::collections::BTreeMap<(usize, usize), Seat> = Default::default();

	for (row, line) in input.enumerate() {
		let line = line?;
		let line = line.as_ref();

		for (col, c) in line.chars().enumerate() {
			match c {
				'L' => { seats.insert((row, col), Seat::Empty); },
				'#' => { seats.insert((row, col), Seat::Occupied); },
				'.' => (),
				c => return Err(format!("unexpected character {:?}", c).into()),
			}
		}
	}

	let (num_rows, num_cols) =
		seats.keys()
		.fold((0, 0), |(num_rows, num_cols), &(row, col)| (std::cmp::max(num_rows, row + 1), std::cmp::max(num_cols, col + 1)));

	Ok((seats, num_rows, num_cols))
}

fn part1(
	seats: std::collections::BTreeMap<(usize, usize), Seat>,
	num_rows: usize, num_cols: usize,
) -> usize {
	solve(seats, num_rows, num_cols, 1, 4)
}

fn part2(
	seats: std::collections::BTreeMap<(usize, usize), Seat>,
	num_rows: usize, num_cols: usize,
) -> usize {
	solve(seats, num_rows, num_cols, usize::max_value(), 5)
}

fn solve(
	mut seats: std::collections::BTreeMap<(usize, usize), Seat>,
	num_rows: usize, num_cols: usize,
	check_distance: usize,
	min_num_occupied_neighbors_to_become_empty: usize,
) -> usize {
	let mut make_occupied = vec![];
	let mut make_empty = vec![];

	loop {
		for (&(row, col), &seat) in &seats {
			let up = (0..row).rev().take(check_distance);
			let down = ((row + 1)..num_rows).take(check_distance);
			let left = (0..col).rev().take(check_distance);
			let right = ((col + 1)..num_cols).take(check_distance);

			let position_iterators: &mut [&mut dyn Iterator<Item = (usize, usize)>] = &mut [
				&mut up.clone().map(|row| (row, col)),
				&mut down.clone().map(|row| (row, col)),
				&mut left.clone().map(|col| (row, col)),
				&mut right.clone().map(|col| (row, col)),
				&mut up.clone().zip(left.clone()),
				&mut up.zip(right.clone()),
				&mut down.clone().zip(left),
				&mut down.zip(right),
			];

			let num_occupied_neighbors: usize =
				position_iterators.iter_mut()
				.filter_map(|positions| positions.find_map(|position| seats.get(&position)))
				.filter(|&&seat| seat == Seat::Occupied)
				.count();

			match (seat, num_occupied_neighbors) {
				(Seat::Empty, 0) =>
					make_occupied.push((row, col)),

				(Seat::Occupied, num_occupied_neighbors) if num_occupied_neighbors >= min_num_occupied_neighbors_to_become_empty =>
					make_empty.push((row, col)),

				_ => (),
			}
		}

		if make_occupied.is_empty() && make_empty.is_empty() {
			break;
		}

		for (row, col) in make_occupied.drain(..) {
			seats.insert((row, col), Seat::Occupied);
		}

		for (row, col) in make_empty.drain(..) {
			seats.insert((row, col), Seat::Empty);
		}
	}

	seats.values().filter(|&&seat| seat == Seat::Occupied).count()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Seat {
	Empty,
	Occupied,
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

	#[test]
	fn part1() {
		let (seats, num_rows, num_cols) = super::parse_seats(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(seats, num_rows, num_cols), 37);
	}

	#[test]
	fn part2() {
		let (seats, num_rows, num_cols) = super::parse_seats(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(seats, num_rows, num_cols), 26);
	}
}
