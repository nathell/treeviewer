use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::iter::{empty, Peekable};
use std::slice::Iter;
use clap::Parser;

use cursive::{
    Cursive,
    event::Key,
    view::{Nameable, Resizable, Scrollable},
    views::{OnEventView, SelectView}
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

fn append_path<'a>(mut t: &'a mut Tree<String>, path: &'a str, separator: &'a str) {
    for node in path.split(separator) {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn init_state(path: &Path, separator: &str) -> State {
    let mut t = Tree {value: String::from(""), children: vec![]};

    for line in read_lines(path).unwrap().flatten() {
        append_path(&mut t, &line, separator);
    }

    State { tree: t, collapsed: HashSet::new() }
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

impl<T> Tree<T> {
    pub fn count(&self, coords: &mut Vec<i32>, collapsed: &HashSet<Vec<i32>>) -> usize {
        if collapsed.contains(coords) {
            return 1;
        }
        let mut res = 1;
        coords.push(0);
        for child in &self.children {
            res += child.count(coords, collapsed);
            *coords.last_mut().unwrap() += 1;
        }
        coords.pop();
        res
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

    fn recursive_coords(&self, tree: &Tree<String>, mut i: usize, mut coords: Vec<i32>) -> Vec<i32> {
        if i == 0 {
            coords
        } else {
            i -= 1;
            let mut next_child: Option<&Tree<String>> = None;
            coords.push(0);
            for child in &tree.children {
                next_child = Some(child);
                let cnt = child.count(&mut coords, &self.collapsed);
                if i >= cnt {
                    i -= cnt;
                    *coords.last_mut().unwrap() += 1;
                } else {
                    break;
                }
            }
            match next_child {
                Some(ch) => {
                    self.recursive_coords(ch, i, coords)
                },
                None => coords
            }
        }
    }

    pub fn coords(&self, i: usize) -> Vec<i32> {
        self.recursive_coords(&self.tree, i, vec![])
    }
}

fn redraw(siv: &mut Cursive) {
    // TODO: collect() feels excessive, but I can't beat the borrow checker without it
    let lines: Vec<String> = siv.with_user_data(|state: &mut State| state.lines().collect()).unwrap();
    siv.call_on_name("select", |select: &mut SelectView| {
        let id = select.selected_id();
        select.clear();
        select.add_all_str(lines);
        if let Some(id) = id {
            select.set_selection(id);
        }
    });
}

/// Reads a list of slash-separated paths and displays them as a tree.
#[derive(Parser)]
struct Cli {
    /// Separator of nodes in paths
    #[arg(short, long, default_value_t = String::from("/"))]
    separator: String,

    /// The file to display
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let state = init_state(&args.file, &args.separator);
    let mut siv = cursive::default();
    let mut select = SelectView::new().with_name("select");
    select.get_mut().add_all_str(state.lines());
    siv.set_user_data(state);

    let xselect = OnEventView::new(select)
        .on_event(Key::Right, move |s| {
            let id = s.call_on_name("select", |select: &mut SelectView| { select.selected_id().unwrap() }).unwrap();
            s.with_user_data(|state: &mut State| {
                let coords = state.coords(id + 1);
                state.collapsed.remove(&coords);
            });
            redraw(s);
        })
        .on_event(Key::Left, move |s| {
            let id = s.call_on_name("select", |select: &mut SelectView| { select.selected_id().unwrap() }).unwrap();
            s.with_user_data(|state: &mut State| {
                let coords = state.coords(id + 1);
                state.collapsed.insert(coords);
            });
            redraw(s);
        });

    siv.add_fullscreen_layer(xselect.scrollable().full_screen());
    siv.add_global_callback('q', Cursive::quit);
    siv.run();
}
