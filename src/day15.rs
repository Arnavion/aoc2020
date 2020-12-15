pub(super) fn run() -> Result<(), super::Error> {
	let mut game = Game::import(super::read_input_lines::<String>("day15")?)?;

	{
		game.autoplay_to(2020);
		let result = game.next_number;

		println!("15a: {}", result);

		assert_eq!(result, 1009);
	}

	{
		game.autoplay_to(30_000_000);
		let result = game.next_number;

		println!("15b: {}", result);

		assert_eq!(result, 62714);
	}

	Ok(())
}

#[derive(Clone, Debug)]
struct Game {
	// This is a "map" where the number is the index and the element is the turn number.
	//
	// `last_seen_turn: std::collections::BTreeMap<usize, usize>` is more space-efficient but takes longer;
	// 15b takes 3.6s with a BTreeMap and only 0.6s with a Vec, albeit the Vec is ~300 MiB and only ~4M of the ~30M elements are used.
	//
	// The Vec can also be pre-allocated for the largest possible number that might be reached in a given number of turns,
	// as `autoplay_to` below does, so that re-allocations don't occur while autoplaying. BTreeMap doesn't support this.
	last_seen_turn: Vec<usize>,

	turn_number: usize,

	next_number: usize,
}

impl Game {
	fn import(mut input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Self, super::Error> {
		let mut game = Game {
			last_seen_turn: vec![],
			turn_number: 1,
			next_number: 0,
		};

		let line = input.next().ok_or("EOF")??;
		let line = line.as_ref();

		for s in line.split(',') {
			let number = s.parse()?;
			game.play(Some(number));
		}

		Ok(game)
	}

	fn play(&mut self, number: Option<usize>) {
		let number = number.unwrap_or(self.next_number);

		if self.last_seen_turn.len() < number + 1 {
			self.last_seen_turn.resize(number + 1, 0);
		}

		self.next_number = match std::mem::replace(&mut self.last_seen_turn[number], self.turn_number) {
			0 => 0,
			last_seen_turn => self.turn_number - last_seen_turn,
		};

		self.turn_number += 1;
	}

	fn autoplay_to(&mut self, turn_number: usize) {
		// Pre-allocate for the worst-case of reaching the largest number possible, ie turn_number - 2
		if self.last_seen_turn.len() < turn_number - 2 {
			self.last_seen_turn.resize(turn_number - 2, 0);
		}

		while self.turn_number < turn_number {
			self.play(None);
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		let mut game = super::Game::import(std::iter::once(Ok("0,3,6"))).unwrap();

		assert_eq!(game.next_number, 0);
		game.play(None);
		assert_eq!(game.next_number, 3);
		game.play(None);
		assert_eq!(game.next_number, 3);
		game.play(None);
		assert_eq!(game.next_number, 1);
		game.play(None);
		assert_eq!(game.next_number, 0);
		game.play(None);
		assert_eq!(game.next_number, 4);
		game.play(None);
		assert_eq!(game.next_number, 0);
	}

	macro_rules! part2 {
		( $($name:ident : $input:literal => $expected:literal ,)* ) => {
			$(
				#[test]
				fn $name() {
					let mut game = super::Game::import(std::iter::once(Ok($input))).unwrap();
					game.autoplay_to(30_000_000);
					let actual = game.next_number;
					assert_eq!(actual, $expected);
				}
			)*
		};
	}

	part2! {
		part2_1 : "0,3,6" => 175594,
		part2_2 : "1,3,2" => 2578,
		part2_3 : "2,1,3" => 3544142,
		part2_4 : "1,2,3" => 261214,
		part2_5 : "2,3,1" => 6895259,
		part2_6 : "3,2,1" => 18,
		part2_7 : "3,1,2" => 362,
	}
}
