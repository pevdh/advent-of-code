use aoc2024::*;

aoc_main!(
    day: 9,
    test_input: r#"
2333133121414131402
"#,
    task_1: task_1,
    expected_1: 1928,
    task_2: task_2,
    expected_2: 2858,
);

fn task_1(input: &str) -> Result<u64> {
    let input: Vec<char> = input.trim().chars().collect_vec();
    let mut disk = Disk::from_disk_map(&input)?;

    disk.compact();

    Ok(disk.checksum())
}

fn task_2(input: &str) -> Result<u64> {
    let input: Vec<char> = input.trim().chars().collect_vec();
    let mut disk = Disk::from_disk_map(&input)?;

    disk.defragment();

    Ok(disk.checksum())
}

#[derive(Debug)]
struct Node {
    id: usize,
    size: usize,
    free: usize,
    next: Option<usize>,
    prev: Option<usize>,
}

struct Disk {
    nodes: Vec<Node>,
}

impl Disk {
    fn from_disk_map(disk_map: &[char]) -> Result<Disk> {
        let mut node_id = 0;
        let mut idx = 0;
        let mut nodes = vec![];
        loop {
            let size = disk_map[idx].to_digit(10).ok_or_parse_error()? as usize;
            let free = disk_map
                .get(idx + 1)
                .unwrap_or(&'0')
                .to_digit(10)
                .ok_or_parse_error()? as usize;

            nodes.push(Node {
                id: node_id,
                size,
                free,
                next: None,
                prev: None,
            });

            if idx > 0 {
                nodes[node_id - 1].next = Some(node_id);
                nodes[node_id].prev = Some(node_id - 1);
            }

            idx += 2;
            node_id += 1;
            if idx > disk_map.len() {
                break;
            }
        }

        Ok(Disk { nodes })
    }

    fn compact(&mut self) {
        let mut free = self.nodes.iter().position(|f| f.free > 0).unwrap();
        let mut node = self.nodes.len() - 1;

        while free != node {
            self.compact_node(free, node);
            if self.nodes[node].size == 0 {
                // remove node from list by unlinking
                self.unlink(node);
                node -= 1;
            }

            // find next free
            while self.nodes[free].free == 0 && free != node {
                free = match self.nodes[free].next {
                    Some(n) => n,
                    None => return, // no free nodes left
                }
            }
        }
    }

    fn compact_node(&mut self, free: usize, curr: usize) {
        assert!(self.nodes[curr].size > 0);

        // insert a new node between free and free.next
        let new_node_size = if self.nodes[curr].size > self.nodes[free].free {
            self.nodes[free].free
        } else {
            self.nodes[curr].size
        };

        let new_node_free = self.nodes[free].free - new_node_size;

        self.nodes.push(Node {
            id: curr,
            size: new_node_size,
            free: new_node_free,
            next: None,
            prev: None,
        });

        let new_node_idx = self.nodes.len() - 1;
        self.insert_after(free, new_node_idx);

        self.nodes[free].free = 0;
        self.nodes[curr].size -= self.nodes[new_node_idx].size;
    }

    fn defragment(&mut self) {
        let mut node = self.nodes.len() - 1;
        while node > 0 {
            let free = match self.find_free(node) {
                Some(n) => n,
                _ => {
                    node -= 1;
                    continue;
                }
            };

            // create new node after free node
            let new_id = self.nodes[node].id;
            let new_size = self.nodes[node].size;
            let new_free = self.nodes[free].free - new_size;

            self.nodes.push(Node {
                id: new_id,
                size: new_size,
                free: new_free,
                next: None,
                prev: None,
            });
            let new_node = self.nodes.len() - 1;
            self.insert_after(free, new_node);

            self.nodes[free].free = 0;

            self.unlink(node);

            node -= 1;
        }
    }

    fn find_free(&mut self, n: usize) -> Option<usize> {
        if n == 0 {
            return None; // cannot relocate first node
        }

        let mut curr = 0;

        loop {
            if self.nodes[curr].free >= self.nodes[n].size {
                return Some(curr);
            }

            if self.nodes[curr].next == Some(n) {
                // Could not find a suitable node before the current node
                // in the list
                return None;
            }

            curr = self.nodes[curr]
                .next
                .expect("reached end of list without encountering node");
        }
    }

    fn insert_after(&mut self, after: usize, n: usize) {
        let next = self.nodes[after].next;

        self.nodes[after].next = Some(n);
        self.nodes[n].prev = Some(after);

        if let Some(next) = next {
            self.nodes[n].next = Some(next);
            self.nodes[next].prev = Some(n);
        }
    }

    fn unlink(&mut self, n: usize) {
        let prev = self.nodes[n].prev.expect("cannot unlink first node"); //  of course we can, but we don't need to for this problem :')

        self.nodes[prev].next = self.nodes[n].next;
        self.nodes[prev].free += self.nodes[n].free + self.nodes[n].size;

        if let Some(next) = self.nodes[n].next {
            self.nodes[next].prev = Some(prev);
        }
    }

    fn checksum(&self) -> u64 {
        let mut curr = 0usize;

        let mut checksum = 0u64;
        let mut block_idx = 0usize;
        loop {
            let size = self.nodes[curr].size;
            for i in block_idx..(block_idx + size) {
                checksum += i as u64 * self.nodes[curr].id as u64;
            }

            block_idx += self.nodes[curr].size + self.nodes[curr].free;
            curr = match self.nodes[curr].next {
                Some(n) => n,
                None => return checksum,
            };
        }
    }
}
