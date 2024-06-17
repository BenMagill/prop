mod parser;

use parser::*;

fn main() {
    let example = "(p ∧ q) ∨";
    let ex2 = format!("(p {OR} q) {AND} (p {OR} r)");
    let ex3 = format!("{NOT}p {OR} {NOT} {NOT}q {OR} r {IMPL} z");
    let ex4 = format!("(p {OR} q) {OR} (r {OR} q)");
    let ex5 = format!("(p {OR} (q {OR} r)) {AND} (((p {AND} q) {OR} r) {AND} z)");

    // Parsed input must have no whitespace
    let in_chars: String = ex5.split_whitespace().collect();
    println!("Parsing: {}", &in_chars);
    (parser(&in_chars));
}
