use std::fmt;
use std::vec::Vec;


pub enum DataCell {
    Number(f64),
    Categorical(String),
    Boolean(bool),
}


pub type DataRow = Vec<DataCell>;


pub struct DataFrame {
    pub ordered_column_names: Vec<String>,
    pub rows: Vec<DataRow>,
}


impl DataFrame {
    pub fn new(ordered_column_names: Vec<&'static str>) -> Self {
        let mut df = DataFrame{ordered_column_names: vec![], rows: vec![]};

        for column_name in ordered_column_names {
            df.ordered_column_names.push(String::from(column_name));
        }

        df
    }
}


impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = vec![];
        
        let header = self.ordered_column_names.join(" | ");
        lines.push(header);

        let output = lines.join("\n");

        write!(f, "{}", output)
    }
}

