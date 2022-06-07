pub(super) fn run() -> Result<(), super::Error> {
	let mut plane = [[false; 8]; 128];

	for line in super::read_input_lines::<String>("day5")? {
		let line = line?;

		let (row_num, seat_num) = find_seat(&line)?;
		plane[row_num][seat_num] = true;
	}

	{
		let result =
			plane.iter()
			.enumerate()
			.flat_map(|(row_num, row)| {
				row.iter()
				.enumerate()
				.filter_map(move |(seat_num, &occupied)| occupied.then(|| seat_id(row_num, seat_num)))
			})
			.max()
			.ok_or("no solution")?;

		println!("5a: {result}");

		assert_eq!(result, 919);
	}

	{
		let result =
			plane.iter()
			.enumerate()
			.find_map(|(row_num, row)|
				if row.iter().copied().any(std::convert::identity) {
					row.iter()
						.copied()
						.position(std::ops::Not::not)
						.map(|seat_num| seat_id(row_num, seat_num))
				}
				else {
					None
				})
			.ok_or("no solution")?;

		println!("5b: {result}");

		assert_eq!(result, 642);
	}

	Ok(())
}

fn find_seat(pass: &str) -> Result<(usize, usize), super::Error> {
	let seat_id: Result<usize, super::Error> =
		pass.chars().try_fold(0, |row_num, c| match c {
			'F' | 'L' => Ok(row_num * 2),
			'B' | 'R' => Ok(row_num * 2 + 1),
			_ => Err(format!("malformed pass {pass:?}").into()),
		});
	let seat_id = seat_id?;

	let row_num = seat_id / 8;
	let seat_num = seat_id % 8;
	Ok((row_num, seat_num))
}

fn seat_id(row_num: usize, seat_num: usize) -> usize {
	row_num * 8 + seat_num
}

#[cfg(test)]
mod tests {
	#[test]
	fn find_seat_and_id() {
		fn find_seat_and_id(pass: &str) -> (usize, usize, usize) {
			let (row_num, seat_num) = super::find_seat(pass).unwrap();
			let seat_id = super::seat_id(row_num, seat_num);
			(row_num, seat_num, seat_id)
		}

		assert_eq!(find_seat_and_id("FBFBBFFRLR"), (44, 5, 357));
		assert_eq!(find_seat_and_id("BFFFBBFRRR"), (70, 7, 567));
		assert_eq!(find_seat_and_id("FFFBBBFRRR"), (14, 7, 119));
		assert_eq!(find_seat_and_id("BBFFBBFRLL"), (102, 4, 820));
	}
}
