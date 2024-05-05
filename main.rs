use std::{error::Error};
use std::fs::File;
use std::collections::HashMap;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;
pub type Nodes = Vec<(String, f64)>;
pub type Assetprices = HashMap<String, Vec<f64>>;
pub type Edges = HashMap<String, Vec<String>>;
pub type Matrix = Vec<Vec<bool>>;

fn read(path:&str) -> Result<(HashMap<String, Vec<f64>>, Vec<(String, f64)>), Box<dyn Error>>{
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);  
    let datapoints: Vec<_> = rdr.records().collect::<Result<_, _>>()?;  
    let mut Nodes: Vec<(String, f64)> = Vec::new();
    let mut Assetprices = HashMap::new();
    let baseline = datapoints.last().unwrap();
    let headers = rdr.headers()?.clone();
    let sample = datapoints.iter().step_by(30);
    for result in sample{
        let record = result;
        let date = record.get(0).unwrap();
        for index in 1..record.len(){
          let base = baseline.get(index).unwrap_or("Base Not Readable").parse::<f64>()?;
          let asset = headers.get(index).unwrap().to_string();
          let value: f64 = record.get(index)
            .unwrap()
            .parse::<f64>()?;
          let logvalue: f64 = (value/base).ln();
          let label = format!("{:?}, {:?}", date, asset);
         Assetprices.entry(asset).or_insert_with(Vec::new).push(logvalue);
          Nodes.push((label, logvalue.into()));
        }
    }
    Ok((Assetprices, Nodes))
}

mod adjacent;
mod create;
use create::Graph;

fn recommend(Edges: Edges, risklevel:i32) -> HashMap<String, String>{
  let mut farneigh: HashMap<String, String> = HashMap::new();
  for asset in Edges.keys(){
    let original:String = asset.to_string(); 
    let mut step: String = Edges[&original].choose(&mut rand::thread_rng()).unwrap().to_string();
    let mut current = step.clone();
    let mut visitedneigh: Vec<String> = Vec::new();
    visitedneigh.push(current.clone());
    for _ in 0..risklevel{
      step = Edges[&current].choose(&mut rand::thread_rng()).unwrap().to_string();
      if Edges.contains_key(&step) && !visitedneigh.contains(&step){
        current = step.clone();
        visitedneigh.push(current.clone());
      }
    }
    farneigh.insert(asset.to_string(), current.to_string());
  }
  farneigh
}

fn main() {
  let path = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\project\daily_asset_prices.csv";
  let (_Assetprices, Nodes) = read(path).expect("Couldn't Read!");
  let n = Nodes.len();
  let (adjmap, adjmat) = adjacent::createadj(Nodes.clone(), 0.1, n);
  let far = recommend(adjmap.clone(), 7);
  let graph = Graph::new(n, Nodes.clone(), adjmap.clone(), adjmat.clone());
  let (positive, negative) = graph.dailyexpect();
  graph.portfolio();
  graph.groups();
}

#[test]
fn test(){
  let testpath = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\final_proj\final proj test.csv";
  let (Assetstest, Nodestest) = read(testpath).expect("Couldn't Read!");
  let nt = Nodestest.len();
  let (testmap, testmat) = createadj(Nodestest.clone(), 0.1, nt);
  let far_test = recommend(testmap.clone());
  let testgraph = Graph::new(nt, Nodestest.clone(), testmap.clone(), testmat.clone());
  let (testpos, testneg) = testgraph.dailyexpect();
  assert_eq!(testneg.iter().map(|(s, v)| (s.as_str(), *v)).collect::<Vec<_>>(), 
    [("\"11/4/2013\", \"Commodities\"", -0.004334982158052806), ("\"11/4/2013\", \"Emerg Markets Bonds\"", -0.00018130723716086204)]);
  let risk = testgraph.portfolio();
}
