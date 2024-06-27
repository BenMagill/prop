// TODO:
// ability to test the assigning of values to variables and caculate it
// ability to convert to CNF
// solving and returning the asignment of variables
// //

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Not,
    rc::Rc,
};

use crate::parser::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    name: char,
    negated: bool,
}

impl Not for &Variable {
    type Output = Variable;

    fn not(self) -> Self::Output {
        Variable {
            name: self.name,
            negated: !self.negated,
        }
    }
}

type Clause = Vec<Variable>;
type CNF = Vec<Clause>;

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

fn expand(tree: NodeRef) {
    if tree.borrow().left.is_some() {
        expand(tree.borrow().left.clone().unwrap());
    }
    if tree.borrow().right.is_some() {
        expand(tree.borrow().right.clone().unwrap());
    }

    if tree.borrow().value == OR {
        let n1 = tree.borrow().left.clone().unwrap();
        let n2 = tree.borrow().right.clone().unwrap();

        if n1.borrow().value == AND {
            let a = n1.borrow().left.clone();
            let b = n1.borrow().right.clone();
            let c = tree.borrow().right.clone();

            let l = Rc::new(RefCell::new(Node {
                value: OR,
                left: a,
                right: c.clone(),
            }));
            let r = Rc::new(RefCell::new(Node {
                value: OR,
                left: b,
                right: c.clone(),
            }));

            dbg!(&tree);
            //tree.borrow_mut().value = AND;
            //tree.borrow_mut().left = Some(l);
            //tree.borrow_mut().right = Some(r);
            tree.replace(Node {
                value: AND,
                left: Some(l),
                right: Some(r),
            });
            dbg!(&tree);
        } else if n2.borrow().value == AND {
            let a = tree.borrow().left.clone();
            let b = n2.borrow().left.clone();
            let c = n2.borrow().right.clone();

            let l = Rc::new(RefCell::new(Node {
                value: OR,
                left: a.clone(),
                right: b,
            }));
            let r = Rc::new(RefCell::new(Node {
                value: OR,
                left: a.clone(),
                right: c,
            }));

            dbg!(&tree);
            //tree.borrow_mut().value = AND;
            //tree.borrow_mut().left = Some(l);
            //tree.borrow_mut().right = Some(r);
            tree.replace(Node {
                value: AND,
                left: Some(l),
                right: Some(r),
            });
            dbg!(&tree);
        }
    }
}

pub fn cnf(tree: NodeRef) {
    remove_impl(tree.clone());
    remove_neg(tree.clone());
    expand(tree.clone());

    // TODO: convert to lists of clauses
}

pub fn to_clauses(tree: NodeRef) -> CNF {
    // When in CNF, all ANDs are at the top before any other symbols
    let no_and = remove_and(tree);

    let mut v = Vec::new();
    for c in no_and {
        v.push(
            remove_or(c)
                .iter()
                .map(|n| {
                    let value = n.borrow().value;
                    if n.borrow().value == NOT {
                        Variable {
                            name: n.borrow().left.clone().unwrap().borrow().value,
                            negated: true,
                        }
                    } else {
                        Variable {
                            name: value,
                            negated: false,
                        }
                    }
                })
                .collect(),
        );
    }

    v
}

fn remove_or(tree: NodeRef) -> Vec<NodeRef> {
    if tree.borrow().value == OR {
        let l = tree.borrow().left.clone().unwrap();
        let r = tree.borrow().right.clone().unwrap();

        let mut lv = remove_or(l);
        let mut rv = remove_or(r);

        let mut v = Vec::new();
        v.append(lv.as_mut());
        v.append(rv.as_mut());

        v
    } else {
        return vec![tree.clone()];
    }
}

fn remove_and(tree: NodeRef) -> Vec<NodeRef> {
    if tree.borrow().value == AND {
        let l = tree.borrow().left.clone().unwrap();
        let r = tree.borrow().right.clone().unwrap();

        let mut lv = remove_and(l);
        let mut rv = remove_and(r);

        let mut v = Vec::new();
        v.append(lv.as_mut());
        v.append(rv.as_mut());

        v
    } else {
        return vec![tree.clone()];
    }
}

