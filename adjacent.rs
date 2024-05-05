use std::{error::Error};
use std::fs::File;
use std::collections::HashMap;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;
pub type Nodes = Vec<(String, f64)>;
pub type Assetprices = HashMap<String, Vec<f64>>;
pub type Edges = HashMap<String, Vec<String>>;
pub type Matrix = Vec<Vec<bool>>;

pub fn createadj(Nodes: Nodes, threshold:f64, n:usize) -> (Edges, Matrix){
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