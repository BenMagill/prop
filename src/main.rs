mod parser;
mod solver;

use std::collections::HashMap;

use parser::*;
use solver::{apply, cnf};

use crate::solver::{dpll, to_clauses};

fn main() {
    let example = "(p ∧ q) ∨";
    let ex2 = format!("(p {OR} q) {AND} (p {OR} r)");
    let ex3 = format!("{NOT}p {OR} {NOT} {NOT}q {OR} r {IMPL} z");
    let ex4 = format!("(p {OR} q) {OR} (r {OR} q)");
    let ex5 = format!("(p) {AND} (z) {AND} ( {NOT}z)");
    let ex6 = format!("{NOT}p {IMPL} (p {IMPL} {NOT}(p {OR} p))");
    let ex7 = format!("(p {OR} (q {OR} r)) {AND} (((p {AND} q) {OR} r) {AND} {NOT}z)");
    let ex8 = format!("{NOT}(p {AND} q {IMPL} q {AND} r)");

    // Parsed input must have no whitespace
    let in_chars: String = ex8.split_whitespace().collect();
    println!("Parsing: {}", &in_chars);
    let tree = (parser(&in_chars));

    let tree_0 = tree.get(0).unwrap().clone();
    cnf(tree_0.clone());
    dbg!(&tree_0);
    let c = to_clauses(tree_0.clone());

    dbg!(dpll(c));

    let mut map = HashMap::new();
    map.insert('p', true);
    map.insert('q', true);
    map.insert('r', true);
    map.insert('z', true);
    let value = apply(tree.get(0).unwrap().clone(), &map);
    println!("value = {}", value);
}
