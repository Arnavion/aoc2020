// Ref: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
//
//           (  q, r - 1  )    (q + 1, r - 1)
//
//  (  q - 1, r  )    (    q, r    )    (q + 1, r    )
//
//           (q - 1, r + 1)    (  q, r + 1  )

pub(super) fn run() -> Result<(), super::Error> {
	let mut black_tiles = parse(super::read_input_lines::<String>("day24")?)?;

	{
		let result = part1(&black_tiles);

		println!("24a: {result}");

		assert_eq!(result, 322);
	}

	{
		let result = part2(&mut black_tiles);

		println!("24b: {result}");

		assert_eq!(result, 3831);
	}

	Ok(())
}

fn parse(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<std::collections::BTreeSet<(i8, i8)>, super::Error> {
	#[derive(Clone, Copy, Debug)]
	enum Direction {
		East,
		NorthEast,
		NorthWest,
		SouthEast,
		SouthWest,
		West,
	}

	let mut black_tiles: std::collections::BTreeSet<(i8, i8)> = Default::default();

	for line in input {
		let line = line?;
		let mut line = line.as_ref();

		let mut pos = (0, 0);

		while !line.is_empty() {
			let (direction, rest) =
				line.strip_prefix('e').map(|rest| (Direction::East, rest))
				.or_else(|| line.strip_prefix("ne").map(|rest| (Direction::NorthEast, rest)))
				.or_else(|| line.strip_prefix("nw").map(|rest| (Direction::NorthWest, rest)))
				.or_else(|| line.strip_prefix("se").map(|rest| (Direction::SouthEast, rest)))
				.or_else(|| line.strip_prefix("sw").map(|rest| (Direction::SouthWest, rest)))
				.or_else(|| line.strip_prefix('w').map(|rest| (Direction::West, rest)))
				.ok_or_else(|| format!("invalid direction in line {line:?}"))?;

			match direction {
				Direction::East => {
					pos.0 += 1;
				},

				Direction::NorthEast => {
					pos.0 += 1;
					pos.1 -= 1;
				},

				Direction::NorthWest => {
					pos.1 -= 1;
				},

				Direction::SouthEast => {
					pos.1 += 1;
				},

				Direction::SouthWest => {
					pos.0 -= 1;
					pos.1 += 1;
				},

				Direction::West => {
					pos.0 -= 1;
				},
			}

			line = rest;
		}

		if black_tiles.contains(&pos) {
			black_tiles.remove(&pos);
		}
		else {
			black_tiles.insert(pos);
		}
	}

	Ok(black_tiles)
}

fn part1(black_tiles: &std::collections::BTreeSet<(i8, i8)>) -> usize {
	black_tiles.len()
}

fn part2(black_tiles: &mut std::collections::BTreeSet<(i8, i8)>) -> usize {
	let (min_q, max_q, min_r, max_r) =
		black_tiles.iter()
		.fold(
			(i8::max_value(), i8::min_value(), i8::max_value(), i8::min_value()),
			|(min_q, max_q, min_r, max_r), &(q, r)|
				(std::cmp::min(min_q, q), std::cmp::max(max_q, q), std::cmp::min(min_r, r), std::cmp::max(max_r, r)),
		);

	let mut make_white = vec![];
	let mut make_black = vec![];

	for i in 0..100 {
		for q in (min_q - i - 1)..=(max_q + i + 1) {
			for r in (min_r - i - 1)..=(max_r + i + 1) {
				let is_black = black_tiles.contains(&(q, r));
				let num_black_neighbors =
					[(q - 1, r), (q - 1, r + 1), (q, r - 1), (q, r + 1), (q + 1, r - 1), (q + 1, r)].iter()
					.filter(|pos| black_tiles.contains(pos))
					.count();
				match (is_black, num_black_neighbors) {
					(true, num_black_neighbors) if num_black_neighbors == 0 || num_black_neighbors > 2 => make_white.push((q, r)),
					(false, 2) => make_black.push((q, r)),
					_ => (),
				}
			}
		}

		for pos in make_white.drain(..) {
			black_tiles.remove(&pos);
		}

		for pos in make_black.drain(..) {
			black_tiles.insert(pos);
		}
	}

	black_tiles.len()
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew\
";

	#[test]
	fn part1() {
		let black_tiles = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&black_tiles), 10);
	}

	#[test]
	fn part2() {
		let mut black_tiles = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&mut black_tiles), 2208);
	}
}
