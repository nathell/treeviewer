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

fn append_path<'a>(ut: &mut Tree<&'a str>, path: &'a str) {
    let mut t = ut;
    for node in path.split("/") {
        let match_last = match t.children.last() {
            None => false,
            Some(x) => x.value == node
        };

        if match_last {
            t = t.children.last_mut().expect("should not happen");
        } else {
            let subtree = Tree { value: node, children: vec![] };
            t.children.push(subtree);
            t = t.children.last_mut().expect("should not happen");
        }
    }
}

fn main() {
    let mut t = Tree {value: "raz", children: vec![]};

    append_path(&mut t, "dwa/trzy/trzysta");
    append_path(&mut t, "dwa/cztery/cztery-i-pół");
    append_path(&mut t, "pięć/sześć/siedem");
    append_path(&mut t, "osiem");

    print_tree(&t);
}

#[cfg(test)]
mod tests {
}
