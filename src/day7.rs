pub(super) fn run() -> Result<(), super::Error> {
	let (graph, shiny_gold_node_index) = parse(super::read_input_lines::<String>("day7")?)?;

	{
		let result = part1(&graph, shiny_gold_node_index);

		println!("7a: {result}");

		assert_eq!(result, 164);
	}

	{
		let result = part2(&graph, shiny_gold_node_index);

		println!("7b: {result}");

		assert_eq!(result, 7872);
	}

	Ok(())
}

fn parse(input: impl Iterator<Item = Result<impl AsRef<str>, super::Error>>) ->
	Result<
		(
			petgraph::Graph<(), usize>,
			petgraph::graph::NodeIndex<petgraph::graph::DefaultIx>,
		),
		super::Error,
	>
{
	static LINE_REGEX: once_cell::sync::Lazy<regex::Regex> =
		once_cell::sync::Lazy::new(||
			regex::Regex::new(r"^(?P<kind>\S+ \S+) bag(?:s?) contain (?P<contents>.+)\.$")
			.expect("hard-coded regex must compile successfully"));

	static CONTENT_REGEX: once_cell::sync::Lazy<regex::Regex> =
		once_cell::sync::Lazy::new(||
			regex::Regex::new(r"^(?P<num>\d+) (?P<kind>\S+ \S+) bag(?:s?)$")
			.expect("hard-coded regex must compile successfully"));

	let mut graph = petgraph::Graph::new();

	let mut nodes: std::collections::BTreeMap<_, _> = Default::default();

	for line in input {
		let line = line?;
		let line = line.as_ref();

		let captures = LINE_REGEX.captures(line).ok_or_else(|| format!("input line {line:?} has invalid format"))?;

		let kind = &captures["kind"];
		let contents = &captures["contents"];

		let node_index =
			if let Some(&node_index) = nodes.get(kind) {
				node_index
			}
			else {
				let node_index = graph.add_node(());
				nodes.insert(kind.to_owned(), node_index);
				node_index
			};

		if contents == "no other bags" {
			continue;
		}

		let contents = contents.split(", ");

		for content in contents {
			let captures = CONTENT_REGEX.captures(content).ok_or_else(|| format!("input line {line:?} has invalid format"))?;

			let content_num: usize = captures["num"].parse()?;
			let content_kind = &captures["kind"];

			let content_node_index =
				if let Some(&node_index) = nodes.get(content_kind) {
					node_index
				}
				else {
					let node_index = graph.add_node(());
					nodes.insert(content_kind.to_owned(), node_index);
					node_index
				};

			graph.update_edge(node_index, content_node_index, content_num);
		}
	}

	graph.shrink_to_fit();

	let &shiny_gold_node_index = nodes.get("shiny gold").ok_or("could not find rule for shiny gold bags")?;

	Ok((graph, shiny_gold_node_index))
}

fn part1(graph: &petgraph::Graph<(), usize>, shiny_gold_node_index: petgraph::graph::NodeIndex<petgraph::graph::DefaultIx>) -> usize {
	let graph = petgraph::visit::Reversed(graph);
	let walker = petgraph::visit::Bfs::new(graph, shiny_gold_node_index);
	let walker = petgraph::visit::Walker::iter(walker, graph);
	walker.count() - 1 // Don't count the original shiny gold bag.
}

fn part2(graph: &petgraph::Graph<(), usize>, shiny_gold_node_index: petgraph::graph::NodeIndex<petgraph::graph::DefaultIx>) -> usize {
	let mut result = 0;

	let mut to_visit: std::collections::VecDeque<_> = std::iter::once((shiny_gold_node_index, 1)).collect();

	while let Some((node_index, num)) = to_visit.pop_front() {
		result += num;

		for edge in graph.edges(node_index) {
			let &content_num = edge.weight();
			let content_node_index = petgraph::visit::EdgeRef::target(&edge);
			to_visit.push_back((content_node_index, num * content_num));
		}
	}

	// Don't count the original shiny gold bag.
	result - 1
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.\
";

	#[test]
	fn part1() {
		let (graph, shiny_gold_node_index) = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part1(&graph, shiny_gold_node_index), 4);
	}

	#[test]
	fn part2() {
		const INPUT2: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.\
";

		let (graph, shiny_gold_node_index) = super::parse(INPUT.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&graph, shiny_gold_node_index), 32);

		let (graph, shiny_gold_node_index) = super::parse(INPUT2.split('\n').map(Ok)).unwrap();
		assert_eq!(super::part2(&graph, shiny_gold_node_index), 126);
	}
}
