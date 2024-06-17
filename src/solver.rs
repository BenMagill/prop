// TODO:
// ability to test the assigning of values to variables and caculate it
// ability to convert to CNF
// solving and returning the asignment of variables
// //

use std::collections::HashMap;

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
            // TODO: assign value to variable
            let assignment = assignments
                .get(&c)
                .expect(format!("Unexpected variable {}", c).as_str());
            println!("PROP VAR: {} = {}", c, assignment);
            *assignment
        }
    }
}
