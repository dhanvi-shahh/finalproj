use std::{error::Error};
//use polars::prelude::*;
use std::fs::File;
//use polars::prelude::CsvReader;

fn read(path:&str) -> Result<(), Box<dyn Error>>{
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut dailyprice: Vec<(String, f64)> = Vec::new();
    let headers = rdr.headers()?.clone();
    for result in rdr.records(){
        let record = result?;
        let date = record.get(0).unwrap();
        for index in 1..record.len(){
          let asset = headers.get(index).unwrap();
          let value: f64 = record.get(index)
            .unwrap()
            .parse::<f64>()?;

          let label = format!("{:?}, {:?}", date, asset);
          dailyprice.push((label, value.into()));
        }
        println!("{:?}", dailyprice);
    }
    Ok(())
}


fn main() {
    let path = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\project\daily_asset_prices.csv";
    let data = read(path);
    println!("{:?}", data);
    //if let Err(err) = example() {
      //  println!("error running example: {}", err);
        //process::exit(1);
    //}
}

