use aoc2022::*;

aoc_main!(
    day: 7,
    test_input:
    r#"
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    "#,
    parser: parse,
    task_1: task_1,
    expected_1: 95437,
    task_2: task_2,
    expected_2: 24933642,
);

fn parse(raw_input: &str) -> Result<String> {
    Ok(raw_input.to_owned())
}

fn task_1(input: &str) -> Result<usize> {
    let directories = determine_directory_sizes_from_terminal_output(input);

    Ok(directories
        .values()
        .cloned()
        .filter(|sz| *sz <= 100000)
        .sum())
}

fn task_2(input: &str) -> Result<usize> {
    let directories = determine_directory_sizes_from_terminal_output(input);

    let disk_space_available = 70000000usize;
    let required_unused_space = 30000000usize;

    let used_space = directories.get(&vec![]).unwrap();
    let current_unused_space = disk_space_available - used_space;
    let min_directory_size_needed = required_unused_space - current_unused_space;

    directories
        .values()
        .cloned()
        .filter(|sz| *sz >= min_directory_size_needed)
        .min_by_key(|sz| *sz - min_directory_size_needed)
        .ok_or_else(|| anyhow!("No solution"))
}

fn determine_directory_sizes_from_terminal_output(
    terminal_output: &str,
) -> HashMap<Vec<&str>, usize> {
    let lines = terminal_output.lines();

    let mut directories = HashMap::default();
    let mut path_components = vec![];

    for line in lines {
        if line == "$ cd /" {
            continue;
        }

        if line.starts_with("$ cd") {
            let target = line.split(' ').last().unwrap();
            if target == ".." {
                path_components.pop();
            } else {
                path_components.push(target);
            }

            continue;
        }

        if line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("dir ") {
            continue;
        }

        let mut spl = line.split(' ');
        let size: usize = spl.next().unwrap().parse().unwrap();

        directories
            .entry(path_components.clone())
            .and_modify(|sz| *sz += size)
            .or_insert(size);

        // update parent directory sizes
        for parent_path_components in (0..path_components.len()).map(|i| &path_components[..i]) {
            directories
                .entry(parent_path_components.to_vec())
                .and_modify(|sz| *sz += size)
                .or_insert(size);
        }
    }

    directories
}
