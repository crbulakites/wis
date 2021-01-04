extern crate wis;

use wis::DecisionTree;
use wis::df::*;


fn main() {
    let df = DataFrame::new(vec!["x", "y", "label"]);
    println!("{}", df);

    let dt = DecisionTree{};
    println!("{}", dt);
}

