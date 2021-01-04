use std::fmt;


pub enum DataColumn {
    Numerical(Vec<f64>),
    Categorical(Vec<String>),
    Boolean(Vec<bool>),
}


pub struct DataFrame {
    pub column_names: Vec<String>,
    pub columns: Vec<DataColumn>,
}


impl DataFrame {
    pub fn new(
        column_names: Vec<&'static str>,
        columns: Vec<DataColumn>,
    ) -> Self {
        let mut df = DataFrame{column_names: vec![], columns};

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

        let num_rows = match &self.columns[0] {
            DataColumn::Numerical(v) => v.len(),
            DataColumn::Categorical(v) => v.len(),
            DataColumn::Boolean(v) => v.len(),
        };

        for i in 0..num_rows {
            let mut row: Vec<String> = vec![];

            for column in &self.columns {
                let cell = match column {
                    DataColumn::Numerical(v) => format!("{}", v[i]),
                    DataColumn::Categorical(v) => format!("{}", v[i]),
                    DataColumn::Boolean(v) => format!("{}", v[i]),
                };

                row.push(cell);
            }

            let row_formatted = row.join(" | ");
            lines.push(row_formatted);
        }

        let output = lines.join("\n");
        write!(f, "{}", output)
    }
}

