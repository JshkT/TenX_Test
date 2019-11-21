/* Written by Joshua Tan in 2019
* For TenX Technical Exercise
* (The Exchange Rate Path Problem)
*/

use crate::graph_helpers::{get_index_from_node, graph_contains};
use crate::{ExchangeRateRequest, Vertex, DEBUG};

use matrix::format::compressed::Variant;
use matrix::prelude::Compressed;
use matrix::Matrix;
use petgraph::graph::node_index;
use petgraph::Graph;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

//==============MODIFIED FLOYD-WARSHALL======================
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
                println!("u: {:?}, a: {:?}, b:  {:?}", u, a, b);

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

pub fn get_path_from_request(
    rate_request: &ExchangeRateRequest,
    next: &Compressed<usize>,
    graph: &Graph<String, f32>,
) -> Option<Vec<usize>> {
    let rate_request = rate_request.clone();
    let source_vertex = Vertex {
        exchange: rate_request.source_exchange,
        currency: rate_request.source_currency,
    };
    let dest_vertex = Vertex {
        exchange: rate_request.destination_exchange,
        currency: rate_request.destination_currency,
    };

    let u = get_index_from_node(&source_vertex, &graph);
    let v = get_index_from_node(&dest_vertex, &graph);

    let path = u.and_then(|u| v.and_then(|v| get_path_from_index(u, v, next)));
    return path;
}

pub fn get_path_from_index(u: usize, v: usize, next: &Compressed<usize>) -> Option<Vec<usize>> {
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

//Builds Rate lookup table.
pub fn make_best_rate_table(graph: &Graph<String, f32>) -> Compressed<f32> {
    let mut rate: Compressed<f32> = Compressed::zero((graph.node_count(), graph.node_count()));
    for i in 0..graph.node_count() {
        for j in 0..graph.node_count() {
            let x = graph.find_edge(node_index(i), node_index(j));
            match x {
                None => {
                    rate.set((i, j), 0.0);
                }
                Some(e) => {
                    let y = graph.edge_weight(e);
                    y.map(|y| rate.set((i, j), *y));
                }
            }
        }
    }

    // Prints out the initial "rate" lookup table.
    if DEBUG {
        for i in 0..rate.rows {
            for j in 0..rate.columns {
                print!("{} ", rate.get((i, j)));
            }
            print!("\n");
        }
    }
    println!("{:?}", rate);
    return rate;
}

// Creates initial state for the "next" lookup table as specified in the brief.
pub fn make_next_table(graph: &Graph<String, f32>) -> Compressed<usize> {
    let mut next: Compressed<usize> =
        Compressed::new((graph.node_count(), graph.node_count()), Variant::Column);

    for i in 0..graph.node_count() {
        for j in 0..graph.node_count() {
            next.set((i, j), j);
        }
    }

    // Prints out the initial "next" lookup table
    if DEBUG {
        for i in 0..next.rows {
            for j in 0..next.columns {
                print!("{} ", next.get((i, j)));
            }
            print!("\n");
        }
    }

    return next;
}

pub fn get_best_rates(
    rate_request: ExchangeRateRequest,
    rate: &Compressed<f32>,
    graph: &Graph<String, f32>,
) -> Option<f32> {
    let source_vertex = Vertex {
        exchange: rate_request.source_exchange,
        currency: rate_request.source_currency,
    };
    let dest_vertex = Vertex {
        exchange: rate_request.destination_exchange,
        currency: rate_request.destination_currency,
    };

    if graph_contains(&source_vertex, &graph) && graph_contains(&dest_vertex, &graph) {
        //        let source_index = get_index_from_vertex(&source_vertex, &vertex_data, &vertex_index);
        //        let dest_index = get_index_from_vertex(&dest_vertex, &vertex_data, &vertex_index);

        let u = get_index_from_node(&source_vertex, &graph);
        let v = get_index_from_node(&dest_vertex, &graph);

        //        let u = source_index.map(|u1| u1.index());
        //        let v = dest_index.map(|v1| v1.index());
        let best_rate = u.and_then(|u| v.map(|v| rate.get((u, v))));
        return best_rate;
    } else {
        println!("Either Source or Destination does not exist yet.");
        return None;
    }
}
