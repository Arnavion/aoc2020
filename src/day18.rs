pub(super) fn run() -> Result<(), super::Error> {
	{
		let result = part1(super::read_input_lines::<String>("day18")?)?;

		println!("18a: {result}");

		assert_eq!(result, 8929569623593);
	}

	{
		let result = part2(super::read_input_lines::<String>("day18")?)?;

		println!("18b: {result}");

		assert_eq!(result, 231235959382961);
	}

	Ok(())
}

fn part1(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<u64, super::Error> {
	let mut sum = 0;

	for line in input {
		let line = line?;
		let mut tokens = Token::parse(line.as_ref());
		let result = evaluate(&mut tokens, false)?;
		sum += result;
	}

	Ok(sum)
}

fn part2(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<u64, super::Error> {
	let mut sum = 0;

	for line in input {
		let line = line?;
		let mut tokens = Token::parse(line.as_ref());
		let result = evaluate(&mut tokens, true)?;
		sum += result;
	}

	Ok(sum)
}

#[derive(Clone, Copy, Debug)]
enum Token {
	Num(u64),
	Plus,
	Star,
	ParenOpen,
	ParenClose,
}

impl Token {
	fn parse(s: &str) -> impl Iterator<Item = Result<Self, super::Error>> + '_ {
		s.chars()
			.filter_map(|c| match c {
				' ' => None,
				c @ '0'..='9' => Some(Ok(Token::Num(u64::from(c) - u64::from('0')))),
				'+' => Some(Ok(Token::Plus)),
				'*' => Some(Ok(Token::Star)),
				'(' => Some(Ok(Token::ParenOpen)),
				')' => Some(Ok(Token::ParenClose)),
				c => Some(Err(format!("unexpected character {c:?}").into())),
			})
	}
}

#[derive(Clone, Copy, Debug)]
enum State {
	/// ``; expect number / `(`
	Start,

	/// `N`; expect operator / `)` / EOF
	Num(u64),

	/// `N1 * N2`; expect operator / `)` / EOF
	ProductNum(u64, u64),

	/// `N +`; expect number / `(`
	Add(u64),

	/// `N *`; expect number / `(`
	Mul(u64),

	/// `N1 * N2 +`; expect number / `(`
	MulAdd(u64, u64),
}

fn evaluate(tokens: &mut impl Iterator<Item = Result<Token, super::Error>>, add_has_precedence: bool) -> Result<u64, super::Error> {
	let mut state = State::Start;

	loop {
		let token = tokens.next().transpose()?;
		state = match (state, token) {
			(State::Start, Some(Token::Num(num))) => State::Num(num),
			(State::Start, Some(Token::ParenOpen)) => State::Num(evaluate(&mut *tokens, add_has_precedence)?),

			(State::Num(num), Some(Token::Plus)) => State::Add(num),
			(State::Num(num), Some(Token::Star)) => State::Mul(num),
			(State::Num(num), Some(Token::ParenClose) | None) => return Ok(num),

			(State::ProductNum(num1, num2), Some(Token::Plus)) => State::MulAdd(num1, num2),
			(State::ProductNum(num1, num2), Some(Token::Star)) => State::Mul(num1 * num2),
			(State::ProductNum(num1, num2), Some(Token::ParenClose) | None) => return Ok(num1 * num2),

			(State::Add(num1), Some(Token::Num(num2))) => State::Num(num1 + num2),
			(State::Add(num), Some(Token::ParenOpen)) => State::Num(num + evaluate(&mut *tokens, add_has_precedence)?),

			(State::Mul(num1), Some(Token::Num(num2))) if add_has_precedence => State::ProductNum(num1, num2),
			(State::Mul(num), Some(Token::ParenOpen)) if add_has_precedence => State::ProductNum(num, evaluate(&mut *tokens, add_has_precedence)?),

			(State::Mul(num1), Some(Token::Num(num2))) => State::Num(num1 * num2),
			(State::Mul(num), Some(Token::ParenOpen)) => State::Num(num * evaluate(&mut *tokens, add_has_precedence)?),

			(State::MulAdd(num1, num2), Some(Token::Num(num3))) => State::ProductNum(num1, num2 + num3),
			(State::MulAdd(num1, num2), Some(Token::ParenOpen)) => State::ProductNum(num1, num2 + evaluate(&mut *tokens, add_has_precedence)?),

			(state, Some(c)) => return Err(format!("unexpected char {c:?} in state {state:?}").into()),

			(_, None) => return Err("EOF".into()),
		};
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		for &(input, expected) in &[
			("1 + 2 * 3 + 4 * 5 + 6", 71),
			("1 + (2 * 3) + (4 * (5 + 6))", 51),
			("2 * 3 + (4 * 5)", 26),
			("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
			("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
			("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
		] {
			let mut tokens = super::Token::parse(input);
			assert_eq!(super::evaluate(&mut tokens, false).unwrap(), expected);
		}
	}

	#[test]
	fn part2() {
		for &(input, expected) in &[
			("1 + 2 * 3 + 4 * 5 + 6", 231),
			("1 + (2 * 3) + (4 * (5 + 6))", 51),
			("2 * 3 + (4 * 5)", 46),
			("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
			("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
			("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
		] {
			let mut tokens = super::Token::parse(input);
			assert_eq!(super::evaluate(&mut tokens, true).unwrap(), expected);
		}
	}
}
