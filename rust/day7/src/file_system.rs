use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::{Rc, Weak};

pub struct File {
    name: String,
    size: usize,
}

pub struct Directory {
    pub parent: Weak<RefCell<Directory>>,
    pub name: String,
    pub files: Vec<File>,
    pub dirs: HashMap<String, Rc<RefCell<Directory>>>,
    pub size: usize,
}

impl Directory {
    pub fn new(parent: Weak<RefCell<Directory>>, name: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent,
            name,
            files: Vec::new(),
            dirs: HashMap::new(),
            size: 0,
        }))
    }
    pub fn create_dir(self_: Rc<RefCell<Directory>>, name: String) {
        self_
            .borrow_mut()
            .dirs
            .insert(name.clone(), Self::new(Rc::downgrade(&self_), name));
    }

    pub fn create_file(self_: Rc<RefCell<Directory>>, name: String, size: usize) {
        self_.borrow_mut().files.push(File { name, size });
        Self::update_size(self_, size);
    }

    fn update_size(self_: Rc<RefCell<Directory>>, size: usize) {
        let mut self_ = self_.borrow_mut();
        self_.size += size;
        if let Some(parent) = self_.parent.upgrade() {
            Self::update_size(parent, size);
        }
    }

    pub fn push_overlaped_sizes(&self, sizes: &mut Vec<usize>) {
        sizes.push(self.size);
        for dir in self.dirs.values() {
            dir.borrow().push_overlaped_sizes(sizes);
        }
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("indentation is not ok");
        writeln!(f, "- {} dir", self.name)?;
        for dir in self.dirs.values() {
            write!(f, "\t{}", dir.borrow())?;
        }
        for file in &self.files {
            writeln!(
                f,
                "- {} (file, size={}m dir={})",
                file.name, file.size, self.name
            )?;
        }
        Ok(())
    }
}
