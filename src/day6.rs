pub(super) fn run() -> Result<(), super::Error> {
	{
		let groups = part1(super::read_input_lines::<String>("day6")?)?;

		let result: usize = groups.iter().map(std::collections::BTreeSet::len).sum();

		println!("6a: {result}");

		assert_eq!(result, 6310);
	}

	{
		let groups = part2(super::read_input_lines::<String>("day6")?)?;

		let result: usize = groups.iter().map(std::collections::BTreeSet::len).sum();

		println!("6b: {result}");

		assert_eq!(result, 3193);
	}

	Ok(())
}

fn part1(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Vec<std::collections::BTreeSet<char>>, super::Error> {
	parse_answers(input, |group, questions| questions.for_each(|question| {
		group.insert(question);
	}))
}

fn part2(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Vec<std::collections::BTreeSet<char>>, super::Error> {
	parse_answers(input, |group, questions| {
		let original_group = std::mem::take(group);
		*group = original_group.intersection(&questions.collect()).copied().collect();
	})
}

fn parse_answers(
	input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>,
	mut merge: impl FnMut(&mut std::collections::BTreeSet<char>, std::str::Chars<'_>),
) -> Result<Vec<std::collections::BTreeSet<char>>, super::Error> {
	let mut groups = vec![];

	let mut group: Option<std::collections::BTreeSet<_>> = None;
	for line in input {
		let line = line?;
		let line = line.as_ref();

		if line.is_empty() {
			groups.push(group.unwrap());
			group = None;
		}
		else if let Some(group) = &mut group {
			merge(group, line.chars());
		}
		else {
			group = Some(line.chars().collect());
		}
	}
	if let Some(group) = group {
		groups.push(group);
	}

	Ok(groups)
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";

	#[test]
	fn part1() {
		let groups = super::part1(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(&groups, &[
			['a', 'b', 'c'].iter().copied().collect(),
			['a', 'b', 'c'].iter().copied().collect(),
			['a', 'b', 'c'].iter().copied().collect(),
			std::iter::once('a').collect(),
			std::iter::once('b').collect(),
		]);

		let result: usize = groups.iter().map(std::collections::BTreeSet::len).sum();
		assert_eq!(result, 11);
	}

	#[test]
	fn part2() {
		let groups = super::part2(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(&groups, &[
			['a', 'b', 'c'].iter().copied().collect(),
			Default::default(),
			['a'].iter().copied().collect(),
			std::iter::once('a').collect(),
			std::iter::once('b').collect(),
		]);

		let result: usize = groups.iter().map(std::collections::BTreeSet::len).sum();
		assert_eq!(result, 6);
	}
}
