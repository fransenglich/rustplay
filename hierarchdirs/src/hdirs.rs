use std::fs;

pub struct Node {
    pub entry: fs::DirEntry,
    pub children: Vec<Node>,
}

impl Node {
    fn new(entry: fs::DirEntry, children: Vec<Node>) -> Self {
        Node {
            entry: entry,
            children: children,
        }
    }
}

pub fn build_for(start: fs::DirEntry) -> Node {
    let entries: fs::ReadDir = fs::read_dir(start.path().as_path()).unwrap();
    let mut children: Vec<Node> = Vec::new();

    for entry in entries {
        let e: fs::DirEntry = entry.unwrap();

        if e.path().is_dir() {
            children.push(build_for(e));
        }
    }

    Node::new(start, children)
}
