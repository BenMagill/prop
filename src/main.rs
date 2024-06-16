use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;
use std::sync::Arc;
use std::vec;

use slab_tree::*;

const OR: char = '∨';
const AND: char = '∧';
const NOT: char = '¬';
const IMPL: char = '→';

// Order:
// 1. NOT
// 2. AND / OR
// 3. IMPL

fn main() {
    let example = "(p ∧ q) ∨";
    let ex2 = format!("(p {OR} q) {AND} (p {OR} r)");
    let ex3 = format!("{NOT}p {OR} {NOT} {NOT}q {OR} r {IMPL} z");
    let ex4 = format!("(p {OR} q) {OR} (r {OR} q)");
    let ex5 = format!("(p {OR} (q {OR} r)) {AND} (((p {AND} q) {OR} r) {AND} z)");
    println!("Hello, world!");
    let mut in_chars: String = ex5.split_whitespace().collect();
    println!("Parsing: {}", &in_chars);
    (parser2(&in_chars));

    let mut chars = in_chars.chars().peekable();
    //let out = parser(&mut chars);
    //dbg!(out);
}

struct Parser {
    index: usize,
}
//impl Parser {
//fn parse(inp: Vec<char>) {
//while true {
//c
//if
//}
//}
//}

#[derive(Debug)]
struct Node {
    value: char,
    children: Vec<NodeRef>,
}

type NodeRef = Rc<RefCell<Node>>;

fn parser2(chars: &str) -> Vec<NodeRef> {
    println!("Running parser on: {}", chars);
    let mut tokens: Vec<NodeRef> = vec![];

    // Convert characters into nodes with no children
    chars.chars().for_each(|c| {
        tokens.push(Rc::new(RefCell::new(Node {
            value: c,
            children: vec![],
        })))
    });

    // Loop through merging nodes
    // TODO: First brackets
    // TODO: once brackets work, need to redo how the rest of it is parsed
    let mut i = 0;
    while true {
        if i >= tokens.len() {
            break;
        }

        let node = tokens.get(i).unwrap();
        if node.borrow().value == '(' {
            let mut indent_level = 1;
            let mut buf = vec![];
            let start_i = i;
            while true {
                i = i + 1;
                let c = tokens.get(i).unwrap().borrow().value;
                if c == '(' {
                    indent_level = indent_level + 1;
                } else if c == ')' {
                    indent_level = indent_level - 1;

                    if indent_level == 0 {
                        break;
                    }
                }
                buf.push(c);
            }
            //for c in chars.chars().skip(i + 1) {
                //i = i + 1;
                //if c == '(' {
                    //indent_level = indent_level + 1;
                //} else if c == ')' {
                    //indent_level = indent_level - 1;

                    //if indent_level == 0 {
                        //break;
                    //}
                //}
                //buf.push(c);
            //}
            let end_i = i;
            let brackets = buf.into_iter().collect::<String>();
            println!("Parsing Inside brackets: {}", brackets);
            let nodes = parser2(brackets.as_str());

            println!(
                "Removing items from {} to {} from {}",
                start_i, end_i, chars
            );

            // TODO: might be wrong removal amount
            for x in (start_i..=end_i).rev() {
                println!("Removing i={}", x);
                tokens.remove(x);
            }
            i = i - (end_i - start_i);
            let node = Rc::new(RefCell::new(Node {
                value: '_',
                children: nodes,
            }));
            tokens.insert(start_i, node);
            println!("After removal and insertion i = {}", i);
            dbg!(&tokens);
        } else {
            i = i + 1;
        }
    }

    let mut i = tokens.len() - 1;
    while true {
        let node = tokens.get(i).unwrap();
        //dbg!(node);

        if node.borrow().value == NOT {
            let next = tokens.get(i + 1).unwrap();
            node.borrow_mut().children.push(next.to_owned());
            tokens.remove(i + 1);
        }

        if i == 0 {
            break;
        }
        i = i - 1;
    }

    let mut i = 0;
    while true {
        if i >= tokens.len() {
            break;
        }

        let node = tokens.get(i).unwrap();
        //dbg!(node);

        if node.borrow().value == AND || node.borrow().value == OR {
            let prev = tokens.get(i - 1).unwrap();
            let next = tokens.get(i + 1).unwrap();
            node.borrow_mut().children.push(prev.to_owned());
            node.borrow_mut().children.push(next.to_owned());
            tokens.remove(i + 1);
            tokens.remove(i - 1);
        } else {
            i = i + 1;
        }
    }

    let mut i = 0;
    while true {
        if i >= tokens.len() {
            break;
        }

        let node = tokens.get(i).unwrap();
        //dbg!(node);

        if node.borrow().value == IMPL {
            let prev = tokens.get(i - 1).unwrap();
            let next = tokens.get(i + 1).unwrap();
            node.borrow_mut().children.push(prev.to_owned());
            node.borrow_mut().children.push(next.to_owned());
            tokens.remove(i + 1);
            tokens.remove(i - 1);
        } else {
            i = i + 1;
        }
    }

    dbg!(&tokens);
    return tokens;
}

