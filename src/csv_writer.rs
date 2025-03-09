use std::io::{Write,BufWriter,BufRead,BufReader};
use std::fs::{self,File};

pub trait CSVWritable {
    fn to_csv_row(&self) -> Vec<String>;
}
pub struct CSVWriter {
    writer : BufWriter<File>,
    columns : Vec<String>,
}

impl CSVWriter {
    pub fn new(file_path : &str, columns : Vec<String>) -> Self {
        let file = File::create(file_path).unwrap();
        let mut body = CSVWriter {
            writer : BufWriter::new(file),
            columns
        };
        body.write_header();
        body
    }

    fn write_header(&mut self) {
        let header = self.columns.join(",");
        self.writer.write_all(header.as_bytes()).unwrap();
        self.writer.write_all(b"\n").unwrap();
    }

    pub fn write_row<T : CSVWritable>(&mut self, row : &T) {
        let row = row.to_csv_row();
        assert!(row.len() == self.columns.len());
        let row = row.join(",");
        self.writer.write_all(row.as_bytes()).unwrap();
        self.writer.write_all(b"\n").unwrap();
        self.writer.flush().unwrap();
    }

    pub fn get_listener<TReceiveData : CSVWritable>(mut self) -> Box<dyn FnMut(&TReceiveData)> {
        Box::new(move |data : &TReceiveData| {
            self.write_row(data);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestData {
        data : i32,
        data2 : i32,
    }

    impl CSVWritable for TestData {
        fn to_csv_row(&self) -> Vec<String> {
            vec![self.data.to_string(), self.data2.to_string()]
        }
    }

    #[test]
    fn test_csv_writer() {
        let columns = vec![String::from("data"), String::from("data2")];
        let writer = CSVWriter::new("test.csv", columns);
        let mut listener = writer.get_listener();
        listener(&TestData { data: 1, data2 : 3 });
        listener(&TestData { data: 2, data2 : 4 });

        let file = File::open("test.csv").unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        assert_eq!(lines.next().unwrap().unwrap(), "data,data2");
        assert_eq!(lines.next().unwrap().unwrap(), "1,3");
        assert_eq!(lines.next().unwrap().unwrap(), "2,4");
        fs::remove_file("test.csv").unwrap();
    }
}