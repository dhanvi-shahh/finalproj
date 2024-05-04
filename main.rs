use std::{error::Error};
use ndarray::prelude::*;
use ndarray::array;
use std::fs::File;
use polars::prelude::CsvReader;
use std::collections::HashMap;
use rand::prelude::SliceRandom;

pub type nodes = Vec<(String, f64)>;
pub type assetprices = HashMap<String, Vec<f64>>;
pub type edges = HashMap<String, Vec<String>>;

fn read(path:&str) -> Result<(HashMap<String, Vec<f64>>, Vec<(String, f64)>), Box<dyn Error>>{
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);  
    let datapoints: Vec<_> = rdr.records().collect::<Result<_, _>>()?;  
    let mut nodes: Vec<(String, f64)> = Vec::new();
    let mut assetprices = HashMap::new();
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
         assetprices.entry(asset).or_insert_with(Vec::new).push(logvalue);
          nodes.push((label, logvalue.into()));
        }
    }
    Ok((assetprices, nodes))
}


fn createadjlist(nodes: nodes, threshold:f64, n:usize) -> (edges, Vec<Vec<bool>>){
  let mut edges = HashMap::new();
  let mut adjmat = vec![vec![false;n];n];
  for (i, (ilabel, ivalue)) in nodes.iter().enumerate(){
    for (j, (jlabel, jvalue)) in nodes.iter().enumerate(){
      if i==j{
        adjmat[i][j]= true;
        continue;
      }
      else{
        let dprod = ivalue *jvalue;
        let v1mag = (ivalue*ivalue).sqrt();
        let v2mag = (jvalue *jvalue).sqrt();
        let corr = dprod/(v1mag + v2mag);
        if corr > threshold {
          edges.entry(ilabel.to_string()).or_insert_with(Vec::new).push(jlabel.to_string());
          adjmat[i][j] = true;
        }
      }
    }
  }
  (edges, adjmat)
}

fn furthest(edges: edges) -> HashMap<String, String>{
  let mut farneigh: HashMap<String, String> = HashMap::new();
  for asset in edges.keys(){
    let original:String = asset.to_string(); 
    let mut step: String = edges[&original].choose(&mut rand::thread_rng()).unwrap().to_string();
    let mut current = step.clone();
    let mut visitedneigh: Vec<String> = Vec::new();
    visitedneigh.push(current.clone());
    for _ in 0..20{
      step = edges[&current].choose(&mut rand::thread_rng()).unwrap().to_string();
      if edges.contains_key(&step) && !visitedneigh.contains(&step){
        current = step.clone();
        visitedneigh.push(current.clone());
      }
    }
    farneigh.insert(asset.to_string(), current.to_string());
  }
  farneigh
}

#[derive(Debug)]
pub struct Graph{
  n:usize, 
  vertices: nodes, 
  adjlist: edges, 
}

impl Graph{
  pub fn new(n:usize, vertices: Vec<(String, f64)>, adjlist: HashMap<String, Vec<String>>) -> Self{
    Graph {n, vertices, adjlist: HashMap::new()}
  }

  fn undirected(n: usize, vertices: Vec<(String, f64)>, adjlist: HashMap<String, Vec<String>>) -> Graph{
    let mut graph = Graph{n, vertices, adjlist};
    graph.vertices.sort_by(|a, b| a.0.cmp(&b.0));
    graph
  }

  fn dailyexpect(&self) -> (Vec<(String, f64)>, Vec<(String, f64)>){
    let (posret, negret): (Vec<_>, Vec<_>) =
      self.vertices.clone()
      .into_iter()
      .partition(|&(_, value)| value >= 0.0);
    (posret, negret)
  }
}

fn main() {
  let path = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\project\daily_asset_prices.csv";
  let (assetprices, nodes) = read(path).expect("Couldn't Read!");
  let n = nodes.len();
  let (adjlist, adjmat) = createadjlist(nodes.clone(), 0.5, n);
  println!{"{:?}", adjlist};
  let far = furthest(adjlist.clone());
  //println!("{:?}", far);
  let graph = Graph::new(n, nodes.clone(), adjlist.clone());
  let (positive, negative) = graph.dailyexpect();
}

#[test]
fn test(){
  let path = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\final_proj\final proj test.csv";
  let (all, daily) = read(path).expect("Couldn't Read");
  let n = daily.len();
  let (adjlisttest, adjmattest)  = createadjlist(daily.clone(), 0.05, n);
  let far_test = furthest(adjlisttest.clone());
  println!("{:?}", far_test);
  assert_eq(far_test)
}