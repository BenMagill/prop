// TODO:
// ability to test the assigning of values to variables and caculate it
// ability to convert to CNF
// solving and returning the asignment of variables
// //

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::*;

pub fn apply(tree: NodeRef, assignments: &HashMap<char, bool>) -> bool {
    // Collapse tree to a single truth value

    let mut l = None;
    if tree.borrow().left.is_some() {
        let node = tree.borrow_mut().left.clone().unwrap();
        let solution = apply(node, assignments);
        l = Some(solution);
    }

    let mut r = None;
    if tree.borrow().right.is_some() {
        let node = tree.borrow().right.clone().unwrap();
        let solution = apply(node, assignments);
        r = Some(solution);
    }

    let c = tree.borrow().value;

    match c {
        NOT => !l.unwrap(),
        AND => l.unwrap() && r.unwrap(),
        OR => l.unwrap() || r.unwrap(),
        IMPL => (!l.unwrap()) || r.unwrap(),
        _ => {
            let assignment = assignments
                .get(&c)
                .expect(format!("Unexpected variable {}", c).as_str());
            //println!("PROP VAR: {} = {}", c, assignment);
            *assignment
        }
    }
}

fn remove_impl(tree: NodeRef) {
    if tree.borrow().value == IMPL {
        let l = tree.borrow().left.clone();
        //let r = tree.borrow().right.clone().unwrap();
        tree.borrow_mut().value = OR;
        let node = Rc::new(RefCell::new(Node {
            value: NOT,
            left: l,
            right: None,
        }));
        tree.borrow_mut().left = Some(node);
    }

    if tree.borrow().left.clone().is_some() {
        remove_impl(tree.borrow().left.clone().unwrap());
    }
    if tree.borrow().right.clone().is_some() {
        remove_impl(tree.borrow().right.clone().unwrap());
    }
}

fn remove_neg(tree: NodeRef) {
    if tree.borrow().value == NOT {
        let value = tree.borrow().left.clone().unwrap().borrow().value;
        let n1 = tree.borrow().left.clone().unwrap();
        if value == NOT {
            let n2 = n1.borrow().left.clone().unwrap();
            tree.borrow_mut().value = n2.borrow().value;
            tree.borrow_mut().left = n2.borrow().left.clone();
            tree.borrow_mut().right = n2.borrow().right.clone();
        } else if value == AND {
            // NOT (a AND b) = NOT a OR NOT b

            let l = n1.borrow().left.clone();
            let r = n1.borrow().right.clone();

            tree.borrow_mut().value = OR;
            tree.borrow_mut().left = Some(Rc::new(RefCell::new(Node {
                value: NOT,
                left: l,
                right: None,
            })));
            tree.borrow_mut().right = Some(Rc::new(RefCell::new(Node {
                value: NOT,
                left: r,
                right: None,
            })));
        } else if value == OR {
            let l = n1.borrow().left.clone();
            let r = n1.borrow().right.clone();

            tree.borrow_mut().value = AND;
            tree.borrow_mut().left = Some(Rc::new(RefCell::new(Node {
                value: NOT,
                left: l,
                right: None,
            })));
            tree.borrow_mut().right = Some(Rc::new(RefCell::new(Node {
                value: NOT,
                left: r,
                right: None,
            })));
        }
    }
    if tree.borrow().left.clone().is_some() {
        remove_neg(tree.borrow().left.clone().unwrap());
    }
    if tree.borrow().right.clone().is_some() {
        remove_neg(tree.borrow().right.clone().unwrap());
    }
}
fn push_neg(tree: NodeRef) {}

pub fn cnf(tree: NodeRef) {
    // TODO:
    // replace all a IMPL b with  NOT a OR b
    // replace all a OR (b AND c) with (a OR b) AND (a OR c)
    // replace all (a AND b) OR c with (a OR c) AND (b OR c)

    remove_impl(tree.clone());
    remove_neg(tree.clone());
    //push_neg(tree);
}
