pub(super) fn run() -> Result<(), super::Error> {
	let actions = Action::parse(super::read_input_lines::<Action>("day12")?)?;

	{
		let result = part1(&actions);

		println!("12a: {result}");

		assert_eq!(result, 1152);
	}

	{
		let result = part2(&actions);

		println!("12b: {result}");

		assert_eq!(result, 58637);
	}

	Ok(())
}

type Vector = num_complex::Complex<i64>;

const EAST: Vector = Vector::new(1, 0);
const NORTH: Vector = Vector::new(0, 1);
const SOUTH: Vector = Vector::new(0, -1);
const WEST: Vector = Vector::new(-1, 0);

const ROTATE_LEFT: Vector = Vector::new(0, 1);
const ROTATE_RIGHT: Vector = Vector::new(0, -1);

#[derive(Clone, Copy, Debug)]
enum Action {
	Forward(i64),
	Rotate(Vector),
	Translate(Vector),
}

impl Action {
	fn parse(input: impl Iterator<Item = Result<Self, super::Error>>) -> Result<Vec<Self>, super::Error> {
		let actions: Result<Vec<_>, super::Error> = input.collect();
		let actions = actions?;
		Ok(actions)
	}
}

impl std::str::FromStr for Action {
	type Err = super::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let amount: i64 = s[1..].parse().map_err(|err| format!("invalid action {s:?}: {err}"))?;

		Ok(match &s[..1] {
			"E" => Action::Translate(EAST * amount),
			"F" => Action::Forward(amount),
			"L" => Action::Rotate(num_traits::Pow::pow(&ROTATE_LEFT, amount / 90)),
			"N" => Action::Translate(NORTH * amount),
			"R" => Action::Rotate(num_traits::Pow::pow(&ROTATE_RIGHT, amount / 90)),
			"S" => Action::Translate(SOUTH * amount),
			"W" => Action::Translate(WEST * amount),
			_ => return Err(format!("invalid action {s:?}").into()),
		})
	}
}

#[derive(Clone, Copy, Debug)]
struct Ship {
	pos: Vector,
	waypoint: Vector,
	translate_waypoint: bool,
}

impl Ship {
	fn update(&mut self, action: Action) {
		match action {
			Action::Forward(amount) => self.pos += self.waypoint * amount,
			Action::Rotate(vector) => self.waypoint *= vector,
			Action::Translate(vector) => *(if self.translate_waypoint { &mut self.waypoint } else { &mut self.pos }) += vector,
		}
	}
}

fn part1(actions: &[Action]) -> i64 {
	let mut ship = Ship {
		pos: Default::default(),
		waypoint: EAST,
		translate_waypoint: false,
	};

	for &action in actions {
		ship.update(action);
	}

	ship.pos.l1_norm()
}

fn part2(actions: &[Action]) -> i64 {
	let mut ship = Ship {
		pos: Default::default(),
		waypoint: Vector::new(10, 1),
		translate_waypoint: true,
	};

	for &action in actions {
		ship.update(action);
	}

	ship.pos.l1_norm()
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
F10
N3
F7
R90
F11\
";

	#[test]
	fn part1() {
		let actions = super::Action::parse(INPUT.split('\n').map(|line| Ok(line.parse()?))).unwrap();
		assert_eq!(super::part1(&actions), 25);
	}

	#[test]
	fn part2() {
		let actions = super::Action::parse(INPUT.split('\n').map(|line| Ok(line.parse()?))).unwrap();
		assert_eq!(super::part2(&actions), 286);
	}
}
