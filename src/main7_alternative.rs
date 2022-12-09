use std::fs::read_to_string;

struct FSDirectory {
    children: Vec<FSNode>,
    size: Option<usize>,
    name: String,
}

impl FSDirectory {
    fn mkdir(&mut self, name: impl ToString) -> &FSNode {
        let res = FSNode::Dir(FSDirectory {
            children: vec![],
            size: None,
            name: name.to_string(),
        });
        self.children.push(res);
        self.children.last().unwrap()
    }

    fn touch(&mut self, name: impl ToString, size: usize) -> &FSNode {
        let res = FSNode::File(FSFile {
            size: size,
            name: name.to_string(),
        });
        self.children.push(res);
        self.children.last().unwrap()
    }

    fn calc_size(&mut self) -> usize {
        let res = self.children.iter_mut().map(|c| c.calc_size()).sum();
        self.size = Some(res);
        res
    }

    fn visit_tree(&self, visitor: VisitorFN) {
        visitor(self.size.unwrap(), self.name.as_str(), "dir");
        for c in self.children.iter() {
            c.visit_tree(visitor);
        }
    }
}

struct FSFile {
    size: usize,
    name: String,
}

enum FSNode {
    Dir(FSDirectory),
    File(FSFile),
}

type VisitorFN<'a> = &'a mut dyn FnMut(usize, &str, &str);

impl FSNode {
    fn mkdir(&mut self, name: impl ToString) -> Option<&FSNode> {
        match self {
            FSNode::Dir(d) => Some(d.mkdir(name)),
            FSNode::File(_) => None,
        }
    }
    fn touch(&mut self, name: impl ToString, size: usize) -> Option<&FSNode> {
        match self {
            FSNode::Dir(d) => Some(d.touch(name, size)),
            FSNode::File(_) => None,
        }
    }
    fn navigate_to_mut<'a>(&'a mut self, path: &'_ [String]) -> Option<&'a mut FSNode> {
        if path.is_empty() {
            return Some(self);
        }
        let next_part = path.first().unwrap();
        match self {
            FSNode::Dir(d) => {
                let next_link = d.children.iter_mut().find(|v| v.name() == next_part);
                next_link.map(|l| l.navigate_to_mut(&path[1..])).flatten()
            }
            FSNode::File(_) => None,
        }
    }

    fn name(&self) -> &str {
        match self {
            FSNode::Dir(d) => d.name.as_str(),
            FSNode::File(f) => f.name.as_str(),
        }
    }

    fn calc_size(&mut self) -> usize {
        match self {
            FSNode::Dir(d) => d.calc_size(),
            FSNode::File(f) => f.size,
        }
    }

    fn visit_tree(&self, visitor: VisitorFN) {
        match self {
            FSNode::Dir(d) => {
                d.visit_tree(visitor);
            }
            FSNode::File(f) => {
                visitor(f.size, f.name.as_str(), "file");
            }
        }
    }
}

fn main() {
    let input = read_to_string("input7a.txt").unwrap();

    let root_fs = FSDirectory {
        children: vec![],
        name: String::from(""),
        size: None,
    };
    let mut root_fs = FSNode::Dir(root_fs);

    let mut cursor = &mut root_fs;
    let mut current_path = vec![];

    for line in input.lines() {
        let mut split_line = line.split_whitespace();

        let first_word = split_line.next().unwrap();
        let num_parse = first_word.parse::<usize>();

        match first_word {
            "$" => {
                // this is a command
                let command = split_line.next().unwrap();
                match command {
                    "cd" => {
                        let path_chunk = split_line.next().unwrap();
                        if path_chunk == ".." {
                            current_path.pop();
                        } else if path_chunk == "/" {
                            current_path.clear();
                        } else {
                            current_path.push(path_chunk.to_string());
                        }
                        cursor = root_fs.navigate_to_mut(&current_path[..]).unwrap();
                    }
                    "ls" => {
                        // nop
                    }
                    _ => {
                        panic!("Unknown command: {command}");
                    }
                }
            }
            "dir" => {
                // directory in ls command output
                let dir_name = split_line.next().unwrap();
                cursor.mkdir(dir_name);
            }
            _ => {
                // file assumed
                let file_size = num_parse.unwrap();
                let file_name = split_line.next().unwrap();
                cursor.touch(file_name, file_size);
            }
        }
    }

    let total_size = root_fs.calc_size();
    let unused_space = 70_000_000 - total_size;
    let min_size_to_free = 30_000_000 - unused_space;
    println!("total size: {total_size}");

    let mut total_size = 0;
    root_fs.visit_tree(&mut |size, _, t| {
        if size <= 100_000 && t == "dir" {
            total_size += size;
        }
    });
    println!("total size of small dirs: {total_size}");
    let mut candidates_vec: Vec<usize> = vec![];
    root_fs.visit_tree(&mut |size, _, t| {
        if size >= min_size_to_free {
            candidates_vec.push(size);
        }
    });
    candidates_vec.sort();
    println!(
        "minimum size to delete: {}",
        candidates_vec.first().unwrap()
    );
}
