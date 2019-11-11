use crate::{PriceUpdate, Vertex};
use std::collections::{HashMap, LinkedList};


pub fn vertex_factory(price_update: PriceUpdate) -> [Vertex;2] {
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

//pub fn edge_factory(input_list: LinkedList<Vertex>) -> HashMap<Vertex, Vertex> {
//    let edges: HashMap<Vertex, Vertex>;
//    let input_list_copy = inputlist.clone();
//    while input_list.len() > 0 {
//        let abc: Vertex = input_list_copy.pop_back();
//        edges.insert()
//    }
//    return edges;
//
//}
//
//pub fn