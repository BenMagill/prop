use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

pub const OR: char = '∨';
pub const AND: char = '∧';
pub const NOT: char = '¬';
pub const IMPL: char = '→';

#[derive(Debug)]
pub struct Node {
    value: char,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

impl Node {
    // Determine if a node should be skipped in parsing
    fn parsed(&self) -> bool {
        return self.left.is_some() || self.right.is_some();
    }
}

pub type NodeRef = Rc<RefCell<Node>>;

pub fn parser(chars: &str) -> Vec<NodeRef> {
    println!("Running parser on: {}", chars);
    let mut tokens: Vec<NodeRef> = vec![];

    // Convert characters into nodes with no children
    chars.chars().for_each(|c| {
        tokens.push(Rc::new(RefCell::new(Node {
            value: c,
            left: None,
            right: None,
        })))
    });

    // Parse brackets and their nested expressions
    let mut i = 0;
    loop {
        if i >= tokens.len() {
            break;
        }

        let node = tokens.get(i).unwrap();
        if node.borrow().value == '(' {
            // Determine where the open bracket closes
            let mut indent_level = 1;
            let mut buf = vec![];
            let start_i = i;
            loop {
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
            let end_i = i;
            let brackets = buf.into_iter().collect::<String>();
            println!("Parsing Inside brackets: {}", brackets);
            let nodes = parser(brackets.as_str());

            println!(
                "Removing items from {} to {} from {}",
                start_i, end_i, chars
            );

            // Replace the expression in the brackets with the parsed version
            for x in (start_i..=end_i).rev() {
                println!("Removing i={}", x);
                tokens.remove(x);
            }
            i = i - (end_i - start_i);

            tokens.insert(start_i, nodes.get(0).unwrap().clone());
            println!("After removal and insertion i = {}", i);
            dbg!(&tokens);
        } else {
            i = i + 1;
        }
    }

    // Parse NOT backwards to properly next double negations
    // TODO: could filter out double negations first instead?
    let mut i = tokens.len() - 1;
    loop {
        let node = tokens.get(i).unwrap();

        if node.borrow().value == NOT && !node.borrow().parsed() {
            let next = tokens.get(i + 1).unwrap();
            node.borrow_mut().left = Some(next.to_owned());
            tokens.remove(i + 1);
        }

        if i == 0 {
            break;
        }
        i = i - 1;
    }

    let mut i = 0;
    loop {
        if i >= tokens.len() {
            break;
        }

        let node = tokens.get(i).unwrap();

        if (node.borrow().value == AND || node.borrow().value == OR) && !node.borrow().parsed() {
            println!("i = {}", i);
            let prev = tokens.get(i - 1).unwrap();
            let next = tokens.get(i + 1).unwrap();
            node.borrow_mut().left = Some(prev.to_owned());
            node.borrow_mut().right = Some(next.to_owned());
            tokens.remove(i + 1);
            tokens.remove(i - 1);
        } else {
            i = i + 1;
        }
    }

    let mut i = 0;
    loop {
        if i >= tokens.len() {
            break;
        }

        let node = tokens.get(i).unwrap();

        if node.borrow().value == IMPL && !node.borrow().parsed() {
            let prev = tokens.get(i - 1).unwrap();
            let next = tokens.get(i + 1).unwrap();
            node.borrow_mut().left = Some(prev.to_owned());
            node.borrow_mut().right = Some(next.to_owned());
            tokens.remove(i + 1);
            tokens.remove(i - 1);
        } else {
            i = i + 1;
        }
    }

    dbg!(&tokens);
    return tokens;
}
