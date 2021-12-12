use aoc2021::*;

aoc_main!(
    day: 12,
    test_input: r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
    parser: parse,
    task_1: task_1,
    expected_1: 10,
    task_2: task_2,
    expected_2: 36,
);

#[derive(Debug, Clone, PartialEq)]
enum Cave {
    Start,
    End,
    Big { name: char },
    Small { name: char },
}

#[derive(Debug)]
struct ParsedInput {
    graph: Graph<Cave>,
    start: NodeIndex,
    end: NodeIndex,
}

fn parse(raw_input: &str) -> Result<ParsedInput> {
    let mut graph = Graph::new();
    let mut start = None;
    let mut end = None;

    let mut node_to_idx: HashMap<String, NodeIndex> = HashMap::new();

    for line in raw_input.lines() {
        let mut parts = line.split("-");
        let a = parts.next().ok_or(anyhow!("Invalid input"))?;
        let b = parts.next().ok_or(anyhow!("Invalid input"))?;

        for n in [a, b] {
            if !node_to_idx.contains_key(n) {
                let node_idx = if n == "start" {
                    let idx = graph.add_node(Cave::Start);
                    start = Some(idx);
                    idx
                } else if n == "end" {
                    let idx = graph.add_node(Cave::End);
                    end = Some(idx);
                    idx
                } else {
                    let c = n.chars().next().unwrap();
                    graph.add_node(if c.is_uppercase() {
                        Cave::Big { name: c }
                    } else {
                        Cave::Small { name: c }
                    })
                };

                node_to_idx.insert(n.to_string(), node_idx);
            }
        }
    }

    for line in raw_input.lines() {
        let mut parts = line.split("-");
        let a = parts.next().ok_or(anyhow!("Invalid input"))?;
        let b = parts.next().ok_or(anyhow!("Invalid input"))?;

        let a_idx = *node_to_idx.get(a).unwrap();
        let b_idx = *node_to_idx.get(b).unwrap();

        graph.add_edge(a_idx, b_idx);
    }

    Ok(ParsedInput {
        graph,
        start: start.ok_or(anyhow!("Unable to find start node"))?,
        end: end.ok_or(anyhow!("Unable to find end node"))?,
    })
}

fn task_1(input: &ParsedInput) -> Result<u32> {
    let mut graph = input.graph.clone();
    let num_paths = find_paths(&mut graph, input.start);

    Ok(num_paths)
}

fn task_2(input: &ParsedInput) -> Result<u32> {
    let mut graph = input.graph.clone();
    let num_paths = find_paths_visit_one_small_cave_twice(&mut graph, input.start);

    Ok(num_paths)
}

type GraphPath = Vec<NodeIndex>;

fn find_paths(graph: &mut Graph<Cave>, from: NodeIndex) -> u32 {
    let mut num_paths = 0;

    let mut incomplete_paths: Vec<GraphPath> = vec![vec![from]];

    let mut completed_paths: Vec<usize> = vec![];
    let mut new_paths: Vec<GraphPath> = vec![];

    while !incomplete_paths.is_empty() {
        completed_paths.clear();
        new_paths.clear();

        for (path_idx, path) in incomplete_paths.iter_mut().enumerate() {
            // Check if this path is completed
            if let Some(&Cave::End) = graph.node(*path.last().unwrap()) {
                // println!("Found path (total={})", num_paths);
                // print_path(graph, path);
                num_paths += 1;
                completed_paths.push(path_idx);
                continue;
            }

            // Advance this path by 1
            let last_node_idx = *path.last().unwrap();
            let eligible_adjacent_nodes: Vec<NodeIndex> = graph.nodes_adjacent_to(last_node_idx).iter()
                .filter(|&node_idx| {
                    let already_visited = path.contains(node_idx);

                    match (already_visited, graph.node(*node_idx).unwrap()) {
                        (_, Cave::Big { .. }) | (false, Cave::End) | (false, Cave::Small { .. }) => true,
                        _ => false,
                    }
                })
                .copied()
                .collect();

            if eligible_adjacent_nodes.len() == 0 {
                // Unable to advance this path any further - remove
                completed_paths.push(path_idx);
                continue;
            }

            // Branch off into new directions
            for &eligible_adjacent_node in eligible_adjacent_nodes.iter().skip(1) {
                let mut new_path = path.clone();
                new_path.push(eligible_adjacent_node);

                new_paths.push(new_path);
            }

            path.push(eligible_adjacent_nodes[0]);
        }

        incomplete_paths = incomplete_paths.into_iter()
            .enumerate()
            .filter(|(idx, _)| !completed_paths.contains(idx))
            .map(|(_, p)| p)
            .collect();
        incomplete_paths.extend(new_paths.iter().cloned());
    }

    num_paths
}

