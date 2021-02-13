pub(super) fn run() -> Result<(), super::Error> {
	let input = Input::parse(super::read_input_lines::<String>("day20")?)?;

	{
		let result = part1(&input);

		println!("20a: {}", result);

		assert_eq!(result, 1699 * 2137 * 2539 * 2549);
	}

	{
		let result = part2(&input)?;

		println!("20b: {}", result);

		assert_eq!(result, 2256);
	}

	Ok(())
}

struct Input {
	tiles: std::collections::BTreeMap<u64, [[bool; 10]; 10]>,

	// Map value is [right neighbor, down neighbor]
	neighbors: std::collections::BTreeMap<(u64, Op), [Option<(u64, Op)>; 2]>,

	corners: [u64; 4],
}

impl Input {
	fn parse(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Self, super::Error> {
		#[derive(Clone, Copy, Debug)]
		enum Direction {
			Right,
			Down,
		}

		fn can_be_neighbors(tile1: &[[bool; 10]; 10], tile2: &[[bool; 10]; 10], direction: Direction) -> bool {
			match direction {
				Direction::Right => (0..10).all(|i| tile1[i][9] == tile2[i][0]),
				Direction::Down => tile1[9] == tile2[0],
			}
		}

		let mut tiles: std::collections::BTreeMap<u64, [[bool; 10]; 10]> = Default::default();

		let mut id_tile_row: Option<(u64, [[bool; 10]; 10], usize)> = None;

		for line in input {
			let line = line?;
			let line = line.as_ref();

			if line.is_empty() {
				if let Some((id, tile, _)) = id_tile_row.take() {
					tiles.insert(id, tile);
				}
			}
			else if let Some(suffix) = line.strip_prefix("Tile ") {
				let num = suffix.strip_suffix(":").ok_or_else(|| format!("malformed input: line {:?} does not match tile ID line pattern", line))?;
				let id = num.parse().map_err(|err| format!("expected tile number but got {:?}: {}", num, err))?;
				id_tile_row = Some((id, [[false; 10]; 10], 0));
			}
			else {
				let (_, tile, row) = id_tile_row.as_mut().ok_or("malformed input: line tile definition without tile ID line")?;
				for (col, c) in line.chars().enumerate() {
					tile[*row][col] = c == '#';
				}

				*row += 1;
			}
		}

		let mut neighbors: std::collections::BTreeMap<(u64, Op), [Option<(u64, Op)>; 2]> = Default::default();

		for (&id1, tile1) in &tiles {
			for &op1 in ALL_OPS {
				let tile1 = {
					let mut new_tile1 = [[false; 10]; 10];
					op1.transform(tile1, &mut new_tile1);
					new_tile1
				};

				for (&id2, tile2) in &tiles {
					if id1 == id2 {
						continue;
					}

					for &op2 in ALL_OPS {
						let tile2 = {
							let mut new_tile2 = [[false; 10]; 10];
							op2.transform(tile2, &mut new_tile2);
							new_tile2
						};

						if can_be_neighbors(&tile1, &tile2, Direction::Right) {
							neighbors.entry((id1, op1)).or_default()[0] = Some((id2, op2));
						}
						else if can_be_neighbors(&tile1, &tile2, Direction::Down) {
							neighbors.entry((id1, op1)).or_default()[1] = Some((id2, op2));
						}
					}
				}
			}
		}

		let mut neighbor_ids: std::collections::BTreeMap<u64, std::collections::BTreeSet<u64>> = Default::default();
		for (&(id1, _), &[horizontal, vertical]) in &neighbors {
			for &(id2, _) in horizontal.iter().chain(vertical.iter()) {
				neighbor_ids.entry(id1).or_default().insert(id2);
			}
		}

		let corners: Vec<_> =
			neighbor_ids.iter()
			.filter_map(|(&id, neighbor_ids)| (neighbor_ids.len() == 2).then(|| id))
			.collect();
		if corners.len() != 4 {
			return Err(format!("expected four corners but found {:?}", corners).into());
		}

		let corners = [corners[0], corners[1], corners[2], corners[3]];

		Ok(Input {
			tiles,
			neighbors,
			corners,
		})
	}
}

fn part1(input: &Input) -> u64 {
	input.corners.iter().product()
}

fn part2(Input { tiles, neighbors, corners }: &Input) -> Result<usize, super::Error> {
	fn is_sea_monster(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
		fn safe_get(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
			grid.get(row).and_then(|row| row.get(col)).copied().unwrap_or_default()
		}

		// (0, 0) ..................#.
		//        #....##....##....###
		//        .#..#..#..#..#..#... (2, 19)

		safe_get(grid, row, col + 18) &&
		safe_get(grid, row + 1, col) &&
		safe_get(grid, row + 1, col + 5) &&
		safe_get(grid, row + 1, col + 6) &&
		safe_get(grid, row + 1, col + 11) &&
		safe_get(grid, row + 1, col + 12) &&
		safe_get(grid, row + 1, col + 17) &&
		safe_get(grid, row + 1, col + 18) &&
		safe_get(grid, row + 1, col + 19) &&
		safe_get(grid, row + 2, col + 1) &&
		safe_get(grid, row + 2, col + 4) &&
		safe_get(grid, row + 2, col + 7) &&
		safe_get(grid, row + 2, col + 10) &&
		safe_get(grid, row + 2, col + 13) &&
		safe_get(grid, row + 2, col + 16)
	}

	#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
	let num_tiles_in_grid_side = (tiles.len() as f32).sqrt() as usize;

	let id_start = corners[0];

	let &op_start =
		ALL_OPS.iter()
		.find(|&&op| matches!(neighbors.get(&(id_start, op)), Some([Some(_), Some(_)])))
		.ok_or_else(|| format!("could not find neighbors of corner {}", id_start))?;

	let mut grid = Vec::with_capacity(tiles.len());

	for row in 0..num_tiles_in_grid_side {
		for col in 0..num_tiles_in_grid_side {
			if row == 0 && col == 0 {
				grid.push((id_start, op_start));
				continue;
			}

			let pos = row * num_tiles_in_grid_side + col;
			let value =
				if col > 0 {
					let (id1, op1) = grid[pos - 1];
					neighbors[&(id1, op1)][0].ok_or_else(|| format!("could not find right neighbor of ({:?}){}", op1, id1))?
				}
				else {
					let (id1, op1) = grid[pos - num_tiles_in_grid_side];
					neighbors[&(id1, op1)][1].ok_or_else(|| format!("could not find down neighbor of ({:?}){}", op1, id1))?
				};
			grid.push(value);
		}
	}

	let merged_grid = {
		let mut merged_grid = vec![vec![false; 8 * num_tiles_in_grid_side]; 8 * num_tiles_in_grid_side];

		for (pos, (id, op)) in grid.into_iter().enumerate() {
			let (row, col) = (pos / num_tiles_in_grid_side, pos % num_tiles_in_grid_side);

			let src = &tiles[&id];
			let src = &[
				&src[1][1..][..8],
				&src[2][1..][..8],
				&src[3][1..][..8],
				&src[4][1..][..8],
				&src[5][1..][..8],
				&src[6][1..][..8],
				&src[7][1..][..8],
				&src[8][1..][..8],
			];

			let (row_0, rest) = merged_grid[(row * 8)..].split_first_mut().expect("must succeed");
			let (row_1, rest) = rest.split_first_mut().expect("must succeed");
			let (row_2, rest) = rest.split_first_mut().expect("must succeed");
			let (row_3, rest) = rest.split_first_mut().expect("must succeed");
			let (row_4, rest) = rest.split_first_mut().expect("must succeed");
			let (row_5, rest) = rest.split_first_mut().expect("must succeed");
			let (row_6, rest) = rest.split_first_mut().expect("must succeed");
			let (row_7, _) = rest.split_first_mut().expect("must succeed");
			let dest = &mut [
				&mut row_0[(col * 8)..][..8],
				&mut row_1[(col * 8)..][..8],
				&mut row_2[(col * 8)..][..8],
				&mut row_3[(col * 8)..][..8],
				&mut row_4[(col * 8)..][..8],
				&mut row_5[(col * 8)..][..8],
				&mut row_6[(col * 8)..][..8],
				&mut row_7[(col * 8)..][..8],
			];

			op.transform(src, dest);
		}

		merged_grid
	};

	let result =
		ALL_OPS.iter()
		.find_map(|&op| {
			let mut transformed_grid = vec![vec![false; 8 * num_tiles_in_grid_side]; 8 * num_tiles_in_grid_side];
			op.transform(&merged_grid, &mut transformed_grid);

			let num_sea_monsters =
				(0..(8 * num_tiles_in_grid_side - 2))
				.flat_map(|row| (0..(8 * num_tiles_in_grid_side - 18)).map(move |col| (row, col)))
				.filter(|&(row, col)| is_sea_monster(&transformed_grid, row, col))
				.count();

			(num_sea_monsters > 0).then(|| transformed_grid.iter().flatten().filter(|&&value| value).count() - num_sea_monsters * 15)
		})
		.ok_or("no solution")?;
	Ok(result)
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Op {
	None,
	RotateRight,
	RotateRight2,
	RotateRight3,
	FlipLeftRight,
	FlipTopBottom,
	FlipTopLeftBottomRight,
	FlipTopRightBottomLeft,
}

impl Op {
	fn transform(self, src: &[impl AsRef<[bool]>], dest: &mut [impl AsMut<[bool]>]) {
		let len = src.len();

		for (i, row) in src.iter().enumerate() {
			for (j, &value) in row.as_ref().iter().enumerate() {
				match self {
					Op::None => dest[i].as_mut()[j] = value,
					Op::RotateRight => dest[j].as_mut()[len - 1 - i] = value,
					Op::RotateRight2 => dest[len - 1 - i].as_mut()[len - 1 - j] = value,
					Op::RotateRight3 => dest[len - 1 - j].as_mut()[i] = value,
					Op::FlipLeftRight => dest[i].as_mut()[len - 1 - j] = value,
					Op::FlipTopBottom => dest[len - 1 - i].as_mut()[j] = value,
					Op::FlipTopLeftBottomRight => dest[len - 1 - j].as_mut()[len - 1 - i] = value,
					Op::FlipTopRightBottomLeft => dest[j].as_mut()[i] = value,
				}
			}
		}
	}
}

const ALL_OPS: &[Op] = &[
	Op::None,
	Op::RotateRight, Op::RotateRight2, Op::RotateRight3,
	Op::FlipLeftRight, Op::FlipTopBottom,
	Op::FlipTopLeftBottomRight, Op::FlipTopRightBottomLeft,
];

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

";

	#[test]
	fn part1() {
		let input = super::Input::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&input), 1951 * 3079 * 2971 * 1171);
	}

	#[test]
	fn part2() {
		let input = super::Input::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&input).unwrap(), 273);
	}
}
