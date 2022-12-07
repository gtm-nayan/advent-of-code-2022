#![feature(once_cell)]
use std::{
    cell::{OnceCell, RefCell},
    fmt::Debug,
    rc::Rc,
};

type WrappedNode = Rc<RefCell<FsEntry>>;

struct Children {
    inner: Vec<WrappedNode>,
}

impl Debug for Children {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.inner).finish()
    }
}

impl Children {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn add_child(&mut self, node: FsEntry) -> WrappedNode {
        let child = node.wrap();
        self.inner.push(Rc::clone(&child));
        child
    }

    fn get_child(&self, name: impl AsRef<str>) -> Option<WrappedNode> {
        self.inner
            .iter()
            .find(|child| name.as_ref().eq(child.borrow_mut().name()))
            .map(Rc::clone)
    }
}

#[derive(Debug)]
enum FsEntry {
    File {
        name: String,
        size: usize,
    },
    Folder {
        name: String,
        children: Children,
        size: OnceCell<usize>,
    },
}

impl FsEntry {
    pub fn new_file(name: String, size: usize) -> Self {
        Self::File { name, size }
    }

    pub fn new_folder(name: String) -> Self {
        Self::Folder {
            name,
            children: Children::new(),
            size: OnceCell::new(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            FsEntry::File { name, .. } | FsEntry::Folder { name, .. } => name,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            FsEntry::File { size, .. } => *size,
            FsEntry::Folder { children, size, .. } => *size.get_or_init(|| {
                children
                    .inner
                    .iter()
                    .map(|child| child.borrow().size())
                    .sum()
            }),
        }
    }

    pub fn children(&self) -> Option<&Children> {
        match self {
            FsEntry::File { .. } => None,
            FsEntry::Folder { children, .. } => Some(children),
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Children> {
        match self {
            FsEntry::File { .. } => None,
            FsEntry::Folder { children, .. } => Some(children),
        }
    }

    pub fn wrap(self) -> WrappedNode {
        Rc::new(RefCell::new(self))
    }

    /// Returns `true` if the node is [`Folder`].
    ///
    /// [`Folder`]: Node::Folder
    #[must_use]
    fn is_folder(&self) -> bool {
        matches!(self, Self::Folder { .. })
    }
}

enum Line {
    Cd(String),
    Ls,
    Node(FsEntry),
}

fn parse_line(line: &str) -> Line {
    let mut splits = line.split_ascii_whitespace();

    let start = splits.next().expect("Line to be not empty");

    if start == "$" {
        let cmd = splits.next().expect("Command to be not empty");
        match cmd {
            "cd" => {
                let dir = splits.next().expect("cd to have a target");
                Line::Cd(dir.to_string())
            }
            "ls" => Line::Ls,
            _ => unreachable!("Unknown command"),
        }
    } else if start == "dir" {
        let dirname = splits.next().expect("Dirname to not be blank").to_string();
        Line::Node(FsEntry::new_folder(dirname))
    } else if let Ok(size) = start.parse() {
        let filename = splits
            .next()
            .expect("File name to not be blank")
            .to_string();
        Line::Node(FsEntry::new_file(filename, size))
    } else {
        unreachable!()
    }
}

fn to_file_tree(input: &str) -> (WrappedNode, Vec<WrappedNode>) {
    let lines = input.lines().map(parse_line);

    let root = FsEntry::Folder {
        name: "".into(),
        children: Children::new(),
        size: Default::default(),
    }
    .wrap();
    let mut dirs = vec![];
    let mut state = vec![Rc::downgrade(&root)];

    for line in lines {
        match line {
            Line::Cd(target) => match target.as_str() {
                "/" => {
                    state.truncate(1);
                }
                ".." => {
                    if state.len() > 1 {
                        state.pop();
                    }
                }
                name => {
                    let cwd = state.last().expect("State to not be empty");
                    let child = cwd
                        .upgrade()
                        .expect("Node to be in memory")
                        .borrow()
                        .children()
                        .expect("cwd to be a folder")
                        .get_child(name)
                        .expect("Child to exist");
                    assert!(child.borrow().is_folder());
                    state.push(Rc::downgrade(&child));
                }
            },
            Line::Ls => {}
            Line::Node(node) => {
                let cwd = state.last().expect("State to not be empty");
                let is_dir = node.is_folder();
                let child = cwd
                    .upgrade()
                    .expect("Node to be in memory")
                    .borrow_mut()
                    .children_mut()
                    .expect("cwd to be a folder")
                    .add_child(node);
                if is_dir {
                    dirs.push(child);
                }
            }
        }
    }
    (root, dirs)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, subdirs) = to_file_tree(input);
    Some(
        subdirs
            .iter()
            .filter_map(|dir| {
                let size = dir.borrow().size();
                size.lt(&100000).then_some(size as u32)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (root, subdirs) = to_file_tree(input);
    let to_free = 30000000 - (70000000 - root.borrow().size());

    subdirs
        .iter()
        .filter_map(|dir| {
            let size = dir.borrow().size();
            size.gt(&to_free).then_some(size as u32)
        })
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
