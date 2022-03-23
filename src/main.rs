extern crate csv;

use std::error::Error;
use csv::StringRecord;

#[derive(Debug)]
struct DataFrame {
    header: csv::StringRecord,
    asofdate: Vec<String>,
    firstname: Vec<String>,
    lastname: Vec<String>,
    score: Vec<u32>
}


impl DataFrame {

    fn new() -> DataFrame {
        DataFrame {
            header: csv::StringRecord::new(),
            asofdate: Vec::new(),
            firstname: Vec::new(),
            lastname: Vec::new(),
            score: Vec::new(),
        }
    }

    fn read_csv(filepath: &str, has_headers: bool) -> DataFrame {
        // Open file
        let file = std::fs::File::open(filepath).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);

        let mut data_frame = DataFrame::new();

        // push all the records
        for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
        }
        return data_frame;
    }

    fn push(&mut self, row: &csv::StringRecord) {
        // asofdate
        self.asofdate.push(row[0].to_string());
        // firstname
        self.firstname.push(row[1].to_string());
        // lastname
        self.lastname.push(row[2].to_string());
        // score
        self.score.push(row[3].parse().unwrap());
    }
}


fn main() {
    let data = DataFrame::read_csv("./src/some.csv", true);

    println!("{:?}", data)
}