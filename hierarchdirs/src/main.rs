use std::env;
use std::fs;

mod hdirs;

fn print_node(node: hdirs::Node, depth: usize) {
    let binding = node.entry.path();
    let path = binding.as_path();

    let filename = String::from(path.file_name().unwrap().to_str().unwrap());
    let indent: String = String::from("   ");

    let mut final_string: String = String::with_capacity(filename.len() + indent.len() * depth);

    for _i in 1..depth {
        final_string = final_string + &indent;
    }

    final_string = final_string + &filename;

    println!("{}", final_string);

    for child in node.children {
        print_node(child, depth + 1);
    }
}

fn main() {
    let mut current_dir = env::current_dir().unwrap();

    // Get rid of the filename so we have a directory.
    current_dir.pop();

    let start: fs::DirEntry = fs::read_dir(current_dir).unwrap().next().unwrap().unwrap();

    let _root = hdirs::build_for(start);

    print_node(_root, 0);
}
