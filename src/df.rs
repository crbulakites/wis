use std::fmt;


pub enum DataColumn {
    Numerical(Vec<f64>),
    Categorical(Vec<String>),
    Boolean(Vec<bool>),
}


pub struct DataFrame {
    pub column_names: Vec<String>,
    pub rows: Vec<DataColumn>,
}


impl DataFrame {
    pub fn new(
        column_names: Vec<&'static str>,
        rows: Vec<DataColumn>,
    ) -> Self {
        let mut df = DataFrame{column_names: vec![], rows};

        for column_name in column_names {
            df.column_names.push(String::from(column_name));
        }

        df
    }
}


impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = vec![];
        
        let header = self.column_names.join(" | ");
        lines.push(header);

        for row in &self.rows {
            let row_formatted = match row {
                DataColumn::Numerical(_) => String::from("Numerical Row"),
                DataColumn::Categorical(v) => v.join(" | "),
                DataColumn::Boolean(_) => String::from("Boolean Row"),
            };
            lines.push(row_formatted);
        }

        let output = lines.join("\n");

        write!(f, "{}", output)
    }
}

