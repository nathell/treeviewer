use std::fmt::Display;
use std::io::{self, BufRead};

pub struct Tree<T> {
    value: T,
    children: Vec<Tree<T>>,
}

fn print_tree_with_prefix<T: Display>(prefix: &str, prefix1: &str, prefix2: &str, t: &Tree<T>) {
    println!("{0}{1}{2}", prefix, prefix1, t.value);
    let mut it = t.children.iter().peekable();

    let subprefix = format!("{0}{1}", prefix, prefix2);

    while let Some(child) = it.next() {
        match it.peek() {
            None => print_tree_with_prefix(&subprefix, "└─ ", "   ", child),
            Some(_) => print_tree_with_prefix(&subprefix, "├─ ", "│  ", child),
        }
    }
}

fn print_tree<T: Display>(t: &Tree<T>) {
    print_tree_with_prefix("", "", "", t);
}

fn append_path<'a>(mut t: &mut Tree<&'a str>, path: &'a str) {
    for node in path.split("/") {
        let match_last = match t.children.last() {
            None => false,
            Some(x) => x.value == node
        };

        if match_last {
            t = t.children.last_mut().unwrap();
        } else {
            let subtree = Tree { value: node, children: vec![] };
            t.children.push(subtree);
            t = t.children.last_mut().unwrap();
        }
    }
}

fn main() {
    let mut t = Tree {value: "", children: vec![]};
    let mut input: Vec<String> = vec![];
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }

    for line in input.iter() {
        append_path(&mut t, line);
    }

    print_tree(&t);
}

#[cfg(test)]
mod tests {
}
