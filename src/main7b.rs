use std::{fs::read_to_string, rc::{Rc, Weak}, cell::RefCell};

enum FSNode {
    Directory {
        children: Vec<Rc<RefCell<FSNode>>>,
        size: Option<usize>,
        name: String,
        parent: Option<Weak<RefCell<FSNode>>>,
    },
    File {
        size: usize,
        name: String,
        parent: Weak<RefCell<FSNode>>,
    }
}

impl FSNode {
    fn new_dir(name: impl ToString, parent: Option<Rc<RefCell<FSNode>>>) -> Rc<RefCell<Self>> {
        let mut res = FSNode::Directory { children: vec![], size: None, name: name.to_string(), parent: None };
        if let Some(p) = parent {
            match &mut res {
                FSNode::Directory { children, size, name, parent } => {
                    *parent = Some(Rc::downgrade(&p));
                },
                _ => todo!(),
            }
        }
        Rc::new(RefCell::new(res))
    }

    fn mkdir(&mut self, name: impl ToString, parent: Option<Rc<RefCell<FSNode>>>) -> Rc<RefCell<Self>> {
        let res = Self::new_dir(name, parent);
        if let FSNode::Directory { children, size, name, parent } = self {
            children.push(res.clone());
        }
        res
    }

    fn touch(&mut self, name: impl ToString, size: usize, parent: Rc<RefCell<FSNode>>) -> Rc<RefCell<Self>> {
        let res = Rc::new(RefCell::new(FSNode::File { size: size, name: name.to_string(), parent: Rc::downgrade(&parent) }));
        if let FSNode::Directory { children, size, name, parent } = self {
            children.push(res.clone());
        }
        res
    }

    fn name(&self) -> String {
        match self {
            FSNode::Directory { children, size, name, parent } => name,
            FSNode::File { size, name, parent } => name,
        }.clone()
    }

    fn parent(&self) -> Rc<RefCell<Self>> {
        match self {
            FSNode::Directory { children, size, name, parent } => {
                parent.clone().unwrap().upgrade().unwrap()
            },
            FSNode::File { size, name, parent } => {
                parent.upgrade().unwrap()
            },
        }
    }

    fn calc_sizes(&mut self) -> usize {
        match self {
            FSNode::Directory { children, size, name, parent } => {
                let mut total_size = 0;
                for c in children {
                    let mut c = c.borrow_mut();
                    total_size += c.calc_sizes();
                }
                *size = Some(total_size);
                total_size
            },
            FSNode::File { size, name, parent } => *size,
        }
    }

    fn visit_dirs(&self, f: &mut Vec<usize>, min_size: usize) {
        match self {
            FSNode::Directory { children, size, name, parent } => {
                // f(name.as_str(), size.unwrap());
                let size = size.unwrap();
                if size >= min_size {
                    f.push(size);
                }
                for c in children {
                    let child = c.borrow();
                    child.visit_dirs(f, min_size);
                }
            },
            FSNode::File { size, name, parent } => {},
        }
    }
}

fn main() {
    let mut input = read_to_string("input7a.txt").unwrap();

    let mut fs_root = FSNode::new_dir("/", None);
    let mut current_dir = fs_root.clone();

    for line in input.lines() {
        let mut split_iter = line.split_whitespace();
        let first_part = split_iter.next().unwrap();
        let size_parse = first_part.parse::<usize>().ok();
        match first_part {
            "$" => {
                // this is a command
                let part = split_iter.next().unwrap();
                match part {
                    "cd" => {
                        let my_name = split_iter.next().unwrap();
                        let temp = current_dir.clone();
                        // change current dir
                        let pwd = temp.borrow();
                        let mut cd_changed = false;
                        if my_name == ".." {
                            // special case to go up a level
                            current_dir = pwd.parent();
                            cd_changed = true;
                        } else if my_name == "/" {
                            current_dir = fs_root.clone();
                            cd_changed = true;
                        }
                        else if let FSNode::Directory { children, size, name, parent } = &*pwd {
                            for c in children {
                                let child = c.borrow();
                                if child.name() == my_name {
                                    current_dir = c.clone();
                                    cd_changed = true;
                                    break;
                                }
                            }
                        }
                        if !cd_changed {
                            println!("cannot change into directory: {}", my_name);
                        }
                    },
                    "ls" => {
                        // noop
                    },
                    _ => {
                        panic!("unknown command: {part}")
                    }
                }
            },
            "dir" => {
                // create a directory inside current dir
                let name = split_iter.next().unwrap();
                let temp = current_dir.clone();
                let mut pwd = temp.borrow_mut();
                pwd.mkdir(name, Some(temp.clone()));
            },
            _ => {
                let size_parse = size_parse.unwrap();
                let name = split_iter.next().unwrap();
                let temp = current_dir.clone();
                let mut pwd = temp.borrow_mut();
                pwd.touch(name, size_parse, temp.clone());
            }
        }
    }
    let total_size = fs_root.borrow_mut().calc_sizes();
    let unused_space = 70000000 - total_size;
    let min_size_to_free = 30000000 - unused_space;
    println!("total size: {total_size}");

    let mut size_sum = 0;
    let mut candidates_vec: Vec<usize> = vec![];
    fs_root.borrow().visit_dirs(&mut candidates_vec, min_size_to_free);
    candidates_vec.sort();
    println!("sum of sizes: {size_sum}");
    println!("smallest candidate: {}", candidates_vec.first().unwrap());
}