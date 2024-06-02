use std::collections::HashSet;
use std::fmt::Display;
use std::io::{self, BufRead};
use std::iter::{empty, Peekable};
use std::slice::Iter;

use cursive::{
    Cursive,
    view::{Resizable, Scrollable},
    views::SelectView
};

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
    collapsed: &'a HashSet<Vec<i32>>,
    current: Vec<i32>,
}

pub struct State {
    tree: Tree<String>,
    collapsed: HashSet<Vec<i32>>,
}

fn append_path<'a>(mut t: &'a mut Tree<String>, path: &'a str) {
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
            let subtree = Tree { value: String::from(node), children: vec![] };
            t.children.push(subtree);
            t = t.children.last_mut().unwrap();
        }
    }
}

fn init_state() -> State {
    let mut t = Tree {value: String::from(""), children: vec![]};
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        append_path(&mut t, &line.unwrap());
    }

    let hs = HashSet::from([vec![0, 1, 1]]);

    State { tree: t, collapsed: hs /* HashSet::new() */ }
}

impl<'a, T> Iterator for TreeIterator<'a, T> where T: Display {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.emitted && !self.immediate_prefix.is_empty() {
            self.emitted = true;
            let val = format!("{}{}{}", self.parent_prefix, self.immediate_prefix, self.value);
            self.current.push(0);
            Some(val)
        } else if let Some(val) = self.viter.next() {
            let val = format!("{}", val);
            Some(val)
        } else if let Some(child) = self.citer.next() {
            let node_collapsed = self.collapsed.contains(&self.current);
            let subprefix = format!("{0}{1}", self.parent_prefix, self.parent_suffix);
            let last = !self.citer.peek().is_some();
            let immediate_prefix = if last { "└─ " } else { "├─ " };
            let parent_suffix = if last { "   " } else { "│  " };
            let val = if !node_collapsed {
                self.viter = Box::new(child.prefixed_lines(subprefix, immediate_prefix, parent_suffix, &self.collapsed, self.current.clone()));
                self.next()
            } else {
                let immediate_prefix = if last { "└⊞ " } else { "├⊞ " };
                Some(format!("{}{}{}", subprefix, immediate_prefix, child.value))
            };
            *self.current.last_mut().unwrap() += 1;
            val
        } else {
            None
        }
    }
}

impl<T> Tree<T> where T: Display {
    pub fn prefixed_lines<'a>(&'a self, parent_prefix: String, immediate_prefix: &'static str, parent_suffix: &'static str, collapsed: &'a HashSet<Vec<i32>>, current: Vec<i32>) -> TreeIterator<'a, T> {
        TreeIterator {
            parent_prefix: parent_prefix,
            immediate_prefix: immediate_prefix,
            parent_suffix: parent_suffix,
            value: &self.value,
            emitted: false,
            viter: Box::new(empty()),
            citer: Box::new(self.children.iter().peekable()),
            collapsed: collapsed,
            current: current,
        }
    }
}

impl State {
    pub fn lines<'a>(&'a self) -> TreeIterator<'a, String> {
        self.tree.prefixed_lines(String::from(""), "", "", &self.collapsed, vec![0])
    }
}

fn main() {
    let state = init_state();
    let mut siv = cursive::default();
    let mut select = SelectView::new();
    select.add_all_str(state.lines());

    siv.add_fullscreen_layer(select.scrollable().full_screen());
    siv.add_global_callback('q', Cursive::quit);
    siv.run();
}

#[cfg(test)]
mod tests {
}
