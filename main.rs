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


fn createadj(Nodes: Nodes, threshold:f64, n:usize) -> (Edges, Matrix){
  let mut Edges = HashMap::new();
  let mut Matrix: Vec<Vec<bool>> = vec![vec![false;n];n];
  for (i, (ilabel, ivalue)) in Nodes.iter().enumerate(){
    for (j, (jlabel, jvalue)) in Nodes.iter().enumerate(){
      if i==j{
        Matrix[i][j]= true;
        continue;
      }
      else{
        let corr = (ivalue-jvalue).abs();
        if corr <= threshold {
          Edges.entry(ilabel.to_string()).or_insert_with(Vec::new).push(jlabel.to_string());
          Matrix[i][j] = true;
        }
      }
    }
  }
  (Edges, Matrix)
}

fn furthest(Edges: Edges) -> HashMap<String, String>{
  let mut farneigh: HashMap<String, String> = HashMap::new();
  for asset in Edges.keys(){
    let original:String = asset.to_string(); 
    let mut step: String = Edges[&original].choose(&mut rand::thread_rng()).unwrap().to_string();
    let mut current = step.clone();
    let mut visitedneigh: Vec<String> = Vec::new();
    visitedneigh.push(current.clone());
    for _ in 0..20{
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

#[derive(Debug)]
pub struct Graph{
  n:usize, 
  vertices: Nodes, 
  newmap: Edges, 
  newmatrix: Matrix,
}

impl Graph{
  pub fn new(n:usize, vertices: Vec<(String, f64)>, newmap: HashMap<String, Vec<String>>, newmatrix: Vec<Vec<bool>>) -> Self{
    Graph {n, vertices, newmap, newmatrix}
  }

  pub fn undirected(&mut self) -> &Graph{
    self.vertices.sort_by(|a, b| a.0.cmp(&b.0));
    self
  }

  pub fn dailyexpect(&self) -> (Vec<(String, f64)>, Vec<(String, f64)>){
    let (posret, negret): (Vec<_>, Vec<_>) =
      self.vertices.clone()
      .into_iter()
      .partition(|&(_, value)| value >= 0.0);
    (posret, negret)
  }

  pub fn risk(&self, i:usize) {
    let mut distance: Vec<Option<u32>> = vec![None; self.n];
    distance[i] = Some(0);
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(i);
    while let Some(v) = queue.pop_front(){
      for (u, connect) in self.newmatrix[v].iter().enumerate(){
        if *connect && distance[u].is_none(){
            distance[u] = Some(distance[v].unwrap() + 1);
            queue.push_back(u);
          }
      }
    }
    for v in 0..self.n{
      println!("{:?}: {:?}", self.vertices[v].0, distance[v])
    }
  }
  pub fn portfolio(&self){
    for i in 0..self.n{
      println!("Distance from Asset {}", self.vertices[i].0);
      self.risk(i);
    }
  }
  pub fn findcomp(&self, point:usize, component:&mut Vec<Option<usize>>, count:usize){
    component[point] = Some(count); 
    let mut queue = VecDeque::new(); 
    queue.push_back(point);
    while let Some(point) = queue.pop_front(){
      for (connection, &value) in self.newmatrix[point].iter().enumerate(){
        if None ==component[connection] && value{
          component[connection] = Some(count);
          queue.push_back(connection);
        }
      }
    }
  }
  pub fn groups(&self){
    let mut component: Vec<Option<usize>> = vec![None; self.n];
    let mut count = 0;
    for point in 0..self.n{
      if let None = component[point]{
        count +=1;
        self.findcomp(point, &mut component, count);
      }
    }
    println!("{} components:", count);
    for point in 0..self.n{
      println!("{} : {:?}", point, component[point]);
    }
  }
}

fn main() {
  let path = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\project\daily_asset_prices.csv";
  let (_Assetprices, Nodes) = read(path).expect("Couldn't Read!");
  let n = Nodes.len();
  let (adjmap, adjmat) = createadj(Nodes.clone(), 0.1, n);
  let far = furthest(adjmap.clone());
  let graph = Graph::new(n, Nodes.clone(), adjmap.clone(), adjmat.clone());
  let (positive, negative) = graph.dailyexpect();
  //let risk = graph.portfolio();
  graph.groups();
}

#[test]
fn test(){
  let testpath = r"C:\Users\dhanv\OneDrive\Desktop\2023-2024\Spring 24\DS 210\final_proj\final proj test.csv";
  let (Assetstest, Nodestest) = read(testpath).expect("Couldn't Read!");
  let nt = Nodestest.len();
  let (testmap, testmat) = createadj(Nodestest.clone(), 0.1, nt);
  let far_test = furthest(testmap.clone());
  let testgraph = Graph::new(nt, Nodestest.clone(), testmap.clone(), testmat.clone());
  let (testpos, testneg) = testgraph.dailyexpect();
  assert_eq!(testneg.iter().map(|(s, v)| (s.as_str(), *v)).collect::<Vec<_>>(), 
    [("\"11/4/2013\", \"Commodities\"", -0.004334982158052806), ("\"11/4/2013\", \"Emerg Markets Bonds\"", -0.00018130723716086204)]);
  let risk = testgraph.portfolio();
}
