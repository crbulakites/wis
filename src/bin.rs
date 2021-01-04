extern crate wis;

use wis::DecisionTree;
use wis::df::{DataFrame, DataColumn};


fn main() {
    let df = DataFrame::new(
        vec!["x", "y", "label"],
        vec![
            DataColumn::Numerical(vec![0.0, 0.0, 1.0, 1.0]),
            DataColumn::Numerical(vec![0.0, 1.0, 0.0, 1.0]),
            DataColumn::Boolean(vec![false, true, true, false]),
        ],
    );
    println!("{}", df);

    let dt = DecisionTree{};
    println!("{}", dt);
}

