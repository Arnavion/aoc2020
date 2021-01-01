pub(super) fn run() -> Result<(), super::Error> {
	{
		let mut input = super::read_input_lines::<String>("day19")?;
		let rules = Rule::parse(&mut input)?;

		let mut result = 0;
		for line in input {
			let line = line?;

			if matches1(&line, &rules) {
				result += 1;
			}
		}

		println!("19a: {}", result);

		assert_eq!(result, 147);
	}

	{
		let mut input = super::read_input_lines::<String>("day19")?;
		let rules = Rule::parse(&mut input)?;

		Rule::validate_for_part2(&rules)?;

		let mut result = 0;
		for line in input {
			let line = line?;

			if matches2(&line, &rules) {
				result += 1;
			}
		}

		println!("19b: {}", result);

		assert_eq!(result, 263);
	}

	Ok(())
}

#[derive(Clone, Debug)]
enum Rule<'a> {
	Str(String),
	Alts(std::borrow::Cow<'a, [std::borrow::Cow<'a, [u16]>]>),
}

impl Rule<'_> {
	fn parse(input: &mut impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<std::collections::BTreeMap<u16, Rule<'static>>, super::Error> {
		let mut rules: std::collections::BTreeMap<u16, Rule<'_>> = Default::default();

		for line in &mut *input {
			let line = line?;
			let line = line.as_ref();

			if line.is_empty() {
				return Ok(rules);
			}

			let mut parts = line.splitn(2, ": ");

			let id = parts.next().expect("str::split yields at least one part").parse().map_err(|err| format!("malformed rule {:?}: {}", line, err))?;

			let value = parts.next().ok_or_else(|| format!("malformed rule {:?}", line))?;
			let rule =
				if let Some(value) = value.strip_prefix(r#"""#) {
					let value = value.strip_suffix(r#"""#).ok_or_else(|| format!("malformed rule {:?}", line))?;
					Rule::Str(value.to_owned())
				}
				else {
					let mut alts = vec![];
					let mut alt = vec![];

					for s in value.split(' ').chain(std::iter::once("|")) {
						if s == "|" {
							alts.push(alt.into());
							alt = vec![];
						}
						else {
							let alt_id = s.parse().map_err(|err| format!("malformed rule {:?}: {}", line, err))?;
							alt.push(alt_id);
						}
					}

					Rule::Alts(alts.into())
				};

			rules.insert(id, rule);
		}

		Err("EOF".into())
	}

	fn validate_for_part2(rules: &std::collections::BTreeMap<u16, Self>) -> Result<(), super::Error> {
		// matches2 only works if rule 0 is `8 | 11`
		match &rules[&0] {
			Rule::Alts(alts) if matches!(&**alts, [alt] if matches!(&**alt, &[8, 11])) => Ok(()),
			rule => Err(format!("expected rule `0: 8 | 11` but it's {:?}", rule).into()),
		}
	}
}

fn matches1(message: &str, rules: &std::collections::BTreeMap<u16, Rule<'_>>) -> bool {
	matches_inner(message, &rules[&0], rules) == Some(message.len())
}

fn matches2(message: &str, rules: &std::collections::BTreeMap<u16, Rule<'_>>) -> bool {
	// Rule 0 is:
	//
	//     0: 8 11
	//
	// The proposed modification is:
	//
	//     8: 42 | 42 8
	//    11: 42 31 | 42 11 31
	//
	// Combined, these rules effectively make:
	//
	//     0: 42{N1} 31{N2}
	//
	// ... where 1 <= N2 < N1 < message.len()
	//
	// So, a simple implementation is to take all possible pairs of substrings from splitting the original message,
	// then for each pair, check if the first substring matches N1 instances of 42, and the second substring matches N2 instances of 31.

	for i in 1..message.len() {
		let (message1, message2) = message.split_at(i);

		let mut alt_42s = Vec::with_capacity(message.len() - 1);
		let mut alt_31s = Vec::with_capacity(message.len() - 1);

		let mut num_42s_in_message1 = None;

		for num_42s in 1..message1.len() {
			alt_42s.clear();
			alt_42s.resize(num_42s, 42);
			let alts = [(&alt_42s[..]).into()];
			let rule = Rule::Alts((&alts[..]).into());
			if matches_inner(message1, &rule, rules) == Some(message1.len()) {
				num_42s_in_message1 = Some(num_42s);
				break;
			}
		}

		let num_42s_in_message1 =
			if let Some(num_42s_in_message1) = num_42s_in_message1 {
				num_42s_in_message1
			}
			else {
				continue;
			};

		for num_31s in 1..std::cmp::min(num_42s_in_message1, message2.len()) {
			alt_31s.clear();
			alt_31s.resize(num_31s, 31);
			let alts = [(&alt_31s[..]).into()];
			let rule = Rule::Alts((&alts[..]).into());
			if matches_inner(message2, &rule, rules) == Some(message2.len()) {
				return true;
			}
		}
	}

	false
}

