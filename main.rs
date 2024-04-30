use std::{error::Error, io, process};
//use polars::prelude::*;
use std::fs::File;
//use polars::prelude::CsvReader;
use std::io::{BufRead};//, BufReader};

fn read(path:&str) -> Result<(), Box<dyn Error>>{
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records(){
        let record = result?;
        println!("{:?}", record);
        //match record{
          //  record[0] => date, 
            //record[1..] =>
        //}
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

