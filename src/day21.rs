pub(super) fn run() -> Result<(), super::Error> {
	let (recipes, allergenic_ingredients) = solve(super::read_input_lines::<String>("day21")?)?;

	{
		let result = part1(&recipes, &allergenic_ingredients);

		println!("21a: {result}");

		assert_eq!(result, 2542);
	}

	{
		let result = part2(&allergenic_ingredients);

		println!("21b: {result}");

		assert_eq!(result, "hkflr,ctmcqjf,bfrq,srxphcm,snmxl,zvx,bd,mqvk");
	}

	Ok(())
}

fn solve(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) ->
	Result<
		(
			Vec<(std::collections::BTreeSet<String>, std::collections::BTreeSet<String>)>,
			std::collections::BTreeMap<String, String>,
		),
		super::Error,
	>
{
	let mut recipes = vec![];

	for line in input {
		let line = line?;
		let line = line.as_ref();

		let parts = line.split(' ');
		let mut ingredients: std::collections::BTreeSet<_> = Default::default();
		let mut allergens: std::collections::BTreeSet<_> = Default::default();

		let mut parsing_allergens = false;

		for part in parts {
			if parsing_allergens {
				let part = part.trim_end_matches(&[',', ')'][..]);
				allergens.insert(part.to_owned());
			}
			else if part == "(contains" {
				parsing_allergens = true;
			}
			else {
				ingredients.insert(part.to_owned());
			}
		}

		recipes.push((ingredients, allergens));
	}

	let mut allergenic_ingredient_possibilities: std::collections::BTreeMap<&str, std::collections::BTreeSet<&str>> = Default::default();

	for (ingredients, allergens) in &recipes {
		for allergen in allergens {
			match allergenic_ingredient_possibilities.entry(allergen) {
				std::collections::btree_map::Entry::Vacant(entry) => {
					entry.insert(ingredients.iter().map(AsRef::as_ref).collect());
				},

				std::collections::btree_map::Entry::Occupied(mut entry) => {
					entry.insert(entry.get() & &ingredients.iter().map(AsRef::as_ref).collect());
				},
			}
		}
	}

	let mut allergenic_ingredients: std::collections::BTreeMap<_, _> = Default::default();

	while !allergenic_ingredient_possibilities.is_empty() {
		let (&allergen, _) =
			allergenic_ingredient_possibilities.iter()
			.find(|(_, ingredients)| ingredients.len() == 1)
			.ok_or("no solution")?;
		let ingredients = allergenic_ingredient_possibilities.remove(&allergen).expect("key was just discovered");
		let ingredient = ingredients.into_iter().next().expect("value was just discovered to have one element");

		allergenic_ingredients.insert(allergen.to_owned(), ingredient.to_owned());

		for ingredients in allergenic_ingredient_possibilities.values_mut() {
			ingredients.remove(&ingredient);
		}
	}

	Ok((recipes, allergenic_ingredients))
}

fn part1(
	recipes: &[(std::collections::BTreeSet<String>, std::collections::BTreeSet<String>)],
	allergenic_ingredients: &std::collections::BTreeMap<String, String>,
) -> usize {
	let allergenic_ingredients: std::collections::BTreeSet<&str> = allergenic_ingredients.values().map(AsRef::as_ref).collect();

	let num_occurrences_of_non_allergenic_ingredients =
		recipes.iter()
		.flat_map(|(ingredients, _)| ingredients)
		.filter(|&ingredient| !allergenic_ingredients.contains(&**ingredient))
		.count();

	num_occurrences_of_non_allergenic_ingredients
}

fn part2(allergenic_ingredients: &std::collections::BTreeMap<String, String>) -> String {
	let mut result = String::new();
	for ingredient in allergenic_ingredients.values() {
		if !result.is_empty() {
			result.push(',');
		}
		result.push_str(ingredient);
	}
	result
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)\
";

	#[test]
	fn part1() {
		let (recipes, allergenic_ingredients) = super::solve(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&recipes, &allergenic_ingredients), 5);
	}

	#[test]
	fn part2() {
		let (_, allergenic_ingredients) = super::solve(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&allergenic_ingredients), "mxmxvkd,sqjhc,fvjkl");
	}
}
