use std::fmt;
use std::vec::Vec;


pub struct DataFrame<Schema> {
    pub ordered_column_names: Vec<String>,
    pub rows: Vec<Schema>,
}


impl<Schema> DataFrame<Schema> {
    pub fn new(
        ordered_column_names: Vec<&'static str>,
        rows: Vec<Schema>,
    ) -> Self {
        let mut df = DataFrame{ordered_column_names: vec![], rows};

        for column_name in ordered_column_names {
            df.ordered_column_names.push(String::from(column_name));
        }

        df
    }
}


impl<Schema> fmt::Display for DataFrame<Schema> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = vec![];
        
        let header = self.ordered_column_names.join(" | ");
        lines.push(header);

        let output = lines.join("\n");

        write!(f, "{}", output)
    }
}

