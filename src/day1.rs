pub(super) fn run() -> Result<(), super::Error> {
	let input: Result<Vec<_>, super::Error> =
		super::read_input_lines::<u64>("day1")?
		.collect();
	let input = input?;

	{
		let result = find::<typenum::U2>(&input).ok_or("no solution")?;

		println!("1a: {}", result);

		assert_eq!(result, 751776);
	}

	{
		let result = find::<typenum::U3>(&input).ok_or("no solution")?;

		println!("1b: {}", result);

		assert_eq!(result, 42275090);
	}

	Ok(())
}

fn find<N>(input: &[u64]) -> Option<u64>
where
	N: Find,
{
	N::find(input, 2020)
}

trait Find {
	fn find(input: &'_ [u64], target: u64) -> Option<u64>;
}

/// This is a subset of the typenum crate. We can't use the typenum crate itself because of orphan rules -
/// rustc complains that a future version of typenum could impl Sub<B1> for U0 (even though that would be nonsensical).
/// So we define what we need ourselves.
mod typenum {
	macro_rules! sub1 {
		($n:ty => $n_minus_one:path) => {
			impl std::ops::Sub<U1> for $n {
				type Output = $n_minus_one;

				fn sub(self, _: U1) -> Self::Output {
					$n_minus_one
				}
			}
		};
	}

	pub(super) struct U0;

	pub(super) struct U1;
	sub1!(U1 => U0);

	pub(super) struct U2;
	sub1!(U2 => U1);

	pub(super) struct U3;
	sub1!(U3 => U2);
}

impl Find for typenum::U0 {
	fn find(_input: &'_ [u64], target: u64) -> Option<u64> {
		(target == 0).then(|| 1)
	}
}

impl<NRemaining> Find for NRemaining
where
	NRemaining: std::ops::Sub<typenum::U1>,
	<NRemaining as std::ops::Sub<typenum::U1>>::Output: Find,
{
	fn find(input: &'_ [u64], target: u64) -> Option<u64> {
		// Uses a while loop rather than:
		//
		//     input.iter().enumerate().find_map(|(i, &num)| { ...; remaining.find(&input[(i + 1)..], ...) })
		//
		// ... to be able to use `iter.as_slice()` for the recursive call instead. This is because `&input[(i + 1)..]` retains a bounds check,
		// whereas `iter.as_slice()` elids it.

		let mut iter = input.iter();

		// clippy thinks this can be replaced with `for &num in iter`, but that would prevent `iter.as_slice()` from being used in the closure
		#[allow(clippy::while_let_on_iterator)]
		while let Some(&num) = iter.next() {
			if let Some(product) =
				target.checked_sub(num)
				.and_then(|target| <<NRemaining as std::ops::Sub<typenum::U1>>::Output as Find>::find(iter.as_slice(), target))
			{
				return Some(product * num);
			}
		}

		None
	}
}

mod tests {
	#[test]
	fn find_two() {
		assert_eq!(super::find::<super::typenum::U2>(&[1721, 979, 366, 299, 675, 1456]).unwrap(), 1721 * 299);
	}

	#[test]
	fn find_three() {
		assert_eq!(super::find::<super::typenum::U3>(&[1721, 979, 366, 299, 675, 1456]).unwrap(), 979 * 366 * 675);
	}
}
