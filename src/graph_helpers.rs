use crate::{PriceUpdate, Vertex, Edge};
use std::collections::{HashMap, LinkedList};
use petgraph::Graph;
use petgraph::prelude::NodeIndex;



pub fn vertex_factory_array(price_update: &PriceUpdate) -> [Vertex; 2] {
    let source_vertex = Vertex{
        exchange: price_update.exchange.clone(),
        currency: price_update.source_currency.clone()
    };
    let destination_vertex = Vertex {
        exchange: price_update.exchange.clone(),
        currency: price_update.destination_currency.clone()
    };

    return [source_vertex, destination_vertex];
}

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
