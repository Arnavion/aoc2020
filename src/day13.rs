pub(super) fn run() -> Result<(), super::Error> {
	let (earliest_departure_timestamp, bus_ids) = parse_input(super::read_input_lines::<String>("day13")?)?;

	{
		let result = part1(earliest_departure_timestamp, &bus_ids)?;

		println!("13a: {}", result);

		assert_eq!(result, 153);
	}

	#[cfg(not(test))]
	{
		let result = part2(&bus_ids);

		println!("13b: {}", result);

		assert_eq!(
			result,
			"https://www.wolframalpha.com/input/?i=ChineseRemainder%28%7b0%2c+38%2c+984%2c+2%2c+6%2c+16%2c+575%2c+24%2c+7%7d%2c+%7b13%2c+41%2c+997%2c+23%2c+19%2c+29%2c+619%2c+37%2c+17%7d%29",
		);

		let result = 471793476184394;

		assert!(check_part2(result, &bus_ids));
	}

	Ok(())
}

fn parse_input(mut input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<(usize, Vec<Option<usize>>), super::Error> {
	let earliest_departure_timestamp: usize = {
		let line = input.next().ok_or("no earliest departure timestamp")??;
		let line = line.as_ref();
		line.parse().map_err(|err| format!("could not parse earliest departure timestamp: {}", err))?
	};

	let bus_ids = {
		let line = input.next().ok_or("no earliest departure timestamp")??;
		let line = line.as_ref();
		let bus_ids: Result<_, super::Error> =
			line.split(',')
			.map(|id|
				if id == "x" {
					Ok(None)
				} else {
					let id = id.parse().map_err(|err| format!("invalid bus ID {:?}: {}", id, err))?;
					Ok(Some(id))
				})
			.collect();
		bus_ids?
	};

	Ok((earliest_departure_timestamp, bus_ids))
}

fn part1(earliest_departure_timestamp: usize, bus_ids: &[Option<usize>]) -> Result<usize, super::Error> {
	let (next_bus_id, next_bus_departure) =
		bus_ids.iter()
		.filter_map(|&bus_id| {
			let bus_id = bus_id?;
			Some((bus_id, (earliest_departure_timestamp + bus_id - 1) / bus_id * bus_id))
		})
		.min_by_key(|&(_, next_bus_departure)| next_bus_departure)
		.ok_or("no solution")?;

	Ok(next_bus_id * (next_bus_departure - earliest_departure_timestamp))
}

#[cfg(test)]
fn part2(bus_ids: &[Option<usize>]) -> Result<usize, super::Error> {
	let max_bus_id_parameters =
		bus_ids.iter()
		.enumerate()
		.filter_map(|(time_offset, &bus_id)| bus_id.map(move |bus_id| (time_offset, bus_id)))
		.max_by_key(|&(_, bus_id)| bus_id);
	let (time_offset_of_max_bus_id, max_bus_id) =
		if let Some(max_bus_id_parameters) = max_bus_id_parameters {
			max_bus_id_parameters
		}
		else {
			// All buses are "x", so time 0 is valid.
			return Ok(0);
		};

	let mut times =
		(0..)
		.filter_map(|time| (time * max_bus_id).checked_sub(time_offset_of_max_bus_id));

	Ok(times.find(|&time| check_part2(time, bus_ids)).ok_or("no solution")?)
}

#[cfg(not(test))]
fn part2(bus_ids: &[Option<usize>]) -> String {
	// Dammit, I signed up for Advent of Code, not Project Euler.

	fn join(nums: &[usize]) -> String {
		let mut result = String::new();

		for num in nums {
			if !result.is_empty() {
				result.push_str("%2c+");
			}
			result.push_str(&num.to_string());
		}

		result
	}

	// Find time such that:
	//    (time + time_offset) % bus_id = 0
	// => (time + time_offset % bus_id) % bus_id = 0    (since time_offset may be greater than bus_id)
	// => time % bus_id = (bus_id - (time_offset % bus_id)) % bus_id
	let (divisors, remainders): (Vec<_>, Vec<_>) =
		bus_ids.iter()
		.enumerate()
		.filter_map(|(time_offset, &bus_id)| bus_id.map(|bus_id| (bus_id, ((bus_id - time_offset % bus_id) % bus_id))))
		.unzip();

	format!(
		"https://www.wolframalpha.com/input/?i=ChineseRemainder%28%7b{}%7d%2c+%7b{}%7d%29",
		join(&remainders),
		join(&divisors),
	)
}

fn check_part2(time: usize, bus_ids: &[Option<usize>]) -> bool {
	bus_ids.iter()
	.enumerate()
	.all(|(i, &bus_id)| bus_id.map_or(true, |bus_id| (time + i) % bus_id == 0))
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
939
7,13,x,x,59,x,31,19
";

	#[test]
	fn part1() {
		let (earliest_departure_timestamp, bus_ids) = super::parse_input(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(earliest_departure_timestamp, &bus_ids).unwrap(), 295);
	}

	#[test]
	fn part2() {
		let (_, bus_ids) = super::parse_input(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&bus_ids).unwrap(), 1068781);
	}
}
