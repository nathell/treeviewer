use std::fmt::Display;

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

fn main() {
    let t = Tree {
        value: "raz",
        children: vec![
            Tree {value: "dwa", children: vec![
                Tree {value: "trzy", children: vec![]},
                Tree {value: "cztery", children: vec![]},
            ]},
            Tree {value: "pięć", children: vec![
                Tree {value: "sześć", children: vec![
                    Tree {value: "siedem", children: vec![]},
                ]},
            ]},
            Tree {value: "osiem", children: vec![]}]
    };

    print_tree(&t);
}

#[cfg(test)]
mod tests {
}
