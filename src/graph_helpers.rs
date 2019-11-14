use crate::{PriceUpdate, Vertex, Edge};
use std::collections::{HashMap, LinkedList};
use petgraph::Graph;
use petgraph::prelude::NodeIndex;
use matrix::prelude::Compressed;

pub fn get_index_from_vertex(v: &Vertex, vertex_data: &Vec<Vertex>, vertex_index: &Vec<NodeIndex>) -> Option<NodeIndex> {
    let ind = vertex_data.iter().position(|x| x.eq(v));
    match ind {
        None => {
            println!("Get index from vertex not found.");
            return None
        },
        Some(_0) => {
            println!("Get index found: {}", ind.unwrap());
            return Option::from(vertex_index[ind.unwrap()]);
        }
    }

    return None;

}

pub fn get_path(u: usize, v: usize, next: Compressed<usize>) -> Vec<usize> {
//    println!("get_path received {} and {}", u, v);
    let mut path: Vec<usize> = Vec::new();
    path.push(u);
//    println!("{:?}", path.get(u));
    let mut u = u;
    while u != v {
//        println!("{}, {}", u, v);
        u = next.get((u,v));
        path.push(u);
    }
//    println!("{}, {}", u, v);
//    println!("LEN: {}", path.len());
    return path

}
