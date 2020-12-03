pub(super) fn run() -> Result<(), super::Error> {
	let trees = Trees::new(super::read_input_lines::<String>("day3")?)?;

	let down_1_right_3 = trees.count_along_diagonal(1, 3);

	{
		let result = down_1_right_3;

		println!("3a: {}", result);

		assert_eq!(result, 187);
	}

	{
		let down_1_right_1 = trees.count_along_diagonal(1, 1);
		let down_1_right_5 = trees.count_along_diagonal(1, 5);
		let down_1_right_7 = trees.count_along_diagonal(1, 7);
		let down_2_right_1 = trees.count_along_diagonal(2, 1);

		let result = down_1_right_1 * down_1_right_3 * down_1_right_5 * down_1_right_7 * down_2_right_1;

		println!("3b: {}", result);

		assert_eq!(result, 4723283400);
	}

	Ok(())
}

#[derive(Debug)]
struct Trees {
	map: std::collections::BTreeSet<(usize, usize)>,
	num_rows: usize,
	num_cols: usize,
}

impl Trees {
	fn new(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Self, super::Error> {
		let mut map: std::collections::BTreeSet<(usize, usize)> = Default::default();
		let mut num_rows = 0;
		let mut num_cols = 0;

		for (row, line) in input.enumerate() {
			let line = line?;
			let line = line.as_ref();

			if row == 0 {
				num_cols = line.len();
			}

			for (col, c) in line.chars().enumerate() {
				if c == '#' {
					map.insert((row, col));
				}
			}

			num_rows += 1;
		}

		Ok(Trees {
			map,
			num_rows,
			num_cols,
		})
	}

	fn count_along_diagonal(&self, down: usize, right: usize) -> usize {
		(0..(self.num_rows / down))
			.filter(|&row| self.map.contains(&(row * down, (row * right) % self.num_cols)))
			.count()
	}
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

	#[test]
	fn count_along_diagonal() {
		let trees = super::Trees::new(INPUT.lines().map(Ok)).unwrap();

		let down_1_right_3 = trees.count_along_diagonal(1, 3);
		assert_eq!(down_1_right_3, 7);

		let down_1_right_1 = trees.count_along_diagonal(1, 1);
		assert_eq!(down_1_right_1, 2);

		let down_1_right_5 = trees.count_along_diagonal(1, 5);
		assert_eq!(down_1_right_5, 3);

		let down_1_right_7 = trees.count_along_diagonal(1, 7);
		assert_eq!(down_1_right_7, 4);

		let down_2_right_1 = trees.count_along_diagonal(2, 1);
		assert_eq!(down_2_right_1, 2);

		let result = down_1_right_1 * down_1_right_3 * down_1_right_5 * down_1_right_7 * down_2_right_1;
		assert_eq!(result, 2 * 7 * 3 * 4 * 2);
	}
}
