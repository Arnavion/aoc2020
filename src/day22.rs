pub(super) fn run() -> Result<(), super::Error> {
	let (player1, player2) = parse(super::read_input_lines::<String>("day22")?)?;

	{
		let (mut player1, mut player2) = (player1.clone(), player2.clone());
		let result = part1(&mut player1, &mut player2);

		println!("22a: {result}");

		assert_eq!(result, 33680);
	}

	{
		let (mut player1, mut player2) = (player1, player2);
		let result = part2(&mut player1, &mut player2);

		println!("22b: {result}");

		assert_eq!(result, 33683);
	}

	Ok(())
}

fn parse(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) ->
	Result<(std::collections::VecDeque<usize>, std::collections::VecDeque<usize>), super::Error>
{
	let mut player1: std::collections::VecDeque<_> = Default::default();
	let mut player2: std::collections::VecDeque<_> = Default::default();

	let mut player = &mut player1;

	for line in input {
		let line = line?;
		let line = line.as_ref();

		match line {
			"" | "Player 1:" => (),
			"Player 2:" => player = &mut player2,
			line => {
				let card = line.parse().map_err(|err| format!("invalid card {line:?}: {err}"))?;
				player.push_back(card);
			},
		}
	}

	Ok((player1, player2))
}

fn part1(player1: &mut std::collections::VecDeque<usize>, player2: &mut std::collections::VecDeque<usize>) -> usize {
	play_game(player1, player2, false)
}

fn part2(player1: &mut std::collections::VecDeque<usize>, player2: &mut std::collections::VecDeque<usize>) -> usize {
	play_game(player1, player2, true)
}

fn play_game(player1: &mut std::collections::VecDeque<usize>, player2: &mut std::collections::VecDeque<usize>, recursive: bool) -> usize {
	fn player1_wins_game(player1: &mut std::collections::VecDeque<usize>, player2: &mut std::collections::VecDeque<usize>, recursive: bool) -> bool {
		let mut round_history: std::collections::BTreeSet<(std::collections::VecDeque<usize>, std::collections::VecDeque<usize>)> = Default::default();

		loop {
			if recursive && !round_history.insert((player1.clone(), player2.clone())) {
				break true;
			}

			let (card1, card2) = match (player1.pop_front(), player2.pop_front()) {
				(Some(card1), Some(card2)) => (card1, card2),
				(Some(card1), None) => { player1.push_front(card1); break true; },
				(None, Some(card2)) => { player2.push_front(card2); break false; },
				(None, None) => unreachable!(),
			};

			let player1_won_round = {
				if recursive && card1 <= player1.len() && card2 <= player2.len() {
					let mut player1 = player1.iter().copied().take(card1).collect();
					let mut player2 = player2.iter().copied().take(card2).collect();
					player1_wins_game(&mut player1, &mut player2, recursive)
				}
				else {
					card1 > card2
				}
			};

			if player1_won_round {
				player1.push_back(card1);
				player1.push_back(card2);
			}
			else {
				player2.push_back(card2);
				player2.push_back(card1);
			}
		}
	}

	let game_winner = if player1_wins_game(player1, player2, recursive) { player1 } else { player2 };
	let score = game_winner.iter().rev().enumerate().map(|(i, &card)| (i + 1) * card).sum();
	score
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

	#[test]
	fn part1() {
		let (mut player1, mut player2) = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&mut player1, &mut player2), 3 * 10 + 2 * 9 + 10 * 8 + 6 * 7 + 8 * 6 + 5 * 5 + 9 * 4 + 4 * 3 + 7 * 2 + 1 * 1);
	}

	#[test]
	fn part2() {
		let (mut player1, mut player2) = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&mut player1, &mut player2), 291);
	}
}
