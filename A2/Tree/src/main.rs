// binary tree with lifetime 'a
// we've learned about extent and no longer need 'Box'es
enum Tree<'a> {
    Leaf (/* no data */),
    Node (&'a Tree<'a>, &'a Tree<'a>)
}

use Tree::*;

// but we don’t know Rust OO and ‘trait’s
// so implementing ‘Display’ is still beyond us
// therefore so we’ll do it manually
fn serialize_tree(t: &Tree) -> String {
    match t {
        Leaf() => "".to_string(),
        Node(l, r) => [
            "(".to_string(),
            serialize_tree(l),
            ".".to_string(),
            serialize_tree(r),
            ")".to_string()
            ].join("")
    }
}

// read an integer from command-line
// then your program should print the distinct trees
// containing the given number of nodes
// this scaffold simply shows use of the supplied code
use std::env;

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

                    print!("{}\n",
                        serialize_tree(
                            &Node(&Node(&Node(&Leaf(), &Leaf()), &Leaf()),
                                  &Leaf())
                        )
                    );

                    print!("{}\n",
                        serialize_tree(
                            &Node(&Node(&Leaf(), &Leaf()),
                                  &Node(&Leaf(), &Leaf())
                            )
                        )
                    );
                },
                Err(e) => {
                    println!("not_a_number:_{}", e);
                    return;
                }
            }
        }
    }
}