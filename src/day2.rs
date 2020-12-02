pub(super) fn run() -> Result<(), super::Error> {
	{
		let mut result = 0;

		for line in super::read_input_lines::<String>("day2")? {
			let line = line?;
			if check_password1(&line)? {
				result += 1;
			}
		}

		println!("2a: {}", result);

		assert_eq!(result, 416);
	}

	{
		let mut result = 0;

		for line in super::read_input_lines::<String>("day2")? {
			let line = line?;
			if check_password2(&line)? {
				result += 1;
			}
		}

		println!("2b: {}", result);

		assert_eq!(result, 688);
	}

	Ok(())
}

fn check_password1(line: &str) -> Result<bool, super::Error> {
	let (low, high, c, password) = parse_line(&line)?;
	let num_c = password.chars().filter(|&c_| c_ == c).count();
	Ok((low..=high).contains(&num_c))
}

fn check_password2(line: &str) -> Result<bool, super::Error> {
	let (low, high, c, password) = parse_line(&line)?;
	let low_matches = password.chars().nth(low - 1) == Some(c);
	let high_matches = password.chars().nth(high - 1) == Some(c);
	Ok(low_matches ^ high_matches)
}

fn parse_line(line: &str) -> Result<(usize, usize, char, &str), super::Error> {
	static LINE_REGEX: once_cell::sync::Lazy<regex::Regex> =
		once_cell::sync::Lazy::new(||
			regex::Regex::new(r"^(?P<low>\d+)-(?P<high>\d+) (?P<c>[a-z]): (?P<password>[a-z]+)$")
			.expect("hard-coded regex must compile successfully"));

	let captures = LINE_REGEX.captures(&line).ok_or_else(|| format!("input line {:?} has invalid format", line))?;

	let low: usize = captures["low"].parse()?;
	let high: usize = captures["high"].parse()?;
	let c = captures["c"].chars().next().expect("regex matches one-char string");
	let password = captures.name("password").expect("regex contains capture group with this name").as_str();

	Ok((low, high, c, password))
}

#[cfg(test)]
mod tests {
	#[test]
	fn check_password1() {
		assert!(super::check_password1("1-3 a: abcde").unwrap());
		assert!(!super::check_password1("1-3 b: cdefg").unwrap());
		assert!(super::check_password1("2-9 c: ccccccccc").unwrap());
	}

	#[test]
	fn check_password2() {
		assert!(super::check_password2("1-3 a: abcde").unwrap());
		assert!(!super::check_password2("1-3 b: cdefg").unwrap());
		assert!(!super::check_password2("2-9 c: ccccccccc").unwrap());
	}
}
