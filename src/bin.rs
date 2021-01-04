extern crate wis;

use wis::DecisionTree;
use wis::df::DataFrame;


fn main() {
    let df = DataFrame::new(
        vec!["x", "y", "label"],
        vec![
            (0.0, 0.0, false),
            (0.0, 1.0, true),
            (1.0, 0.0, true),
            (1.0, 1.0, false),
        ],
    );
    println!("{}", df);

    let dt = DecisionTree{};
    println!("{}", dt);
}

