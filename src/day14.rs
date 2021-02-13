pub(super) fn run() -> Result<(), super::Error> {
	{
		let result = solve::<Mask1, _, _>(super::read_input_lines::<String>("day14")?)?;

		println!("14a: {}", result);

		assert_eq!(result, 6317049172545);
	}

	{
		let result = solve::<Mask2, _, _>(super::read_input_lines::<String>("day14")?)?;

		println!("14b: {}", result);

		assert_eq!(result, 3434009980379);
	}

	Ok(())
}

#[derive(Clone, Copy, Debug)]
enum MaskBit {
	Zero,
	One,
	X,
}

impl std::convert::TryFrom<char> for MaskBit {
	type Error = String;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		Ok(match c {
			'0' => MaskBit::Zero,
			'1' => MaskBit::One,
			'X' => MaskBit::X,
			c => return Err(format!("invalid mask bit {:?}", c)),
		})
	}
}

trait Mask: for<'a> From<&'a [MaskBit; 36]> {
	fn set(&self, memory: &mut std::collections::BTreeMap<u64, u64>, address: u64, value: u64);
}

#[derive(Debug)]
struct Mask1 {
	and_mask: u64,
	or_mask: u64,
}

impl From<&'_ [MaskBit; 36]> for Mask1 {
	fn from(bits: &'_ [MaskBit; 36]) -> Self {
		let mut result = Mask1 {
			and_mask: 0x0000_000F_FFFF_FFFF_u64,
			or_mask: 0x0000_0000_0000_0000_u64,
		};

		for (i, &bit) in bits.iter().enumerate() {
			match bit {
				MaskBit::Zero => set_bit(&mut result.and_mask, i, false),
				MaskBit::One => set_bit(&mut result.or_mask, i, true),
				MaskBit::X => (),
			}
		}

		result
	}
}

impl Mask for Mask1 {
	fn set(&self, memory: &mut std::collections::BTreeMap<u64, u64>, address: u64, value: u64) {
		let value = (value & self.and_mask) | self.or_mask;
		memory.insert(address, value);
	}
}

#[derive(Debug)]
struct Mask2 {
	or_mask: u64,
	floating_bits: [bool; 36],
}

impl From<&'_ [MaskBit; 36]> for Mask2 {
	fn from(bits: &'_ [MaskBit; 36]) -> Self {
		let mut result = Mask2 {
			or_mask: 0x0000_0000_0000_0000,
			floating_bits: [false; 36],
		};

		for (i, &bit) in bits.iter().enumerate() {
			match bit {
				MaskBit::Zero => (),
				MaskBit::One => set_bit(&mut result.or_mask, i, true),
				MaskBit::X => { result.floating_bits[i] = true; },
			}
		}

		result
	}
}

impl Mask for Mask2 {
	fn set(&self, memory: &mut std::collections::BTreeMap<u64, u64>, address: u64, value: u64) {
		let address = address | self.or_mask;

		let mut addresses =
			std::collections::VecDeque::with_capacity(
				2_usize.pow(std::convert::TryInto::try_into(
					self.floating_bits.iter().filter(|&&bit| bit).count(),
				).expect("<= 36 is infallibly convertible to usize")),
			);
		addresses.push_back(address);

		for pos in self.floating_bits.iter().enumerate().filter_map(|(i, &bit)| bit.then(|| i)) {
			for i in 0..(addresses.len()) {
				let mut address = addresses[i];

				set_bit(&mut address, pos, true);
				addresses[i] = address;

				set_bit(&mut address, pos, false);
				addresses.push_back(address);
			}
		}

		for address in addresses {
			memory.insert(address, value);
		}
	}
}

static MASK_REGEX: once_cell::sync::Lazy<regex::Regex> =
	once_cell::sync::Lazy::new(||
		regex::Regex::new(r"^mask = (?P<mask>[01X]{36})$")
		.expect("hard-coded regex must compile successfully"));

static MEM_REGEX: once_cell::sync::Lazy<regex::Regex> =
	once_cell::sync::Lazy::new(||
		regex::Regex::new(r"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$")
		.expect("hard-coded regex must compile successfully"));

fn solve<M, I, S>(input: I) -> Result<u64, super::Error>
where
	M: Mask,
	I: Iterator<Item = Result<S, super::Error>>,
	S: AsRef<str>,
{
	let mut memory: std::collections::BTreeMap<u64, u64> = Default::default();

	let mut mask_bits = [MaskBit::Zero; 36];
	let mut mask: M = (&mask_bits).into();

	for line in input {
		let line = line?;
		let line = line.as_ref();

		if let Some(captures) = MASK_REGEX.captures(&line) {
			for (i, c) in captures["mask"].chars().enumerate() {
				mask_bits[i] =
					std::convert::TryInto::try_into(c)
					.map_err(|err| format!("input line {:?} has invalid format: {}", line, err))?;
			}

			mask = (&mask_bits).into();
		}
		else if let Some(captures) = MEM_REGEX.captures(&line) {
			let address = captures["address"].parse().map_err(|err| format!("input line {:?} has invalid format: {}", line, err))?;
			let value = captures["value"].parse().map_err(|err| format!("input line {:?} has invalid format: {}", line, err))?;
			mask.set(&mut memory, address, value);
		}
		else {
			return Err(format!("input line {:?} has invalid format", line).into());
		}
	}

	Ok(memory.values().sum())
}

fn set_bit(num: &mut u64, pos: usize, value: bool) {
	if value {
		*num |= 0x0000_0008_0000_0000_u64 >> pos;
	}
	else {
		*num &= !(0x0000_0008_0000_0000_u64 >> pos);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		const INPUT: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0\
";

		let result = super::solve::<super::Mask1, _, _>(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(result, 165);
	}

	#[test]
	fn part2() {
		const INPUT: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1\
";

		let result = super::solve::<super::Mask2, _, _>(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(result, 208);
	}
}