fn parser(chars: &mut Peekable<Chars>) -> NodeRef {
    let vars = ['p', 'q', 'r'];
    // Starting on left
    //
    // create a stack of tokens to create a tree from
    // if ( found, keep parsing
    // if ) found, take everything from top to first ( found
    // if operator found, .....

    //let mut tree: Tree<Node> = TreeBuilder::new().build();

    //let mut out: Vec<String> = Vec::new();

    let mut tree: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
        value: '$',
        children: vec![],
    }));

    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        println!("{c}");
        if c == '(' {
            //let subtree = parser(chars);
            //tree.borrow_mut().children.push(subtree);
        } else if vars.contains(&c) {
            // TODO: handle single variable with no connector
            //println!("Found var {c}");
            let connector = chars.next().unwrap();
            let var2 = chars.next().unwrap();
            let n = Node {
                value: connector,
                children: vec![
                    Rc::new(RefCell::new(Node {
                        value: c,
                        children: vec![],
                    })),
                    Rc::new(RefCell::new(Node {
                        value: var2,
                        children: vec![],
                    })),
                ],
            };
            dbg!(&n);
            if tree.borrow().value == '$' {
                tree.replace(n);
                //*tree.borrow_mut() = n;
            } else {
                tree.borrow_mut().children.push(Rc::new(RefCell::new(n)));
            }
        } else if c == OR {
            let v = chars.next().unwrap();
            let n = Node {
                value: c,
                children: vec![
                    Rc::new(RefCell::new(Node {
                        value: tree.borrow().value,
                        children: tree.borrow().children.clone(),
                    })),
                    Rc::new(RefCell::new(Node {
                        value: v,
                        children: vec![],
                    })),
                ],
            };
            dbg!(&n);
            //*tree.borrow_mut() = n;
            tree.replace(n);
            //dbg!(&tree);
        } else if c == NOT {
        } else if c == ')' {
            return tree;
        }
    }

    for c in &tree.borrow().children {
        let c2 = c.borrow();
        let v = c2.value;
        println!("{v}");
        for c in &c2.children {
            let c2 = c.borrow();
            let v = c2.value;
            println!("   {v}");
            for c in &c2.children {
                let c2 = c.borrow();
                let v = c2.value;
                println!("      {v}");
            }
        }
    }
    dbg!(tree.borrow());
    return tree;

    //for c in input.chars() {
    //match c {
    //'(' => stack.push(c),
    //')' => {
    //let mut buf = String::from("");
    //while stack.last().copied().unwrap() != '(' {
    //let char = stack.pop().unwrap();
    //buf.push(char);
    //}
    //out.push(buf)
    //}
    //_ => stack.push(c),
    ////_ => println!("ERROR with {c}"),
    //}
    //}
}
