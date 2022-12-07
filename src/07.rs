use anyhow::{anyhow, Error};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

const DISK_SPACE_AVAILABLE: u64 = 70000000;
const DISK_SPACE_REQUIRED: u64 = 30000000;

fn main() -> Result<(), Error> {
    let input = include_str!("07.txt");
    let mut filesystem = Filesystem::from_input(input)?;
    println!("Part 1: {}", filesystem.sum_of_total_sizes_at_most(100000));
    println!(
        "Part 2: {}",
        filesystem.smallest_directory_to_free_up_enough_space(
            DISK_SPACE_AVAILABLE,
            DISK_SPACE_REQUIRED
        )?
    );
    Ok(())
}

#[derive(Debug)]
pub struct Directory {
    entries: HashMap<String, Entry>,
    total_size: Option<u64>,
}

#[derive(Debug)]
struct Filesystem {
    root: Directory,
    shell: Shell,
}

#[derive(Debug)]
struct Shell {
    working_directory: Vec<String>,
}

#[derive(Debug)]
enum Line {
    Command(Command),
    List(List),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum List {
    Directory(String),
    File { size: u64, name: String },
}

#[derive(Debug)]
pub enum Entry {
    Directory(Directory),
    File(u64),
}

impl Filesystem {
    fn from_input(input: &str) -> Result<Filesystem, Error> {
        let mut filesystem = Filesystem::new();
        filesystem.build(input)?;
        Ok(filesystem)
    }

    fn new() -> Filesystem {
        Filesystem {
            root: Directory::new(),
            shell: Shell::new(),
        }
    }

    fn build(&mut self, input: &str) -> Result<(), Error> {
        for line in input.lines() {
            let line: Line = line.parse()?;
            match line {
                Line::Command(command) => match command {
                    Command::Cd(target) => self.cd(target),
                    Command::Ls => (),
                },
                Line::List(list) => match list {
                    List::Directory(name) => {
                        self.add_directory(name)?;
                    }
                    List::File { size, name } => {
                        self.add_file(name, size)?;
                    }
                },
            }
        }
        Ok(())
    }

    fn cd(&mut self, target: String) {
        self.shell.cd(target);
    }

    fn add_directory(&mut self, name: String) -> Result<(), Error> {
        let directory = self.working_directory_mut()?;
        directory.insert(name, Entry::Directory(Directory::new()));
        Ok(())
    }

    fn add_file(&mut self, name: String, size: u64) -> Result<(), Error> {
        let directory = self.working_directory_mut()?;
        directory.insert(name, Entry::File(size));
        Ok(())
    }

    fn working_directory_mut(&mut self) -> Result<&mut Directory, Error> {
        let mut directory = &mut self.root;
        for name in &self.shell.working_directory {
            directory = directory.get_directory_mut(name)?;
        }
        Ok(directory)
    }

    #[cfg(test)]
    fn total_size(&mut self, path: &str) -> Result<u64, Error> {
        let directory = self.directory_mut(path)?;
        Ok(directory.total_size())
    }

    #[cfg(test)]
    fn directory_mut(&mut self, path: &str) -> Result<&mut Directory, Error> {
        let mut directory = &mut self.root;
        if !path.starts_with('/') {
            return Err(anyhow!("path must start with a slash: {}", path));
        } else if path == "/" {
            return Ok(directory);
        }
        for name in path.split('/').skip(1) {
            directory = directory.get_directory_mut(name)?;
        }
        Ok(directory)
    }

    fn sum_of_total_sizes_at_most(&mut self, at_most: u64) -> u64 {
        self.root.sum_of_total_sizes_at_most(at_most)
    }

    fn smallest_directory_to_free_up_enough_space(
        &mut self,
        disk_space_available: u64,
        disk_space_required: u64,
    ) -> Result<u64, Error> {
        let used = self.root.total_size();
        let min_size = disk_space_required - (disk_space_available - used);
        self.root
            .smallest_directory_at_least(min_size)
            .ok_or_else(|| anyhow!("no directories at least {} big", min_size))
    }
}

impl Shell {
    fn new() -> Shell {
        Shell {
            working_directory: Vec::new(),
        }
    }

