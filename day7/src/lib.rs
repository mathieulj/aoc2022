use std::{borrow::Cow, collections::HashMap, iter};

use anyhow::{bail, Context};
use itertools::Itertools;

type Directory<'a> = HashMap<Cow<'a, str>, INode<'a>>;

enum INode<'a> {
    File(usize),
    Directory(Directory<'a>),
}

impl<'a> INode<'a> {
    fn get_mut_dir<I, P>(&mut self, mut path: I) -> Option<&mut Directory<'a>>
    where
        I: Iterator<Item = P>,
        P: AsRef<str>,
    {
        match (self, path.next()) {
            (Self::File(..), ..) => None,
            (Self::Directory(ref mut dir), None) => Some(dir),
            (Self::Directory(dir), Some(segment)) => {
                let child = dir.get_mut(segment.as_ref())?;
                child.get_mut_dir(path)
            }
        }
    }

    fn total_size(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Directory(dirs) => dirs.iter().map(|(_, node)| node.total_size()).sum(),
        }
    }

    fn recurse_dirs(&self) -> Box<dyn '_ + Iterator<Item = &'_ INode<'a>>> {
        let INode::Directory(dirs) = self else {
            return Box::new(std::iter::empty());
        };
        Box::new(iter::once(self).chain(dirs.values().flat_map(INode::recurse_dirs)))
    }

    fn from_cli_history(input: &'a str) -> anyhow::Result<Self> {
        let mut lines = input.lines().peekable();
        let mut root = INode::Directory(Default::default());
        let mut path: Vec<&str> = Vec::new();

        while let Some(line) = lines.next() {
            match line {
                "$ cd /" => path.clear(),
                "$ cd .." => {
                    path.pop();
                }
                "$ ls" => {
                    let dir = root.get_mut_dir(path.iter()).context("Non existant path")?;
                    for line in lines.peeking_take_while(|line| !line.starts_with('$')) {
                        if line.starts_with("dir ") {
                            let (_, name) = line.split_at(4);
                            dir.insert(Cow::Borrowed(name), INode::Directory(Default::default()));
                        } else if let Some((size, name)) = line.split_once(' ') {
                            let size: usize = size.parse().context("Invalid file size")?;
                            dir.insert(Cow::Borrowed(name), INode::File(size));
                        } else {
                            bail!("Unexpected command output {line}");
                        }
                    }
                }
                line if line.starts_with("$ cd ") => {
                    let (_, name) = line.split_at(5);
                    path.push(name);
                }
                _ => {
                    bail!("Unexpected line format {line}");
                }
            }
        }

        Ok(root)
    }
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    Ok(INode::from_cli_history(input)?
        .recurse_dirs()
        .map(|dir| dir.total_size())
        .filter(|size| *size < 100000)
        .sum())
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    let root = INode::from_cli_history(input)?;

    let free = 70000000 - root.total_size();
    let min = 30000000 - free;

    root.recurse_dirs()
        .map(|dir| dir.total_size())
        .filter(|size| *size >= min)
        .min()
        .context("No directory big enough")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");
    const SAMPLE: &str = indoc::indoc! {"
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
    "};

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, 95437),
            (INPUT, 1084134),
        ];

        for (input, output) in expected {
            assert_eq!(crate::challenge1(input)?, output, "For input {}", input)
        }
        Ok(())
    }

    #[test]
    fn test_challenge2() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, 24933642),
            (INPUT, 6183184),
        ];

        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }

        assert!(crate::challenge2(INPUT)? < 25622272);
        Ok(())
    }
}
