pub(super) fn run() -> Result<(), super::Error> {
	let (card_public_key, door_public_key) = parse(super::read_input_lines::<String>("day25")?)?;

	{
		let result = part1(card_public_key, door_public_key);

		println!("25a: {}", result);

		assert_eq!(result, 15467093);
	}

	Ok(())
}

fn parse(mut input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<(u64, u64), super::Error> {
	let line = input.next().ok_or("EOF")?;
	let line = line?;
	let line = line.as_ref();
	let card_public_key = line.parse().map_err(|err| format!("could not parse card public key from {:?}: {}", line, err))?;

	let line = input.next().ok_or("EOF")?;
	let line = line?;
	let line = line.as_ref();
	let door_public_key = line.parse().map_err(|err| format!("could not parse door public key from {:?}: {}", line, err))?;

	Ok((card_public_key, door_public_key))
}

struct Dhm {
	subject: u64,
	value: u64,
}

impl Iterator for Dhm {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		let result = self.value;
		self.value = (self.value * self.subject) % 20201227;
		Some(result)
	}
}

fn part1(card_public_key: u64, door_public_key: u64) -> u64 {
	let card_public_key_iterator = Dhm { subject: 7, value: 1 };
	let encryption_key_iterator = Dhm { subject: door_public_key, value: 1 };

	let (_, encryption_key) =
		card_public_key_iterator.zip(encryption_key_iterator)
		.find(|&(value1, _)| value1 == card_public_key)
		.expect("infinite iterator must eventually find solution");

	#[cfg(test)]
	{
		let door_public_key_iterator = Dhm { subject: 7, value: 1 };
		let encryption_key_iterator = Dhm { subject: card_public_key, value: 1 };

		let (_, encryption_key2) =
			door_public_key_iterator.zip(encryption_key_iterator)
			.find(|&(value1, _)| value1 == door_public_key)
			.expect("infinite iterator must eventually find solution");
		assert_eq!(encryption_key, encryption_key2);
	}

	encryption_key
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
5764801
17807724
";

	#[test]
	fn part1() {
		let (card_public_key, door_public_key) = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(card_public_key, door_public_key), 14897079);
	}
}
