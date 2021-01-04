pub mod df;

use std::fmt;


pub struct DecisionTree {
}


impl fmt::Display for DecisionTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DecisionTree")
    }
}

