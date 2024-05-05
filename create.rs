use std::{error::Error};
use std::fs::File;
use std::collections::HashMap;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;
pub type Nodes = Vec<(String, f64)>;
pub type Assetprices = HashMap<String, Vec<f64>>;
pub type Edges = HashMap<String, Vec<String>>;
pub type Matrix = Vec<Vec<bool>>;

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

  pub fn dailyexpect(&self, thres: f64) -> (Vec<(String, f64)>, Vec<(String, f64)>){
    let (posret, negret): (Vec<_>, Vec<_>) =
      self.vertices.clone()
      .into_iter()
      .partition(|&(_, value)| value >= thres);
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
    let i =2;
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
