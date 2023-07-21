use std::fmt::Display;
use std::io::{self, BufRead};
use std::iter::{empty, Peekable};
use std::slice::Iter;

pub struct Tree<T> {
    value: T,
    children: Vec<Tree<T>>,
}

pub struct TreeIterator<'a, T> {
    parent_prefix: String,
    immediate_prefix: &'static str,
    parent_suffix: &'static str,
    value: &'a T,
    emitted: bool,
    viter: Box<dyn Iterator<Item = String> + 'a>,
    citer: Box<Peekable<Iter<'a, Tree<T>>>>,
}

impl<'a, T> Iterator for TreeIterator<'a, T> where T: Display {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted && !self.immediate_prefix.is_empty() {
            self.emitted = true;
            Some(format!("{0}{1}{2}", self.parent_prefix, self.immediate_prefix, self.value))
        } else if let Some(val) = self.viter.next() {
            Some(val)
        } else if let Some(child) = self.citer.next() {
            let subprefix = format!("{0}{1}", self.parent_prefix, self.parent_suffix);
            let last = !self.citer.peek().is_some();
            let immediate_prefix = if last { "└─ " } else { "├─ " };
            let parent_suffix = if last { "   " } else { "│  " };
            self.viter = Box::new(child.prefixed_lines(subprefix, immediate_prefix, parent_suffix));
            self.next()
        } else {
            None
        }
    }
}

impl<T> Tree<T> where T: Display {
    pub fn prefixed_lines<'a>(&'a self, parent_prefix: String, immediate_prefix: &'static str, parent_suffix: &'static str) -> TreeIterator<'a, T> {
        TreeIterator {
            parent_prefix: parent_prefix,
            immediate_prefix: immediate_prefix,
            parent_suffix: parent_suffix,
            value: &self.value,
            emitted: false,
            viter: Box::new(empty()),
            citer: Box::new(self.children.iter().peekable()),
        }
    }

    pub fn lines<'a>(&'a self) -> TreeIterator<'a, T> {
        self.prefixed_lines(String::from(""), "", "")
    }
}

fn print_tree<T: Display>(t: &Tree<T>) {
    for line in t.lines() {
        println!("{}", line);
    }
}

fn append_path<'a>(mut t: &mut Tree<&'a str>, path: &'a str) {
    for node in path.split("/") {
        if node.is_empty() {
            continue;
        }

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
