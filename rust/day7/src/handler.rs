use crate::file_system::Directory;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

pub struct Handler {
    root: Rc<RefCell<Directory>>,
    wd: Weak<RefCell<Directory>>,
    ls: bool,
}

impl Handler {
    fn new() -> Self {
        let root = Directory::new(Weak::new(), String::from("/"));
        Self {
            root: Rc::clone(&root),
            wd: Rc::downgrade(&root),
            ls: false,
        }
    }

    pub fn from_file(input: &str) -> Self {
        let mut iter = input.lines();
        if iter.next().expect("empty file") != "$ cd /" {
            panic!("program should start with: $ cd /");
        }
        let mut result = Self::new();
        for line in iter {
            let mut iter_words = line.split_whitespace();
            match (iter_words.next(), iter_words.next(), iter_words.next()) {
                (Some(word), Some(command), argument) if word == "$" => {
                    result.execute(command, argument);
                }
                (Some(word), Some(dir_name), None) if word == "dir" => {
                    result.create_dir(dir_name.to_string());
                }
                (Some(size), Some(file_name), None) => match size.parse::<usize>() {
                    Ok(size) => result.create_file(size, file_name.to_string()),
                    Err(_) => panic!("number expected instead got: {size}, line = {line}"),
                },
                (Some(_), _, _) => panic!("{line} not implemented"),
                (None, _, _) => unreachable!(),
            }
        }
        result
    }

    fn execute(&mut self, command: &str, argument: Option<&str>) {
        if command == "ls" {
            if argument.is_some() {
                panic!("ls doesn't take any argument");
            }
            self.set_ls(true);
            return;
        }
        self.set_ls(false);
        match (command, argument) {
            ("cd", Some(argument)) => self.cd(argument),
            _ => panic!("command {command} not implemented"),
        }
    }

    fn set_ls(&mut self, b: bool) {
        self.ls = b;
    }

    fn cd(&mut self, argument: &str) {
        self.wd = match argument {
            ".." => Rc::downgrade(
                &self
                    .wd
                    .upgrade()
                    .expect("current directory deleted")
                    .borrow()
                    .parent
                    .upgrade()
                    .expect("no parent directory"),
            ),
            "/" => Rc::downgrade(&self.root),
            _ => Rc::downgrade(
                self.wd
                    .upgrade()
                    .expect("current directory deleted")
                    .borrow()
                    .dirs
                    .get(argument)
                    .unwrap_or_else(|| {
                        panic!("no directory named {argument} in current directory")
                    }),
            ),
        }
    }

    pub fn create_dir(&mut self, name: String) {
        if !self.ls {
            panic!("ls command was not executed");
        }
        Directory::create_dir(self.wd.upgrade().expect("current directory deleted"), name);
    }

    pub fn create_file(&mut self, size: usize, name: String) {
        if !self.ls {
            panic!("ls command was not executed");
        }
        Directory::create_file(
            self.wd.upgrade().expect("current directory deleted"),
            name,
            size,
        );
    }

    pub fn get_total_size_not_greater_than(&self, limit: usize) -> usize {
        let mut sizes = Vec::new();
        self.root.borrow().push_overlaped_sizes(&mut sizes);
        sizes.into_iter().filter(|size| size <= &limit).sum()
    }

    pub fn space_freed(&self, total_space: usize, space_needed: usize) -> usize {
        let mut sizes = Vec::new();
        self.root.borrow().push_overlaped_sizes(&mut sizes);
        let consumed_space = self.root.borrow().size;
        sizes.into_iter().filter(|size| {
            let consumed_space = consumed_space - size;
            let available_space = total_space - consumed_space;
            space_needed <= available_space
        }).min().unwrap_or(0)
    }
}

impl Display for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root.borrow())
    }
}
