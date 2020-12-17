pub(super) fn run() -> Result<(), super::Error> {
	let mut grid = Grid::parse(super::read_input_lines::<String>("day17")?)?;

	{
		let mut grid = grid.clone();

		let result = part1(&mut grid);

		println!("17a: {}", result);

		assert_eq!(result, 267);
	}

	{
		let result = part2(&mut grid);

		println!("17b: {}", result);

		assert_eq!(result, 1812);
	}

	Ok(())
}

type BitSetBlock = u8; // Empirically determined to be very slightly faster than other u* for 17b: ~28ms for u8 vs ~32ms for u64
const BITSET_NUM_BLOCKS: usize = (NUM_CUBES as usize + std::mem::size_of::<BitSetBlock>() - 1) / std::mem::size_of::<BitSetBlock>();

#[derive(Clone, Debug)]
struct Grid {
	// `inner: std::collections::BTreeSet<(i8, i8, i8, i8)>` has better space usage and fewer assumptions about the input size and number of iterations,
	// but takes much longer despite the more complex accesses for the bitset. 17b takes ~300ms with a BTreeSet vs ~5ms with the bitset,
	// albeit the BTreeSet only has ~2k bools while the bitset occupies ~13KiB.
	inner: Box<[BitSetBlock; BITSET_NUM_BLOCKS]>,
}

// The puzzle grid is 8x8x1x1. Each iteration adds two lines to every dimension (-1 and +1).
// The neighbor check on every iteration needs an additional line in every dimension. Therefore, across 6 iterations,
// 6 * 2 + 2 = 14 lines are added to every dimension, and the smallest index accessed in any dimension is -7.
//
// Therefore the max number of cubes is (8 + 14) * (8 + 14) * (1 + 14) * (1 + 14).

const PUZZLE_MAX_X: i8 = 7;
const PUZZLE_MAX_Y: i8 = 7;
const PUZZLE_MAX_Z: i8 = 0;
const PUZZLE_MAX_W: i8 = 0;
const NUM_ITERATIONS: i8 = 6;

const NUM_CUBES: usize =
	(PUZZLE_MAX_X as usize + 1 + NUM_ITERATIONS as usize * 2 + 2) *
	(PUZZLE_MAX_Y as usize + 1 + NUM_ITERATIONS as usize * 2 + 2) *
	(PUZZLE_MAX_Z as usize + 1 + NUM_ITERATIONS as usize * 2 + 2) *
	(PUZZLE_MAX_W as usize + 1 + NUM_ITERATIONS as usize * 2 + 2);

const MAX_DISTANCE: usize = NUM_ITERATIONS as usize + 1;
const STRIDE_X: usize = PUZZLE_MAX_Y as usize + 1 + MAX_DISTANCE * 2;
const STRIDE_Y: usize = PUZZLE_MAX_Z as usize + 1 + MAX_DISTANCE * 2;
const STRIDE_Z: usize = PUZZLE_MAX_W as usize + 1 + MAX_DISTANCE * 2;

const OFFSET: usize =
	MAX_DISTANCE * STRIDE_X * STRIDE_Y * STRIDE_Z +
	MAX_DISTANCE * STRIDE_Y * STRIDE_Z +
	MAX_DISTANCE * STRIDE_Z +
	MAX_DISTANCE;

