pub struct Tree<T> {
    value: T,
    children: Vec<Tree<T>>,
}

fn print_tree<T: std::fmt::Display>(t: &Tree<T>) {
    println!("Hello, world! Tree root is: {0} and first child is {1}", t.value, t.children[0].value);
}

fn main() {
    let t = Tree {
        value: String::from("root item"),
        children: vec![Tree {value: String::from("child"), children: vec![]},
                       Tree {value: String::from("another child"), children: vec![]}]
    };

    print_tree(&t);
}

#[cfg(test)]
mod tests {
}
