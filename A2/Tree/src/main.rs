// Represents a binary tree, either a leaf or a node with two children
#[derive(Clone)]
enum Tree {
    Leaf,
    Node(Box<Tree>, Box<Tree>),
}

use Tree::*;

// Converts a binary tree to a string
fn serialize_tree(t: &Tree) -> String {
    match t {
        Leaf => "".to_string(),
        Node(l, r) => [
            "(".to_string(),
            serialize_tree(l),
            ".".to_string(),
            serialize_tree(r),
            ")".to_string(),
        ].join("")
    }
}

// Generates distinct binary trees with n internal nodes
fn generate_trees(n: i32) -> Vec<Box<Tree>> {
    // Base case: a tree with 0 internal nodes is just a leaf
    if n == 0 {
        return vec![Box::new(Leaf)];
    }

    // Holds all generated trees for n
    let mut result = vec![];

    for i in (0..n).rev() {
        let left_trees = generate_trees(i);
        let right_trees = generate_trees(n - 1 - i);  // Number of internal nodes allocated to right subtree
        for left in &left_trees {
            for right in &right_trees {
                result.push(Box::new(Node(left.clone(), right.clone())));  // New tree node with cloned subtrees, add to result
            }
        }
    }
    result
}

// Serializes distinct binary trees with n internal nodes
fn generate_tree_serializations(n: i32) -> Vec<String> {
    let trees = generate_trees(n);  // Generates all unique trees before preceding to serialization
    let mut serializations = vec![];
    for tree in trees {
        serializations.push(serialize_tree(&tree));
    }
    serializations
}

use std::env;

// Reads an integer from command line and prints tree serializations
fn main() {
    let cmdline: Vec<String> = env::args().collect();

    match cmdline.get(1) {
        None => {
            println!("usage:_CMD_num");
            return;
        },
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => {
                    print!("{}\n", n);

                    let trees = generate_tree_serializations(n);
                    for tree in trees {
                        println!("{}", tree);
                    }
                },
                Err(e) => {
                    println!("not_a_number:_{}", e);
                    return;
                }
            }
        }
    }
}
