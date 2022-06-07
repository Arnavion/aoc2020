pub(super) fn run() -> Result<(), super::Error> {
	let instructions = parse_program(super::read_input_lines("day8")?)?;

	{
		let result = part1(&instructions)?;

		println!("8a: {result}");

		assert_eq!(result, 1818);
	}

	{
		let result = part2(&instructions)?;

		println!("8b: {result}");

		assert_eq!(result, 631);
	}

	Ok(())
}

fn parse_program(input: impl Iterator<Item = Result<Instruction, super::Error>>) -> Result<Vec<Instruction>, super::Error> {
	let mut instructions = vec![];

	for instruction in input {
		let instruction = instruction?;
		instructions.push(instruction);
	}

	Ok(instructions)
}

fn part1(instructions: &[Instruction]) -> Result<i64, super::Error> {
	match boot(instructions)? {
		BootResult::InfiniteLoop(acc) => Ok(acc),
		BootResult::Finished(_) => Err("expected program to infinite loop but it finished".into()),
	}
}

fn part2(instructions: &[Instruction]) -> Result<i64, super::Error> {
	for (i, _) in instructions.iter().enumerate().filter(|(_, instruction)| matches!(instruction, Instruction::Jmp(_))) {
		let mut instructions = instructions.to_owned();
		instructions[i] = Instruction::Nop(0);
		if let BootResult::Finished(acc) = boot(&instructions)? {
			return Ok(acc);
		}
	}

	Err("no solution".into())
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
	Jmp(i64),
	Acc(i64),
	Nop(i64),
}

impl std::str::FromStr for Instruction {
	type Err = super::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let arg =
			s.get(4..)
			.ok_or_else(|| format!("invalid instruction {s:?}: no arg"))?
			.parse()
			.map_err(|err| format!("invalid instruction {s:?}: {err}"))?;

		let instruction = match &s[..3] {
			"jmp" => Instruction::Jmp(arg),
			"acc" => Instruction::Acc(arg),
			"nop" => Instruction::Nop(arg),
			_ => return Err(format!("invalid instruction {s:?}: unrecognized opcode").into()),
		};
		Ok(instruction)
	}
}

fn boot(instructions: &[Instruction]) -> Result<BootResult, super::Error> {
	let mut executed: std::collections::BTreeSet<_> = Default::default();

	let mut pc = 0;
	let mut acc = 0;

	loop {
		if !executed.insert(pc) {
			return Ok(BootResult::InfiniteLoop(acc));
		}

		let instruction = {
			let pc: usize = std::convert::TryInto::try_into(pc).map_err(|err| format!("pc out of range: {err}"))?;
			instructions.get(pc).copied()
		};
		match instruction {
			Some(Instruction::Jmp(arg)) => {
				pc += arg;
				continue;
			},

			Some(Instruction::Acc(arg)) => acc += arg,

			Some(Instruction::Nop(_)) => (),

			None => return Ok(BootResult::Finished(acc)),
		}

		pc += 1;
	}
}

#[derive(Clone, Copy, Debug)]
enum BootResult {
	InfiniteLoop(i64),
	Finished(i64),
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6\
";

	#[test]
	fn part1() {
		let instructions = super::parse_program(INPUT.split('\n').map(|line| Ok(line.parse()?))).unwrap();

		assert_eq!(super::part1(&instructions).unwrap(), 5);
	}

	#[test]
	fn part2() {
		let instructions = super::parse_program(INPUT.split('\n').map(|line| Ok(line.parse()?))).unwrap();

		assert_eq!(super::part2(&instructions).unwrap(), 8);
	}
}
