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

#[derive(Debug, Clone)]
enum Command {
    Cd { directory: String },
    Ls { file_sizes: Vec<usize> },
}

fn parse(raw_input: &str) -> Result<Vec<Command>> {
    let mut file_sizes = vec![];
    let mut in_ls_output = false;

    let mut commands = vec![];

    for line in raw_input.lines() {
        if line.starts_with('$') {
            if in_ls_output {
                commands.push(Command::Ls {
                    file_sizes: file_sizes.clone(),
                });
                file_sizes.clear();
                in_ls_output = false;
            }

            if line.starts_with("$ ls") {
                in_ls_output = true;
            } else if line.starts_with("$ cd") {
                let target = line.split(' ').last().unwrap();
                commands.push(Command::Cd {
                    directory: target.to_owned(),
                });
            }
        } else if !line.starts_with("dir ") {
            let mut spl = line.split(' ');
            let sz = spl.next().unwrap().parse()?;

            file_sizes.push(sz)
        }
    }

    if in_ls_output {
        commands.push(Command::Ls {
            file_sizes: file_sizes.clone(),
        });
    }

    Ok(commands)
}

fn task_1(input: &[Command]) -> Result<usize> {
    let directories = determine_directory_sizes_from(input);

    Ok(directories
        .values()
        .cloned()
        .filter(|sz| *sz <= 100000)
        .sum())
}

fn task_2(input: &[Command]) -> Result<usize> {
    let directories = determine_directory_sizes_from(input);

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

fn determine_directory_sizes_from(commands: &[Command]) -> HashMap<Vec<&str>, usize> {
    let commands = &commands[1..];

    let mut directories = HashMap::new();
    let mut current_path = vec![];

    for command in commands {
        match command {
            Command::Cd { directory } => {
                if directory == ".." {
                    current_path.pop();
                } else {
                    current_path.push(directory.as_str());
                }
            }
            Command::Ls { file_sizes } => {
                let file_sizes = file_sizes.iter().sum();

                directories.insert(current_path.clone(), file_sizes);

                // update parent directory sizes
                let parent_paths = (0..current_path.len()).map(|i| &current_path[..i]);

                for parent_path in parent_paths {
                    directories
                        .entry(parent_path.to_vec())
                        .and_modify(|sz| *sz += file_sizes)
                        .or_insert(0);
                }
            }
        }
    }

    directories
}
