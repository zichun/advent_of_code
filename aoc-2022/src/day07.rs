use itertools::Itertools;
use std::iter::Iterator;
use std::collections::HashMap;
use std::collections::hash_map::Entry::Vacant;

#[derive(Default, Debug)]
struct Dir {
    name: String,
    parent: usize,
    subdirs: HashMap<String, usize>,
    files: Vec<FsObject>
}

impl Dir {
    fn has_subdir(&self, dir_name: &str) -> bool{
        self.subdirs.contains_key(dir_name)
    }
    fn get_subdir(&self, dir_name: &str) -> Option<&usize> {
        self.subdirs.get(dir_name)
    }
    fn add_subdir_index(&mut self, dir_name: &str, dir_index: usize) {
        self.subdirs.insert(dir_name.to_owned(), dir_index);
    }
    fn add_file(&mut self, file: FsObject) {
        self.files.push(file);
    }
    fn files_size(&self) -> u32 {
        self.files.iter().map(|fs| {
            match fs {
                FsObject::File(_, size) => *size,
                FsObject::Dir(_) => 0,
            }
        }).sum::<u32>()
    }
}
#[derive(Default, Debug)]
struct FileSystem {
    dirs: Vec<Dir>,
    at: usize,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            dirs: vec![Dir::default()],
            at: 0
        }
    }
    fn cd(&mut self, cd: &CdCommand) {
        self.at = match cd {
            CdCommand::Root => 0,
            CdCommand::Parent => self.dirs[self.at].parent,
            CdCommand::Subdir(d) =>
                *self.dirs[self.at].get_subdir(d).unwrap()
        };
    }
    fn new_dir(&mut self, dir_name: String) {
        let ind = self.dirs.len();
        if !self.dirs[self.at].has_subdir(&dir_name) {
            self.dirs.push(Dir {
                name: dir_name.clone(),
                parent: self.at,
                ..Dir::default()
            });
            self.dirs[self.at].add_subdir_index(&dir_name, ind);
        }
    }
    fn add_object(&mut self, fs_object: &FsObject) {
        match fs_object {
            FsObject::Dir(d) => self.new_dir(d.to_owned()),
            file => self.dirs[self.at].add_file(file.to_owned()),
        }
    }
}

#[derive(Debug)]
enum CdCommand {
    Root,
    Parent,
    Subdir(String),
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(CdCommand),
}

#[derive(Clone, Debug)]
enum FsObject {
    File(String, u32),
    Dir(String),
}

fn parse(input: &str) -> FileSystem {

    fn parse_cmd(cmd: &str) -> Command {
        let mut cmd_iter = cmd.split(" ").skip(1);
        match cmd_iter.next().unwrap() {
            "ls" => Command::Ls,
            "cd" => {
                match cmd_iter.next().unwrap() {
                    "/" => Command::Cd(CdCommand::Root),
                    ".." => Command::Cd(CdCommand::Parent),
                    dir => Command::Cd(CdCommand::Subdir(dir.to_owned())),
                }
            },
            _ => panic!("Unrecognized command"),
        }
    }

    let fs = FileSystem::new();
    input.lines()
        .batching(|iter| {
            let cmd = parse_cmd(iter.next()?);

            match cmd {
                Command::Ls => {
                    let objs = iter.take_while_ref(|l| {
                        if l.starts_with("$") {
                            false
                        } else {
                            true
                        }
                    }).map(|l| {
                        let mut line_iter = l.split(" ");
                        match line_iter.next().unwrap() {
                            "dir" => FsObject::Dir(line_iter.next().unwrap().to_owned()),
                            filesize => FsObject::File(
                                line_iter.next().unwrap().to_owned(),
                                filesize.parse::<u32>().unwrap()),
                        }
                    }).collect::<Vec<_>>();

                    Some((Command::Ls, objs))
                },
                cmd => Some((cmd, vec![])),
            }
        }).fold(fs, |mut fs, (cmd, files)| {
            match cmd {
                Command::Ls => files.iter().for_each(|file| fs.add_object(file)),
                Command::Cd(cd) => fs.cd(&cd),
            };
            fs
        })
}

pub fn part2(input: &str) -> u32 {
    const TOTALSIZE: u32 = 70000000;
    const REQUIRED: u32 = 30000000;

    let fs = parse(input);
    let mut dirs = Vec::new();

    fn recur(fs: &FileSystem, ind: usize, dirs: &mut Vec<u32>) -> u32 {
        let dir = &fs.dirs[ind];
        let mut size = dir.files_size();
        dir.subdirs.iter().for_each(|(_, child_ind)|
            size += recur(fs, *child_ind, dirs)
        );
        dirs.push(size);
        size
    }

    let used = recur(&fs, 0, &mut dirs);
    dirs.sort();
    *dirs.iter().find(|d| TOTALSIZE - used + **d >= REQUIRED).unwrap()
}

pub fn part1(input: &str) -> u32 {
    let fs = parse(input);
    let mut tr = 0;

    fn recur(fs: &FileSystem, ind: usize, tr: &mut u32) -> u32 {
        let dir = &fs.dirs[ind];
        let mut size = dir.files_size();
        dir.subdirs.iter().for_each(|(_, child_ind)|
            size += recur(fs, *child_ind, tr)
        );
        if size < 100000 {
            *tr += size;
        }
        size
    }

    recur(&fs, 0, &mut tr);
    tr
}

#[test]
fn test() {
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
    assert_eq!(part1(input), 95437);
    assert_eq!(part2(input), 24933642);
}