fn simplify_where_true(clauses: &RefCell<Vec<Clause>>, var: &Variable) {
    let mut new_clauses: Vec<Vec<Variable>> = Vec::new();
    for c in clauses.borrow().iter() {
        if c.contains(var) {
            continue;
        } else if c.contains(&!var) {
            // Remove negative version of unit from clauses
            let i = c.into_iter().clone().filter(|&v| v != &!var);
            let new_c = i.map(|v| v.to_owned());
            new_clauses.push(new_c.collect());
        } else {
            new_clauses.push(c.to_owned());
        }
    }
    clauses.replace(new_clauses);
}

pub fn dpll(clauses: CNF) -> Option<Vec<Variable>> {
    let mut solution = vec![];
    // TODO: in a loop
    // remove tautologies
    // handle unit clauses
    // handle pure literals
    // check if t/f
    // case split
    dbg!(&clauses);

    let mut clauses = clauses.clone();

    // Remove tautologies
    clauses.retain(|c| !is_tautology(c));

    // Remove unit clauses
    let mut clauses = RefCell::new(clauses.clone());
    loop {
        let unit_clause = clauses
            .borrow_mut()
            .clone()
            .into_iter()
            .find(|c| c.len() == 1);

        if let Some(unit) = unit_clause {
            let unit = unit.get(0).unwrap();
            println!(
                "Removing unit {}{}",
                if unit.negated { "¬" } else { "" },
                unit.name
            );
            solution.push(unit.clone());
            // remove all negations inlcuding the unit variable
            simplify_where_true(&clauses, unit);
            //let mut new_clauses: Vec<Vec<Variable>> = Vec::new();
            //for c in clauses.borrow().iter() {
            //if c.contains(unit) {
            //continue;
            //} else if c.contains(&!unit) {
            //// Remove negative version of unit from clauses
            //let i = c.into_iter().clone().filter(|&v| v != &!unit);
            //let new_c = i.map(|v| v.to_owned());
            //new_clauses.push(new_c.collect());
            //} else {
            //new_clauses.push(c.to_owned());
            //}
            //}
            //clauses.replace(new_clauses);
        } else {
            // No units in clauses
            break;
        }
        dbg!(&clauses);
    }

    // Remove pure literals
    // Get all literals (incl negations)
    let c = clauses.borrow().clone();
    let u = unique_variables(&c);

    // Determine if any dont include their negation
    //let mut pure = vec![];
    for v in u.clone() {
        if !u.contains(&&!v) {
            solution.push(v.to_owned());
            // Remove clauses with pure literals
            clauses.borrow_mut().retain(|c| !c.contains(v))
        }
    }

    dbg!(&clauses);

    if not_valid(clauses.borrow().clone()) {
        None
    } else if clauses.borrow().clone().len() == 0 {
        Some(solution)
    } else {
        println!("DOING CASE SPLIT");
        // Select any variable v
        let c = clauses.borrow().clone();
        let v = c.get(0).unwrap().get(0).unwrap();
        // Case T = remove clauses with v, and remove ¬v from clauses
        let t = &clauses.clone();
        simplify_where_true(&clauses.clone(), v);
        let tr = dpll(t.borrow().clone());
        // Case F = remove clauses with ¬v, and remove v from clauses
        let f = &clauses.clone();
        simplify_where_true(&clauses.clone(), &!v);
        let fr = dpll(f.borrow().clone());

        if let Some(mut partial_solution) = tr {
            solution.append(&mut partial_solution);
            Some(solution)
        } else if let Some(mut partial_solution) = fr {
            solution.append(&mut partial_solution);
            Some(solution)
        } else {
            None
        }
    }
}

fn unique_variables(clauses: &Vec<Clause>) -> Vec<&Variable> {
    let mut h = HashSet::new();

    let mut flat: Vec<&Variable> = clauses.iter().flatten().collect();

    flat.retain(|&v| h.insert(v));

    flat
}

fn is_unit(clause: &Clause) -> Option<Variable> {
    if clause.len() == 1 {
        return Some(clause.get(0).unwrap().clone());
    }

    None
}

fn is_tautology(clause: &Clause) -> bool {
    // TODO: very inefficient
    for c in clause {
        for c2 in clause {
            if c.name == c2.name && c.negated != c2.negated {
                return true;
            }
        }
    }

    return false;
}

fn not_valid(clauses: Vec<Clause>) -> bool {
    match clauses.into_iter().find(|c| c.len() == 0) {
        Some(_) => true,
        None => false,
    }
}