impl Grid {
	fn parse(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Self, super::Error> {
		let mut grid = Grid {
			inner: Box::new([0; BITSET_NUM_BLOCKS]),
		};

		for (x, line) in input.enumerate() {
			let line = line?;
			let line = line.as_ref();
			for (y, c) in line.chars().enumerate() {
				if c == '#' {
					let position = (
						std::convert::TryInto::try_into(x).map_err(|err| format!("cell ({}, {}) is out of range: {}", x, y, err))?,
						std::convert::TryInto::try_into(y).map_err(|err| format!("cell ({}, {}) is out of range: {}", x, y, err))?,
						0,
						0,
					);
					let base = Self::position_to_index_base(position);
					unsafe { grid.set_raw(base + OFFSET, true); }
				}
			}
		}

		Ok(grid)
	}

	fn num_active(&self) -> usize {
		self.inner.iter().map(|&block| block.count_ones() as usize).sum()
	}

	unsafe fn get_raw(&self, index: usize) -> bool {
		let block_index = index / std::mem::size_of::<BitSetBlock>();
		let bit_index = index % std::mem::size_of::<BitSetBlock>();
		let block = *self.inner.get_unchecked(block_index);
		let bit = block & ((1 as BitSetBlock) << bit_index);
		bit != 0
	}

	unsafe fn set_raw(&mut self, index: usize, value: bool) {
		let block_index = index / std::mem::size_of::<BitSetBlock>();
		let bit_index = index % std::mem::size_of::<BitSetBlock>();
		let block = self.inner.get_unchecked_mut(block_index);
		if value {
			*block |= (1 as BitSetBlock) << bit_index;
		}
		else {
			#[allow(clippy::cast_possible_truncation)] // `bit_index as u32` is fine because `bit_index < size_of::<BitSetBlock>()`
			{
				*block &= !((1 as BitSetBlock) << bit_index);
			}
		}
	}

	// Every position (x, y, z, w) is mapped to an index in the form:
	//
	//     P = Ax + By + Cz + Dw + OFFSET
	//
	// Every neighbor at (x + x_, y + y_, z + z_, w + w_) would thus be mapped to:
	//
	//     P_ = A(x + x_) + B(y + y_) + C(z + z_) + D(w + w_) + OFFSET
	//        = P + Ax_ + by_ + Cz_ + Dw_
	//
	// Given:
	//
	//     position_to_index_base(x, y, z, w) -> Ax + By + Cz + Dw
	//
	// ... we have:
	//
	//     P = position_to_index_base(x, y, z, w) + OFFSET
	//
	// ... and:
	//
	//     R_ = position_to_index_base(x_, y_, z_, w_)
	//     P_ = P + R_
	//
	// The advantage of this method is that x_, y_, z_, w_ vary in a known range,
	// and thus all values of R_ can be pre-computed for the entire run.

	#[allow(clippy::cast_sign_loss)] // Overflow will be caught by the asserts.
	fn position_to_index_base((x, y, z, w): (i8, i8, i8, i8)) -> usize {
		debug_assert!(((-NUM_ITERATIONS - 1)..=(PUZZLE_MAX_X + NUM_ITERATIONS + 1)).contains(&x));
		debug_assert!(((-NUM_ITERATIONS - 1)..=(PUZZLE_MAX_Y + NUM_ITERATIONS + 1)).contains(&y));
		debug_assert!(((-NUM_ITERATIONS - 1)..=(PUZZLE_MAX_Z + NUM_ITERATIONS + 1)).contains(&z));
		debug_assert!(((-NUM_ITERATIONS - 1)..=(PUZZLE_MAX_W + NUM_ITERATIONS + 1)).contains(&w));

		let base =
			(x as usize).wrapping_mul(STRIDE_X * STRIDE_Y * STRIDE_Z)
			.wrapping_add((y as usize).wrapping_mul(STRIDE_Y * STRIDE_Z))
			.wrapping_add((z as usize).wrapping_mul(STRIDE_Z))
			.wrapping_add(w as usize);
		base
	}
}

fn part1(grid: &mut Grid) -> usize {
	solve(grid, false)
}

fn part2(grid: &mut Grid) -> usize {
	solve(grid, true)
}

fn solve(grid: &mut Grid, consider_w: bool) -> usize {
	fn positions(min_max_i: Option<(i8, i8, i8)>) -> impl Iterator<Item = i8> {
		match min_max_i {
			Some((min, max, i)) => (min - i - 1)..=(max + i + 1),
			None => 0..=0,
		}
	}

	// Precompute all `position_to_index_base(x_, y_, z_, w_)`
	#[allow(clippy::filter_map)] // "more succinctly" my ass.
	let neighbors: Vec<_> =
		(-1..=1)
		.flat_map(|x| (-1..=1).map(move |y| (x, y)))
		.flat_map(|(x, y)| (-1..=1).map(move |z| (x, y, z)))
		.flat_map(|(x, y, z)| (if consider_w { -1..=1 } else { 0..=0 }).map(move |w| (x, y, z, w)))
		.filter(|&offset| offset != (0, 0, 0, 0))
		.map(Grid::position_to_index_base)
		.collect();

	let mut new_states = vec![];

	for i in 0..6 {
		new_states.extend(
			positions(Some((0, PUZZLE_MAX_X, i)))
			.flat_map(|x| positions(Some((0, PUZZLE_MAX_Y, i))).map(move |y| (x, y)))
			.flat_map(|(x, y)| positions(Some((0, PUZZLE_MAX_Z, i))).map(move |z| (x, y, z)))
			.flat_map(|(x, y, z)| positions(if consider_w { Some((0, PUZZLE_MAX_W, i)) } else { None }).map(move |w| (x, y, z, w)))
			.map(Grid::position_to_index_base)
			.filter_map(|base| {
				let index = base + OFFSET;

				let cube = unsafe { grid.get_raw(index) };

				let num_active_neighbors =
					neighbors.iter()
					.filter(|&&offset_| unsafe { grid.get_raw(index.wrapping_add(offset_)) })
					.count();

				#[allow(clippy::match_same_arms)]
				match (cube, num_active_neighbors) {
					(true, 2) | (true, 3) => None,
					(true, _) => Some((index, false)),
					(false, 3) => Some((index, true)),
					(false, _) => None,
				}
			}));

		for (index, new_state) in new_states.drain(..) {
			unsafe { grid.set_raw(index, new_state); }
		}
	}

	grid.num_active()
}

#[cfg(test)]
mod tests {
		const INPUT: &str = "\
.#.
..#
###
";

	#[test]
	fn part1() {
		let mut grid = super::Grid::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&mut grid), 112);
	}

	#[test]
	fn part2() {
		let mut grid = super::Grid::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&mut grid), 848);
	}
}
