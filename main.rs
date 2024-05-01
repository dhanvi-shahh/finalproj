use std::{error::Error};
//use polars::prelude::*;
use std::fs::File;
use polars::prelude::CsvReader;
use std::collections::HashMap;

pub type dailyprice = Vec<(String, f64)>;
pub type allprices = HashMap<String, Vec<f64>>;
pub type edges = HashMap<String, Vec<String>>;

fn read(path:&str) -> Result<(HashMap<String, Vec<f64>>, Vec<(String, f64)>), Box<dyn Error>>{
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);  
    let datapoints: Vec<_> = rdr.records().collect::<Result<_, _>>()?;  
    let mut dailyprice: Vec<(String, f64)> = Vec::new();
    let mut allprices = HashMap::new();
    let baseline = datapoints.last().unwrap();
    let headers = rdr.headers()?.clone();
    let sample = datapoints.iter().step_by(30);
    for result in sample{
        let record = result;
        //println!("{:?}", record);
        let date = record.get(0).unwrap();
        for index in 1..record.len(){
          let base = baseline.get(index).unwrap_or("Base Not Readable").parse::<f64>()?;
          let asset = headers.get(index).unwrap().to_string();
          let value: f64 = record.get(index)
            .unwrap()
            .parse::<f64>()?;
          let logvalue: f64 = (value/base).ln();
          let label = format!("{:?}, {:?}", date, asset);
          allprices.entry(asset).or_insert_with(Vec::new).push(logvalue);
          dailyprice.push((label, logvalue.into()));
        }
    }
    //println!("{:?}", dailyprice);
    //println!("{:?}", allprices);
    Ok((allprices, dailyprice))
}

fn createadjlist(dailyprice: dailyprice) -> edges{
  let mut edges = HashMap::new();
  for (i, (ilabel, ivalue)) in dailyprice.iter().enumerate(){
    for (j, (jlabel, jvalue)) in dailyprice.iter().enumerate(){
      if i==j{
        continue;
      }
      else {
        let similarity = (((ivalue) - (jvalue)).abs())/(ivalue);
        if similarity > 99.99{
          edges.entry(ilabel.to_string()).or_insert_with(Vec::new).push(jlabel.to_string());
      }
    }
    }
  }
  edges
}

#[derive(Debug)]
pub struct Graph{
  n:usize, 
  vertices: dailyprice, 
  adjlist: edges, 
}

impl Graph{
  pub fn new(n:usize, vertices: Vec<(String, f64)>, adjlist: HashMap<String, Vec<String>>) -> Self{
    Graph {n, vertices, adjlist: HashMap::new()}
  }

  //fn undirected(n: usize, vertices: Vec<(String, f64)>) -> Graph{
    //let mut graph = Graph{n, vertices};
    //graph.vertices.sort_by(|a, b| a.0.cmp(&b.0));
    //graph
  //}
}

//fn dailyexpect(dailyprices) -> (posret, negret){
  //let (posret, negret): 
//}

fn main() {
  let path = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\project\daily_asset_prices.csv";
  let (allprices, dailyprice) = read(path).expect("Couldn't Read!");
  let adjlist = createadjlist(dailyprice.clone());
  let n = dailyprice.len();
  let graph = Graph::new(n, dailyprice.clone(), adjlist.clone());
  println!("{:?}", adjlist);
}

