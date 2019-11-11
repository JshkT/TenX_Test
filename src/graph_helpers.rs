use crate::{PriceUpdate, Vertex};
use std::collections::{HashMap, LinkedList};


pub fn vertex_factory(price_update: &PriceUpdate) -> LinkedList<Vertex> {
    let source_vertex = Vertex{
        exchange: price_update.exchange.clone(),
        currency: price_update.source_currency.clone()
    };
    let destination_vertex = Vertex {
        exchange: price_update.exchange.clone(),
        currency: price_update.destination_currency.clone()
    };
    let mut list: LinkedList<Vertex> = LinkedList::new();
    list.push_back(source_vertex);
    list.push_back(destination_vertex);
    return list;
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