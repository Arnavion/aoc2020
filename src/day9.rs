pub(super) fn run() -> Result<(), super::Error> {
	let part1_result = {
		let result = part1(super::read_input_lines("day9")?, 25)?;

		println!("9a: {result}");

		assert_eq!(result, 530627549);

		result
	};

	{
		let result = part2(super::read_input_lines("day9")?, part1_result)?;

		println!("9b: {result}");

		assert_eq!(result, 77730285);
	}

	Ok(())
}

fn part1(input: impl Iterator<Item = Result<u64, super::Error>>, num_summands: usize) -> Result<u64, super::Error> {
	let mut nums = std::collections::VecDeque::with_capacity(num_summands);

	for num in input {
		let num = num?;

		if nums.len() == num_summands {
			let is_valid =
				nums.iter().copied().rev().take(num_summands).enumerate().any(|(i, num1)|
					nums.iter().copied().rev().take(num_summands).enumerate().any(|(j, num2)|
						i != j && num1 + num2 == num));
			if !is_valid {
				return Ok(num);
			}

			let _ = nums.pop_front();
		}

		nums.push_back(num);
	}

	Err("no solution".into())
}

fn part2(input: impl Iterator<Item = Result<u64, super::Error>>, expected_sum: u64) -> Result<u64, super::Error> {
	let mut range: std::collections::VecDeque<_> = Default::default();
	let mut sum = 0;

	for num in input {
		let num = num?;

		range.push_back(num);
		sum += num;

		while sum > expected_sum {
			if let Some(first_num) = range.pop_front() {
				sum -= first_num;
			}
		}

		if sum == expected_sum {
			let (min, max) =
				range.iter()
				.copied()
				.fold(None, |min_max, num| match (min_max, num) {
					(Some((min, max)), num) => Some((std::cmp::min(min, num), std::cmp::max(max, num))),
					(None, num) => Some((num, num)),
				})
				.ok_or("no solution")?;
			return Ok(min + max);
		}
	}

	Err("no solution".into())
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576\
";

	#[test]
	fn part1() {
		assert_eq!(super::part1(INPUT.split('\n').map(|line| Ok(line.parse()?)), 5).unwrap(), 127);
	}

	#[test]
	fn part2() {
		assert_eq!(super::part2(INPUT.split('\n').map(|line| Ok(line.parse()?)), 127).unwrap(), 62);
	}
}
