use std::fmt::Display;

pub struct Tree<T> {
    value: T,
    children: Vec<Tree<T>>,
}

fn print_tree_with_prefix<T: Display>(prefix: &str, t: &Tree<T>) {
    println!("{0}{1}", prefix, t.value);
    let subprefix = format!("  {}", prefix);
    for child in t.children.iter() {
        print_tree_with_prefix(&subprefix, child);
    }
}

fn print_tree<T: Display>(t: &Tree<T>) {
    print_tree_with_prefix("", t);
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
