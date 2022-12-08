use std::collections::HashMap;

use itertools::Itertools;
use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day07.txt");

const TOTAL_SPACE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;

type DirName = String;
type DirId = usize;

#[derive(Clone, Debug)]
struct Input {
    tree: Tree,
}

#[derive(Clone, Debug)]
struct Tree {
    arena: Arena,
    node: DirId,
}

struct DirectorySizes<'a> {
    sizes: HashMap<DirId, usize>,
    tree: &'a Tree,
}

impl<'a> DirectorySizes<'a> {
    fn new(tree: &'a Tree) -> Self {
        Self {
            sizes: HashMap::new(),
            tree,
        }
    }

    fn calculate(&mut self) -> HashMap<DirId, usize> {
        self.calculate_directory_size(self.tree.node);
        self.sizes.clone()
    }

    fn calculate_directory_size(&mut self, id: DirId) -> usize {
        if let Some(size) = self.sizes.get(&id) {
            return *size;
        }

        let mut size = 0;
        let dir = self.tree.arena.get(id);
        for file_size in dir.files.values() {
            size += file_size;
        }
        for &child_id in dir.children.values() {
            size += self.calculate_directory_size(child_id)
        }

        self.sizes.insert(id, size);

        size
    }
}

impl Tree {
    fn directory_sizes(&self) -> HashMap<DirId, usize> {
        let mut sizes = DirectorySizes::new(self);
        sizes.calculate()
    }
}

#[derive(Clone, Debug, Default)]
/// Using an arena to own the data in our tree allows us to use integers as references
/// This avoids needing to worry about Rust's ownership rules for references
struct Arena {
    data: Vec<Directory>,
}

impl Arena {
    /// Add a new directory to the Arena, returning the new id
    fn add(&mut self, dir: Directory) -> DirId {
        let new_id = self.data.len();
        self.data.push(dir);
        new_id
    }

    /// Get a shared reference to a directory
    fn get(&self, id: DirId) -> &Directory {
        &self.data[id]
    }

    /// Get an exclusive reference to a directory
    fn get_mut(&mut self, id: DirId) -> &mut Directory {
        &mut self.data[id]
    }
}

#[derive(Clone, Debug, Default)]
struct Directory {
    // We need to keep a reference to the parent directory, so we can move up
    parent: Option<DirId>,

    // We need to keep a reference to all child directories, keyed by name
    children: HashMap<DirName, DirId>,

    // The data owned by this directory is the files it contains
    files: HashMap<String, usize>,
}

impl Directory {
    fn new(parent: Option<DirId>) -> Self {
        Self {
            parent,
            children: HashMap::default(),
            files: HashMap::default(),
        }
    }

    fn get_child_id(&self, name: &DirName) -> DirId {
        *self.children.get(name).expect("failed to get child dir id")
    }
}

impl Input {
    fn new(input: &str) -> Input {
        let mut arena = Arena::default();

        // Set up the top level directory
        let directory = Directory::new(None);
        let mut current_dir_id = arena.add(directory);

        // Skip the first line as it is just the top-level directory
        for line in input.lines().dropping(1) {
            if line.starts_with('$') {
                // It is a command
                match line {
                    "$ cd .." => {
                        // Step up to the parent directory
                        current_dir_id = arena
                            .get(current_dir_id)
                            .parent
                            .expect("directory has no parent")
                    }
                    "$ ls" => {
                        // No action, continue
                    }
                    _ => {
                        // Step down to this directory
                        let name: String = scan!("$ cd {}" <- line).unwrap();
                        current_dir_id = arena.get(current_dir_id).get_child_id(&name);
                    }
                }
            } else if line.starts_with('d') {
                // It is a directory
                let name: String = scan!("dir {}" <- line).unwrap();
                let new_directory = Directory::new(Some(current_dir_id));
                let dir_id = arena.add(new_directory);
                arena.get_mut(current_dir_id).children.insert(name, dir_id);
            } else {
                // It is a file, add it to the current directory
                let (size, name): (usize, String) = scan!("{} {}" <- line).unwrap();
                arena.get_mut(current_dir_id).files.insert(name, size);
            }
        }

        let tree = Tree { arena, node: 0 };

        Input { tree }
    }
}

fn part1(input: &Input) -> usize {
    input
        .tree
        .directory_sizes()
        .values()
        .filter(|&&s| s <= 100_000)
        .sum()
}

fn part2(input: &Input) -> usize {
    let sizes = input.tree.directory_sizes();
    let current_space = sizes.get(&0).unwrap();
    let remaining_space = TOTAL_SPACE - current_space;
    let required_to_delete = REQUIRED_SPACE - remaining_space;
    *sizes
        .values()
        .sorted()
        .find_or_first(|&&s| s >= required_to_delete)
        .unwrap()
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day07.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 95437);
        assert_eq!(part2(&input), 24933642);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 1449447);
        assert_eq!(part2(&input), 8679207);
    }
}
