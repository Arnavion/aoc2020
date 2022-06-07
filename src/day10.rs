pub(super) fn run() -> Result<(), super::Error> {
	let adapters = parse_adapters(super::read_input_lines::<u64>("day10")?)?;

	{
		let result = part1(&adapters)?;

		println!("10a: {result}");

		assert_eq!(result, 2310);
	}

	{
		let result = part2(&adapters);

		println!("10b: {result}");

		assert_eq!(result, 64793042714624);
	}

	Ok(())
}

fn parse_adapters(input: impl Iterator<Item = Result<u64, super::Error>>) -> Result<Vec<u64>, super::Error> {
	let adapters: Result<Vec<_>, _> = std::iter::once(Ok(0)).chain(input).collect();
	let mut adapters = adapters?;

	adapters.sort_unstable();

	let device = adapters.last().ok_or("no adapters found in input")? + 3;
	adapters.push(device);

	Ok(adapters)
}

fn part1(adapters: &[u64]) -> Result<usize, super::Error> {
	let (num_one_diffs, num_three_diffs) =
		adapters.iter()
		.zip(adapters.iter().skip(1))
		.try_fold((0, 0), |(num_one_diffs, num_three_diffs), (&a, &b)| match b - a {
			0 | 2 => Ok((num_one_diffs, num_three_diffs)),
			1 => Ok((num_one_diffs + 1, num_three_diffs)),
			3 => Ok((num_one_diffs, num_three_diffs + 1)),
			4..=u64::MAX => Err("no solution"),
		})?;
	Ok(num_one_diffs * num_three_diffs)
}

fn part2(adapters: &[u64]) -> u64 {
	let mut num_paths = (0, 0, 1);

	let a = std::iter::repeat(0).take(2).chain(adapters.iter().copied());
	let b = std::iter::repeat(0).take(1).chain(adapters.iter().copied());
	let c = adapters.iter().copied();
	let d = adapters.iter().copied().skip(1);

	for (((a, b), c), d) in a.zip(b).zip(c).zip(d) {
		num_paths = (
			num_paths.1,
			num_paths.2,
			if d - a <= 3 { num_paths.0 } else { 0 } +
			if d - b <= 3 { num_paths.1 } else { 0 } +
			if d - c <= 3 { num_paths.2 } else { 0 },
		);
	}

	num_paths.2
}

#[cfg(test)]
mod tests {
	const INPUT1: &str = "\
16
10
15
5
1
11
7
19
6
12
4\
";

	const INPUT2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3\
";

	#[test]
	fn part1() {
		for &(input, expected) in &[
			(INPUT1, 35),
			(INPUT2, 220),
		] {
			let adapters = super::parse_adapters(input.split('\n').map(|line| Ok(line.parse()?))).unwrap();
			assert_eq!(super::part1(&adapters).unwrap(), expected);
		}
	}

	#[test]
	fn part2() {
		for &(input, expected) in &[
			(INPUT1, 8),
			(INPUT2, 19208),
		] {
			let adapters = super::parse_adapters(input.split('\n').map(|line| Ok(line.parse()?))).unwrap();
			assert_eq!(super::part2(&adapters), expected);
		}
	}
}
