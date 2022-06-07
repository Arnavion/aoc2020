pub(super) fn run() -> Result<(), super::Error> {
	let mut input = Input::parse(super::read_input_lines::<String>("day16")?)?;

	{
		let result = part1(&input);

		println!("16a: {result}");

		assert_eq!(result, 23115);
	}

	{
		let result = part2(&mut input)?;

		println!("16b: {result}");

		assert_eq!(result, 239727793813);
	}

	Ok(())
}

#[derive(Debug)]
struct Input {
	fields: Vec<(String, std::ops::RangeInclusive<u64>, std::ops::RangeInclusive<u64>)>,
	ticket: Vec<u64>,
	nearby_tickets: Vec<Vec<u64>>,
}

impl Input {
	fn parse(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Self, super::Error> {
		enum State {
			Fields,
			TicketPre,
			Ticket,
			TicketPost,
			NearbyTicketsPre,
			NearbyTickets,
		}

		static FIELDS_REGEX: once_cell::sync::Lazy<regex::Regex> =
			once_cell::sync::Lazy::new(||
				regex::Regex::new(r"^(?P<name>[^:]+): (?P<range1_low>\d+)-(?P<range1_high>\d+) or (?P<range2_low>\d+)-(?P<range2_high>\d+)$")
				.expect("hard-coded regex must compile successfully"));

		let mut result = Input {
			fields: vec![],
			ticket: vec![],
			nearby_tickets: vec![],
		};

		let mut state = State::Fields;

		for line in input {
			let line = line?;
			let line = line.as_ref();

			match state {
				State::Fields if line.is_empty() => { state = State::TicketPre; },
				State::Fields => {
					let captures = FIELDS_REGEX.captures(line).ok_or_else(|| format!("input fields line {line:?} has invalid format"))?;
					let name = &captures["name"];
					let range1_low = captures["range1_low"].parse().map_err(|err| format!("input fields line {line:?} has invalid format: {err}"))?;
					let range1_high = captures["range1_high"].parse().map_err(|err| format!("input fields line {line:?} has invalid format: {err}"))?;
					let range2_low = captures["range2_low"].parse().map_err(|err| format!("input fields line {line:?} has invalid format: {err}"))?;
					let range2_high = captures["range2_high"].parse().map_err(|err| format!("input fields line {line:?} has invalid format: {err}"))?;
					result.fields.push((name.to_owned(), range1_low..=range1_high, range2_low..=range2_high));
				},

				State::TicketPre if line == "your ticket:" => { state = State::Ticket; },
				State::TicketPre => return Err(r#"malformed input: expected "your ticket:""#.into()),

				State::Ticket => {
					for part in line.split(',') {
						result.ticket.push(part.parse().map_err(|err| format!("input fields line {line:?} has invalid format: {err}"))?);
					}
					state = State::TicketPost;
				},

				State::TicketPost if line.is_empty() => { state = State::NearbyTicketsPre; },
				State::TicketPost => return Err(r#"malformed input: expected empty line"#.into()),

				State::NearbyTicketsPre if line == "nearby tickets:" => { state = State::NearbyTickets; },
				State::NearbyTicketsPre => return Err(r#"malformed input: expected "nearby tickets:""#.into()),

				State::NearbyTickets if line.is_empty() => { break; },
				State::NearbyTickets => {
					let mut ticket = vec![];
					for part in line.split(',') {
						ticket.push(part.parse().map_err(|err| format!("input fields line {line:?} has invalid format: {err}"))?);
					}
					result.nearby_tickets.push(ticket);
				}
			}
		}

		Ok(result)
	}

	fn trim_invalid_nearby_tickets(&mut self) {
		self.nearby_tickets =
			std::mem::take(&mut self.nearby_tickets)
			.into_iter()
			.filter(|ticket| invalid_fields(ticket, &self.fields).next().is_none())
			.collect();
	}

	fn discover_fields(&self) -> Result<impl Iterator<Item = (&str, u64)> + '_, super::Error> {
		let num_fields = self.fields.len();

		// hypotheses[row, col] = "is ticket[row] specified by fields[col] ?"
		let mut hypotheses = vec![true; num_fields * num_fields];

		for (col, (_, range1, range2)) in self.fields.iter().enumerate() {
			for ticket in &self.nearby_tickets {
				for (row, field) in ticket.iter().enumerate() {
					if !range1.contains(field) && !range2.contains(field) {
						hypotheses[row * num_fields + col] = false;
					}
				}
			}
		}

		// Simplify by finding rows that only have one `true` col and setting that col to `false` in all other rows.
		// Once every row has only one `true` col, that is the mapping.
		//
		// Luckily this puzzle does not require more complex Sudoku-style solving, like discovering N rows with N `true` cols between them
		// to be able to set those N cols for all other rows to `false`.

		// During simplication, the mappings are stored with an offset of +1, so that 0 refers to a mapping that has not yet been discovered.
		// The store could've been a Vec<Option<usize>> instead, but that would have doubled the space requirement.
		let mut mappings = vec![0; num_fields];

		let mut num_mappings_found = 0;

		while num_mappings_found < num_fields {
			let mut new_mappings = vec![];

			for ((row, cols), &existing_mapping) in hypotheses.chunks(num_fields).enumerate().zip(&mappings) {
				if existing_mapping > 0 {
					// Already found this mapping earlier.
					continue;
				}

				let mut cols = cols.iter();
				let true_col = cols.position(|col| *col).ok_or("no solution")?;
				if cols.any(|col| *col) {
					// There are at least two `true`s in this row, so try another row.
					continue;
				}

				new_mappings.push((row, true_col));
			}

			if num_mappings_found < num_fields && new_mappings.is_empty() {
				return Err("no unique solution".into());
			}

			for (row, col) in new_mappings {
				// Set this col in all other rows to `false`
				for row in (0..num_fields).filter(|&row_| row_ != row) {
					hypotheses[row * num_fields + col] = false;
				}

				mappings[row] = col + 1;
				num_mappings_found += 1;
			}
		}

		let result =
			self.ticket.iter()
			.zip(mappings)
			.map(move |(&field, field_num)| {
				let (field_name, _, _) = &self.fields[field_num - 1];
				(&**field_name, field)
			});
		Ok(result)
	}
}

fn invalid_fields<'a>(
	ticket: &'a [u64],
	fields: &'a [(String, std::ops::RangeInclusive<u64>, std::ops::RangeInclusive<u64>)],
) -> impl Iterator<Item = u64> + 'a {
	ticket.iter()
		.filter(move |value| !fields.iter().any(|(_, range1, range2)| range1.contains(value) || range2.contains(value)))
		.copied()
}

fn part1(input: &Input) -> u64 {
	input.nearby_tickets.iter()
		.flat_map(|ticket| invalid_fields(ticket, &input.fields))
		.sum()
}

fn part2(input: &mut Input) -> Result<u64, super::Error> {
	input.trim_invalid_nearby_tickets();

	let result =
		input.discover_fields()?
		.filter_map(|(field_name, field)| field_name.starts_with("departure ").then(|| field))
		.product();
	Ok(result)
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		const INPUT: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

		let input = super::Input::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&input), 4 + 55 + 12);
	}

	#[test]
	fn discover_fields() {
		const INPUT: &str = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

		let input = super::Input::parse(INPUT.split('\n').map(Ok)).unwrap();
		let ticket: std::collections::BTreeMap<_, _> = input.discover_fields().unwrap().collect();
		assert_eq!(ticket, [("row", 11), ("class", 12), ("seat", 13)].iter().copied().collect());
	}
}
