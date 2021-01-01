pub(super) fn run() -> Result<(), super::Error> {
	let passports = Passport::parse(super::read_input_lines::<String>("day4")?)?;

	{
		let result = passports.iter().filter(|passport| passport.is_valid1()).count();

		println!("4a: {}", result);

		assert_eq!(result, 216);
	}

	{
		let result = passports.iter().filter(|passport| passport.is_valid2()).count();

		println!("4b: {}", result);

		assert_eq!(result, 150);
	}

	Ok(())
}

#[derive(Debug, Default)]
struct Passport {
	byr: Option<String>,
	cid: Option<String>,
	ecl: Option<String>,
	eyr: Option<String>,
	hcl: Option<String>,
	hgt: Option<String>,
	iyr: Option<String>,
	pid: Option<String>,
}

impl Passport {
	fn parse(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) -> Result<Vec<Self>, super::Error> {
		let mut passports = vec![];

		let mut passport: Passport = Default::default();
		for line in input {
			let line = line?;
			let line = line.as_ref();

			if line.is_empty() {
				passports.push(passport);
				passport = Default::default();
			}
			else {
				for field in line.split(' ') {
					let mut parts = field.split(':');

					let key = parts.next().expect("str::split yields at least one part");
					let value = parts.next().ok_or_else(|| format!("malformed line {:?}", line))?;

					match key {
						"byr" => passport.byr = Some(value.to_owned()),
						"cid" => passport.cid = Some(value.to_owned()),
						"ecl" => passport.ecl = Some(value.to_owned()),
						"eyr" => passport.eyr = Some(value.to_owned()),
						"hcl" => passport.hcl = Some(value.to_owned()),
						"hgt" => passport.hgt = Some(value.to_owned()),
						"iyr" => passport.iyr = Some(value.to_owned()),
						"pid" => passport.pid = Some(value.to_owned()),
						_ => return Err(format!("malformed line {:?}", line).into()),
					}
				}
			}
		}
		passports.push(passport);

		Ok(passports)
	}

	fn is_valid1(&self) -> bool {
		let Passport { byr, cid: _, ecl, eyr, hcl, hgt, iyr, pid } = self;
		byr.as_ref()
			.and(ecl.as_ref())
			.and(eyr.as_ref())
			.and(hcl.as_ref())
			.and(hgt.as_ref())
			.and(iyr.as_ref())
			.and(pid.as_ref())
			.is_some()
	}

	fn is_valid2(&self) -> bool {
		fn r#catch(f: impl FnOnce() -> Option<()>) -> bool {
			f().is_some()
		}

		r#catch(|| {
			let Passport { byr, cid: _, ecl, eyr, hcl, hgt, iyr, pid } = self;

			{
				let byr = byr.as_deref()?;
				let byr: u16 = byr.parse().ok()?;
				if !(1920..=2002).contains(&byr) {
					return None;
				}
			}

			{
				let ecl = ecl.as_deref()?;
				if !matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
					return None;
				}
			}

			{
				let eyr = eyr.as_deref()?;
				let eyr: u16 = eyr.parse().ok()?;
				if !(2020..=2030).contains(&eyr) {
					return None;
				}
			}

			{
				let hcl = hcl.as_deref()?;
				let mut hcl = hcl.chars();
				if hcl.next()? != '#' {
					return None;
				}
				for _ in 0..6 {
					let c = hcl.next()?;
					if !('0'..='9').contains(&c) && !('a'..='f').contains(&c) {
						return None;
					}
				}
				if hcl.next().is_some() {
					return None;
				}
			}

			{
				let hgt = hgt.as_deref()?;
				if let Some(hgt) = hgt.strip_suffix("cm") {
					let hgt: u8 = hgt.parse().ok()?;
					if !(150..=193).contains(&hgt) {
						return None;
					}
				}
				else if let Some(hgt) = hgt.strip_suffix("in") {
					let hgt: u8 = hgt.parse().ok()?;
					if !(59..=76).contains(&hgt) {
						return None;
					}
				}
				else {
					return None;
				}
			}

			{
				let iyr = iyr.as_deref()?;
				let iyr: u16 = iyr.parse().ok()?;
				if !(2010..=2020).contains(&iyr) {
					return None;
				}
			}

			{
				let pid = pid.as_deref()?;
				let mut pid = pid.chars();
				for _ in 0..9 {
					let c = pid.next()?;
					if !('0'..='9').contains(&c) {
						return None;
					}
				}
				if pid.next().is_some() {
					return None;
				}
			}


			Some(())
		})
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		const INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

		let passports = super::Passport::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(passports.iter().filter(|passport| passport.is_valid1()).count(), 2);
	}

	#[test]
	fn part2() {
		const INPUT_INVALID: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

		const INPUT_VALID: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

		let passports = super::Passport::parse(INPUT_INVALID.split('\n').map(Ok)).unwrap();
		assert_eq!(passports.iter().filter(|passport| passport.is_valid2()).count(), 0);

		let passports = super::Passport::parse(INPUT_VALID.split('\n').map(Ok)).unwrap();
		assert_eq!(passports.iter().filter(|passport| passport.is_valid2()).count(), 4);
	}
}
