/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

use crate::{Vertex, DEBUG};
use matrix::prelude::Compressed;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

pub fn get_index_from_vertex(
    v: &Vertex,
    vertex_data: &Vec<Vertex>,
    vertex_index: &Vec<NodeIndex>,
) -> Option<NodeIndex> {
    let ind = vertex_data.iter().position(|x| x.eq(v));
    //    return vertex_index[ind]?;
    match ind {
        None => {
            //            println!("Get index from vertex not found.");
            return None;
        }
        Some(i) => {
            //            println!("Get index found: {}", ind.unwrap());
            return Some(vertex_index[i]);
            //            return ind.and_then(|ind| Option::from(vertex_index[ind]));
        }
    }
}

pub fn get_path(u: usize, v: usize, next: Compressed<usize>) -> Option<Vec<usize>> {
    //    println!("get_path received {} and {}", u, v);
    let mut path: Vec<usize> = Vec::new();
    path.push(u);
    //    println!("{:?}", path.get(u));
    let mut u = u;
    while u != v {
        //        println!("{}, {}", u, v);
        u = next.get((u, v));
        path.push(u);
    }
    //    println!("{}, {}", u, v);
    //    println!("LEN: {}", path.len());
    return Some(path);
}

/*
Formats as "<exchange> <currency>" to be used as vertex weights.
Doing this mainly to make the graph easier to keep track of.
*/
pub fn vertex_string_format(v: &Vertex) -> String {
    let node_str = format!("{} {}", &v.exchange, &v.currency);
    if DEBUG {
        println!("{}", &node_str);
    }
    return node_str;
}

pub fn modified_floyd_warshall(
    rate: &Compressed<f32>,
    next: &Compressed<usize>,
    graph: &Graph<String, f32>,
) -> (Compressed<f32>, Compressed<usize>) {
    let mut rate_out = rate.clone();
    let mut next_out = next.clone();

    for k in 0..graph.node_count() {
        for i in 0..graph.node_count() {
            for j in 0..graph.node_count() {
                let u = Decimal::from_f32(rate_out.get((i, j)));
                let a = Decimal::from_f32(rate_out.get((i, k)));
                let b = Decimal::from_f32(rate_out.get((k, j)));

                let res = a.and_then(|a| b.and_then(|b| a.checked_mul(b)));

                if let Some(true) = u.and_then(|u| res.map(|res| u < res)) {
                    let x = res.and_then(|res| Decimal::to_f32(&res));

                    // Set rate and next.
                    x.map(|x| rate_out.set((i, j), x));
                    next_out.set((i, j), next.get((i, k)));
                };
            }
        }
    }
    return (rate_out, next_out);
}

//pub fn add_vertex_to_graph(v: &Vertex, g: &Graph<String, f32>) -> Graph<String, f32> {
//    let node_str = vertex_string_format(&vertex_source);
//}