fn find_paths_visit_one_small_cave_twice(graph: &mut Graph<Cave>, from: NodeIndex) -> u32 {
    let mut num_paths = 0;

    let mut incomplete_paths: Vec<GraphPath> = vec![vec![from]];

    let mut completed_paths: Vec<usize> = vec![];
    let mut new_paths: Vec<GraphPath> = vec![];

    while !incomplete_paths.is_empty() {
        completed_paths.clear();
        new_paths.clear();

        for (path_idx, path) in incomplete_paths.iter_mut().enumerate() {
            // Check if this path is completed
            if let Some(&Cave::End) = graph.node(*path.last().unwrap()) {
                num_paths += 1;
                completed_paths.push(path_idx);
                continue;
            }

            // Advance this path by 1
            let last_node_idx = *path.last().unwrap();
            let eligible_adjacent_nodes: Vec<NodeIndex> = graph.nodes_adjacent_to(last_node_idx).iter()
                .filter(|&node_idx| {
                    let already_visited = path.contains(node_idx);

                    let mut small_caves_count = HashMap::new();
                    let mut small_cave_visited_twice = false;
                    for &n in path.iter() {
                        if let Cave::Small { .. } = graph.node(n).unwrap() {
                            let entry = small_caves_count.entry(n).or_insert(0);
                            *entry += 1;

                            if *entry == 2 {
                                small_cave_visited_twice = true;
                                break;
                            }
                        }
                    }

                    match (graph.node(*node_idx).unwrap(), already_visited, small_cave_visited_twice) {
                        (Cave::Big { .. }, _, _, ) | (Cave::End, false, _) | (Cave::Small { .. }, false, _) | (Cave::Small { .. }, true, false) => true,
                        _ => false,
                    }
                })
                .copied()
                .collect();

            if eligible_adjacent_nodes.len() == 0 {
                // Unable to advance this path any further - remove
                completed_paths.push(path_idx);
                continue;
            }

            // Branch off into new directions
            for &eligible_adjacent_node in eligible_adjacent_nodes.iter().skip(1) {
                let mut new_path = path.clone();
                new_path.push(eligible_adjacent_node);

                new_paths.push(new_path);
            }

            path.push(eligible_adjacent_nodes[0]);
        }

        incomplete_paths = incomplete_paths.into_iter()
            .enumerate()
            .filter(|(idx, _)| !completed_paths.contains(idx))
            .map(|(_, p)| p)
            .collect();
        incomplete_paths.extend(new_paths.iter().cloned());
    }

    num_paths
}


fn print_path(graph: &Graph<Cave>, path: &[NodeIndex]) {
    for &n in path {
        let d = graph.node(n).unwrap();

        match d {
            Cave::Start => print!(" start "),
            Cave::End => print!(" end "),
            Cave::Big { name } => print!(" {} ", name),
            Cave::Small { name } => print!(" {} ", name),
        }

        print!("->");
    }

    println!();
}

type NodeIndex = usize;

#[derive(Debug, Clone)]
struct Graph<N> {
    nodes: Vec<Node<N>>,
    edges: Vec<Vec<NodeIndex>>,
}

#[derive(Debug, Clone)]
struct Node<N> {
    data: N,
}

impl<N> Graph<N> {
    fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }

    fn add_node(&mut self, data: N) -> NodeIndex {
        let idx = self.nodes.len();
        self.nodes.push(Node { data });
        self.edges.push(Vec::new());

        return idx;
    }

    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) {
        self.edges[a].push(b);
        self.edges[b].push(a);
    }

    fn node(&self, idx: NodeIndex) -> Option<&N> {
        self.nodes.get(idx).map(|n| &n.data)
    }

    fn node_mut(&mut self, idx: NodeIndex) -> Option<&mut N> {
        self.nodes.get_mut(idx).map(|n| &mut n.data)
    }

    fn nodes_adjacent_to(&self, idx: NodeIndex) -> &[NodeIndex] {
        &self.edges[idx]
    }
}