fn matches_inner(message: &str, rule: &Rule<'_>, rules: &std::collections::BTreeMap<u16, Rule<'_>>) -> Option<usize> {
	match rule {
		Rule::Str(s) =>
			if message.starts_with(s) {
				Some(s.len())
			}
			else {
				None
			},

		Rule::Alts(alts) => {
			for alt in &**alts {
				let mut message = message;
				let mut matched_len = Some(0);
				for &id in &**alt {
					if let Some(len) = matches_inner(message, &rules[&id], rules) {
						matched_len = Some(matched_len.unwrap() + len);
						message = &message[len..];
					}
					else {
						matched_len = None;
						break;
					}
				}

				if let Some(matched_len) = matched_len {
					return Some(matched_len);
				}
			}

			None
		},
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1_1() {
		const INPUT: &str = "\
0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"
";
		let rules = super::Rule::parse(&mut INPUT.split('\n').map(Ok)).unwrap();

		for &message in &[
			"aab",
			"aba",
		] {
			assert!(super::matches1(message, &rules), "{:?} should have parsed successfully", message);
		}
	}

	#[test]
	fn part1_2() {
		const INPUT: &str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"
";
		let rules = super::Rule::parse(&mut INPUT.split('\n').map(Ok)).unwrap();

		for &message in &[
			"aaaabb",
			"aaabab",
			"abbabb",
			"abbbab",
			"aabaab",
			"aabbbb",
			"abaaab",
			"ababbb",
		] {
			assert!(super::matches1(message, &rules), "{:?} should have parsed successfully", message);
		}
	}

		const INPUT3: &str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1
";

	#[test]
	fn part1_3() {
		let rules = super::Rule::parse(&mut INPUT3.split('\n').map(Ok)).unwrap();

		for &message in &[
			"bbabbbbaabaabba",
			"ababaaaaaabaaab",
			"ababaaaaabbbaba",
		] {
			assert!(super::matches1(message, &rules), "{:?} should have parsed successfully", message);
		}

		for &message in &[
			"abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
			"babbbbaabbbbbabbbbbbaabaaabaaa",
			"aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
			"bbbbbbbaaaabbbbaaabbabaaa",
			"bbbababbbbaaaaaaaabbababaaababaabab",
			"baabbaaaabbaaaababbaababb",
			"abbbbabbbbaaaababbbbbbaaaababb",
			"aaaaabbaabaaaaababaa",
			"aaaabbaaaabbaaa",
			"aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
			"babaaabbbaaabaababbaabababaaab",
			"aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
		] {
			assert!(!super::matches1(message, &rules), "{:?} should not have parsed successfully", message);
		}
	}

	#[test]
	fn part2() {
		let rules = super::Rule::parse(&mut INPUT3.split('\n').map(Ok)).unwrap();
		super::Rule::validate_for_part2(&rules).unwrap();

		for &message in &[
			"bbabbbbaabaabba",
			"babbbbaabbbbbabbbbbbaabaaabaaa",
			"aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
			"bbbbbbbaaaabbbbaaabbabaaa",
			"bbbababbbbaaaaaaaabbababaaababaabab",
			"ababaaaaaabaaab",
			"ababaaaaabbbaba",
			"baabbaaaabbaaaababbaababb",
			"abbbbabbbbaaaababbbbbbaaaababb",
			"aaaaabbaabaaaaababaa",
			"aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
			"aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
		] {
			assert!(super::matches2(message, &rules), "{:?} should have parsed successfully", message);
		}

		for &message in &[
			"abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
			"aaaabbaaaabbaaa",
			"babaaabbbaaabaababbaabababaaab",
		] {
			assert!(!super::matches1(message, &rules), "{:?} should not have parsed successfully", message);
		}
	}
}
