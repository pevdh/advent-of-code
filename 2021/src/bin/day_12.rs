use aoc2021::*;
use ndarray::Array1;

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
    Big,
    Small,
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
        let mut parts = line.split('-');
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
                        Cave::Big
                    } else {
                        Cave::Small
                    })
                };

                node_to_idx.insert(n.to_string(), node_idx);
            }
        }
    }

    for line in raw_input.lines() {
        let mut parts = line.split('-');
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
    fn find_num_paths_dfs(
        graph: &Graph<Cave>,
        cave_visits: Array1<u32>,
        current_position: NodeIndex,
    ) -> u32 {
        if let Cave::End = graph.node(current_position) {
            return 1;
        }

        graph
            .nodes_adjacent_to(current_position)
            .iter()
            .filter(|&node_idx| {
                let cave = graph.node(*node_idx);
                let already_visited = cave_visits[*node_idx] > 0;

                if let Cave::Big = cave {
                    return true;
                }

                if let Cave::End = cave {
                    return true;
                }

                if let Cave::Small = cave {
                    // We can only visit small caves if we haven't visited it yet
                    return !already_visited;
                }

                false
            })
            .map(|&eligible_adjacent_node| {
                let mut cave_visits = cave_visits.clone();
                cave_visits[eligible_adjacent_node] += 1;

                find_num_paths_dfs(graph, cave_visits, eligible_adjacent_node)
            })
            .sum()
    }

    let graph = input.graph.clone();
    let num_paths = find_num_paths_dfs(&graph, Array1::zeros(graph.num_nodes()), input.start);

    Ok(num_paths)
}

fn task_2(input: &ParsedInput) -> Result<u32> {
    fn find_num_paths_dfs(
        graph: &Graph<Cave>,
        has_visited_small_caves_twice: bool,
        cave_visits: Array1<u32>,
        current_position: NodeIndex,
    ) -> u32 {
        if let Cave::End = graph.node(current_position) {
            return 1;
        }

        graph
            .nodes_adjacent_to(current_position)
            .iter()
            .filter(|&node_idx| {
                let cave = graph.node(*node_idx);

                if let Cave::Big = cave {
                    // Big caves are always eligible to be visited again
                    return true;
                }

                if let Cave::End = cave {
                    // We can also always visit the end cave
                    return true;
                }

                if let Cave::Small = cave {
                    // Small caves can only be revisited if we haven't
                    // visited it yet, or we haven't visited any small caves twice yet.
                    let already_visited_cave = cave_visits[*node_idx] > 0;

                    return !already_visited_cave || !has_visited_small_caves_twice;
                }

                false
            })
            .map(|&eligible_adjacent_node| {
                let is_small_cave = *graph.node(eligible_adjacent_node) == Cave::Small;
                let mut cave_visits = cave_visits.clone();
                cave_visits[eligible_adjacent_node] += 1;
                let mut has_visited_small_caves_twice = has_visited_small_caves_twice;

                if cave_visits[eligible_adjacent_node] >= 2 && is_small_cave {
                    has_visited_small_caves_twice = true;
                }

                find_num_paths_dfs(
                    graph,
                    has_visited_small_caves_twice,
                    cave_visits,
                    eligible_adjacent_node,
                )
            })
            .sum()
    }

    let graph = input.graph.clone();
    let num_paths =
        find_num_paths_dfs(&graph, false, Array1::zeros(graph.num_nodes()), input.start);

    Ok(num_paths)
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

    fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn add_node(&mut self, data: N) -> NodeIndex {
        let idx = self.nodes.len();
        self.nodes.push(Node { data });
        self.edges.push(Vec::new());

        idx
    }

    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex) {
        self.edges[a].push(b);
        self.edges[b].push(a);
    }

    fn node(&self, idx: NodeIndex) -> &N {
        &self.nodes[idx].data
    }

    fn nodes_adjacent_to(&self, idx: NodeIndex) -> &[NodeIndex] {
        &self.edges[idx]
    }
}