    fn cd(&mut self, target: String) {
        match target.as_str() {
            ".." => {
                let _ = self.working_directory.pop();
            }
            "/" => self.working_directory.clear(),
            _ => self.working_directory.push(target),
        }
    }
}

impl Directory {
    fn new() -> Directory {
        Directory {
            entries: HashMap::new(),
            total_size: None,
        }
    }

    fn total_size(&mut self) -> u64 {
        if let Some(total_size) = self.total_size {
            return total_size;
        }
        let mut total_size = 0;
        for value in self.entries.values_mut() {
            match value {
                Entry::Directory(directory) => total_size += directory.total_size(),
                Entry::File(size) => total_size += *size,
            }
        }
        self.total_size = Some(total_size);
        total_size
    }

    fn get_directory_mut(&mut self, name: &str) -> Result<&mut Directory, Error> {
        match self
            .entries
            .get_mut(name)
            .ok_or_else(|| anyhow!("directory not in filesystem: {}", name))?
        {
            Entry::Directory(d) => Ok(d),
            Entry::File(_) => {
                return Err(anyhow!(
                    "working directory has a file on its path: {}",
                    name
                ))
            }
        }
    }

    fn sum_of_total_sizes_at_most(&mut self, at_most: u64) -> u64 {
        let mut sum = 0;
        for entry in self.entries.values_mut() {
            match entry {
                Entry::Directory(directory) => {
                    let total_size = directory.total_size();
                    if total_size <= at_most {
                        sum += total_size;
                    }
                    sum += directory.sum_of_total_sizes_at_most(at_most);
                }
                Entry::File(_) => (),
            }
        }
        sum
    }

    fn smallest_directory_at_least(&mut self, at_least: u64) -> Option<u64> {
        let mut min_size = None;
        for entry in self.entries.values_mut() {
            if let Entry::Directory(directory) = entry {
                if let Some(size) = directory.smallest_directory_at_least(at_least) {
                    min_size = Some(size)
                }
                let total_size = directory.total_size();
                if total_size >= at_least && min_size.map(|n| n > total_size).unwrap_or(true) {
                    min_size = Some(total_size)
                }
            }
        }
        min_size
    }
}

impl Deref for Directory {
    type Target = HashMap<String, Entry>;
    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for Directory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

impl FromStr for Line {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ") {
            let (_, command) = s.split_at(1);
            let command: Command = command.parse()?;
            Ok(Line::Command(command))
        } else {
            let list = s.parse::<List>()?;
            Ok(Line::List(list))
        }
    }
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let command = iter
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))?;
        match command {
            "cd" => {
                let target = iter.next().ok_or_else(|| anyhow!("no target for cd"))?;
                Ok(Command::Cd(target.to_string()))
            }
            "ls" => Ok(Command::Ls),
            _ => Err(anyhow!("unexpected command: {}", command)),
        }
    }
}

impl FromStr for List {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let first = iter
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))?;
        let name = iter.next().ok_or_else(|| anyhow!("no name in list line"))?;
        if first == "dir" {
            Ok(List::Directory(name.to_string()))
        } else {
            Ok(List::File {
                size: first.parse()?,
                name: name.to_string(),
            })
        }
    }
}

#[test]
fn part_1() {
    let input = "$ cd /
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
7214296 k";
    let mut filesystem = Filesystem::from_input(input).unwrap();
    assert_eq!(filesystem.total_size("/a/e").unwrap(), 584);
    assert_eq!(filesystem.total_size("/a").unwrap(), 94853);
    assert_eq!(filesystem.total_size("/d").unwrap(), 24933642);
    assert_eq!(filesystem.total_size("/").unwrap(), 48381165);
    assert_eq!(filesystem.sum_of_total_sizes_at_most(100000), 95437);
}

#[test]
fn part_2() {
    let input = "$ cd /
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
7214296 k";
    let mut filesystem = Filesystem::from_input(input).unwrap();
    let size = filesystem
        .smallest_directory_to_free_up_enough_space(DISK_SPACE_AVAILABLE, DISK_SPACE_REQUIRED)
        .unwrap();
    assert_eq!(size, 24933642);
}
