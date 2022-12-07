use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum DirCommand {
    In(String),
    Out,
}

trait FileOrDir {
    fn get_size(&self) -> u32;
}

#[derive(Debug)]
enum Contents {
    File(File),
    Dir(String),
}

impl FromStr for Contents {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut data = s.split_whitespace();
        let val = data.next().unwrap();
        match val.clone().parse::<u32>().ok() {
            Some(size) => Ok(Contents::File(File { size })),
            None => Ok(Contents::Dir(data.next().unwrap().to_string())),
        }
    }
}

impl Contents {
    fn get_size(&self, dir_map: &HashMap<String, Dir>) -> u32 {
        match self {
            Contents::File(f) => f.get_size(),
            Contents::Dir(d) => dir_map
                .get(d)
                .expect("invalid dir")
                .contents
                .iter()
                .map(|f| f.get_size(dir_map))
                .sum(),
        }
    }
}

#[derive(Debug)]
struct File {
    size: u32,
}

impl File {
    fn get_size(&self) -> u32 {
        self.size
    }
}

#[derive(Debug)]
struct Dir {
    contents: Vec<Contents>,
}
impl Dir {
    fn new() -> Dir {
        Dir { contents: vec![] }
    }
    fn add_file(&mut self, file: Contents) {
        self.contents.push(file)
    }
}
trait CustomParse {}
impl FromStr for DirCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<DirCommand> {
        if let Some((prefix, cmd)) = s.split_once(" ") {
            if prefix == "$" {
                let mut cmd = cmd.split_whitespace();
                if cmd.next() == Some("cd") {
                    Ok(match cmd.next() {
                        Some("..") => DirCommand::Out,
                        Some(val) => DirCommand::In(val.to_string()),
                        _ => panic!("i am broken"),
                    })
                } else {
                    Err(anyhow::format_err!("could split round"))
                }
            } else {
                Err(anyhow::format_err!("could split round"))
            }
        } else {
            Err(anyhow::format_err!("could split round"))
        }
    }
}

fn dir_walk() -> Result<u32> {
    let input = aoc::read_one_per_line::<String>("./data/day7.sample")?;
    let mut dirs: HashMap<String, Dir> = HashMap::new();
    let mut cur_dir: String = String::new();
    for line in input.iter().skip(1).filter(|l| !(l.contains("$ ls"))) {
        if line == "" {
            continue;
        }
        //println!("{}", line);
        if let Some(cmd) = line.parse::<DirCommand>().ok() {
            match cmd {
                DirCommand::In(dir_name) => {
                    cur_dir.push_str(&format!("/{}", dir_name));
                    let new_dir = Dir::new();
                    dirs.insert(cur_dir.clone(), new_dir);
                    continue;
                }
                DirCommand::Out => {
                    let idx = cur_dir.rfind("/").expect("dont walk here");
                    cur_dir = String::from(&cur_dir[..idx]);
                    continue;
                }
            };
        } else {
            if !cur_dir.is_empty() {
                let data = line.parse::<Contents>().expect("broken");
                let data = match data {
                    Contents::File(val) => Contents::File(val),
                    Contents::Dir(val) => Contents::Dir(format!("{}/{}", cur_dir, val)),
                };
                dirs.get_mut(&cur_dir).unwrap().add_file(data);
            }
        }
    }
    let mut dir_sizes = HashMap::new();
    for (k, dir) in &dirs {
        let size: u32 = dir.contents.iter().map(|val| val.get_size(&dirs)).sum();
        //.collect_vec();
        dir_sizes.insert(k, size);
    }
    Ok(dir_sizes
        .iter()
        .filter(|(_, size)| **size <= 100000)
        .map(|(_, size)| *size)
        .sum())
}

fn main() -> Result<()> {
    println!("Part 1: {}", dir_walk()?);
    Ok(())
}
