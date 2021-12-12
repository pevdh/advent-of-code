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
    let graph = input.graph.clone();
    let num_paths = find_num_paths::<Task1GraphWalker>(&graph, input.start);

    Ok(num_paths)
}

fn task_2(input: &ParsedInput) -> Result<u32> {
    let graph = input.graph.clone();
    let num_paths = find_num_paths::<Task2GraphWalker>(&graph, input.start);

    Ok(num_paths)
}

trait GraphWalker<'graph> {
    type State;

    fn init_at_start(graph: &'graph Graph<Cave>, start: NodeIndex) -> Self::State;
    fn advance(state: Self::State) -> Vec<Self::State>;
    fn is_at_end(state: &Self::State) -> bool;
}

#[derive(Clone)]
struct Task1GraphWalkerState<'graph> {
    graph: &'graph Graph<Cave>,
    visited_small_caves: HashSet<NodeIndex>,
    current_position: NodeIndex,
}

struct Task1GraphWalker {}

impl<'graph> GraphWalker<'graph> for Task1GraphWalker {
    type State = Task1GraphWalkerState<'graph>;

    fn init_at_start(
        graph: &'graph Graph<Cave>,
        start: NodeIndex,
    ) -> Task1GraphWalkerState<'graph> {
        Task1GraphWalkerState {
            graph,
            visited_small_caves: HashSet::new(),
            current_position: start,
        }
    }

    fn advance(state: Task1GraphWalkerState<'graph>) -> Vec<Task1GraphWalkerState<'graph>> {
        let eligible_adjacent_nodes: Vec<NodeIndex> = state
            .graph
            .nodes_adjacent_to(state.current_position)
            .iter()
            .filter(|&node_idx| {
                let already_visited = state.visited_small_caves.contains(node_idx);

                matches!(
                    (state.graph.node(*node_idx), already_visited),
                    (Cave::Big { .. }, _) | (Cave::Small { .. }, false) | (Cave::End, _)
                )
            })
            .copied()
            .collect();

        let mut next_paths = vec![];
        for &eligible_adjacent_node in &eligible_adjacent_nodes {
            let mut visited_small_caves = state.visited_small_caves.clone();
            if let Cave::Small { .. } = state.graph.node(eligible_adjacent_node) {
                visited_small_caves.insert(eligible_adjacent_node);
            }

            next_paths.push(Task1GraphWalkerState {
                graph: state.graph,
                visited_small_caves,
                current_position: eligible_adjacent_node,
            });
        }

        next_paths
    }

    fn is_at_end(state: &Task1GraphWalkerState) -> bool {
        *state.graph.node(state.current_position) == Cave::End
    }
}

#[derive(Clone)]
struct Task2GraphWalkerState<'graph> {
    graph: &'graph Graph<Cave>,
    visited_small_cave_count: HashMap<NodeIndex, u32>,
    small_cave_has_been_visited_twice: bool,
    current_position: NodeIndex,
}

struct Task2GraphWalker {}

impl<'graph> GraphWalker<'graph> for Task2GraphWalker {
    type State = Task2GraphWalkerState<'graph>;

    fn init_at_start(graph: &'graph Graph<Cave>, start: NodeIndex) -> Self::State {
        Self::State {
            graph,
            visited_small_cave_count: HashMap::new(),
            small_cave_has_been_visited_twice: false,
            current_position: start,
        }
    }

    fn advance(state: Self::State) -> Vec<Self::State> {
        let eligible_adjacent_nodes: Vec<NodeIndex> = state
            .graph
            .nodes_adjacent_to(state.current_position)
            .iter()
            .filter(|&node_idx| {
                let already_visited_small_cave =
                    state.visited_small_cave_count.contains_key(node_idx);

                matches!(
                    (
                        state.graph.node(*node_idx),
                        already_visited_small_cave,
                        state.small_cave_has_been_visited_twice
                    ),
                    (Cave::Big { .. }, _, _)
                        | (Cave::Small { .. }, false, _)
                        | (Cave::Small { .. }, true, false)
                        | (Cave::End, _, _)
                )
            })
            .copied()
            .collect();

        let mut next_paths = vec![];
        for &eligible_adjacent_node in &eligible_adjacent_nodes {
            let mut visited_small_cave_count = state.visited_small_cave_count.clone();
            let mut small_cave_has_been_visited_twice = state.small_cave_has_been_visited_twice;

            if let Cave::Small { .. } = state.graph.node(eligible_adjacent_node) {
                let entry = visited_small_cave_count
                    .entry(eligible_adjacent_node)
                    .or_insert(0);
                *entry += 1;

                if *entry == 2 {
                    small_cave_has_been_visited_twice = true;
                }
            }

            next_paths.push(Self::State {
                visited_small_cave_count,
                small_cave_has_been_visited_twice,
                graph: state.graph,
                current_position: eligible_adjacent_node,
            });
        }

        next_paths
    }

    fn is_at_end(state: &Self::State) -> bool {
        *state.graph.node(state.current_position) == Cave::End
    }
}

fn find_num_paths<'graph, Walker: GraphWalker<'graph>>(
    graph: &'graph Graph<Cave>,
    start: NodeIndex,
) -> u32 {
    let mut num_paths = 0;

    let mut states: Vec<Walker::State> = vec![Walker::init_at_start(graph, start)];

    loop {
        let mut new_states = vec![];

        for state in states.into_iter() {
            for new_state in Walker::advance(state) {
                if Walker::is_at_end(&new_state) {
                    num_paths += 1;
                } else {
                    new_states.push(new_state);
                }
            }
        }

        if new_states.is_empty() {
            return num_paths;
        }

        states = new_states;
    }
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
