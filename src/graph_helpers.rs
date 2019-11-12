use crate::{PriceUpdate, Vertex, Edge};
use std::collections::{HashMap, LinkedList};

pub struct Graph {
    vertices: Vec<VertexData>,
    edges: Vec<EdgeData>
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = VertexIndex;

    fn next(&mut self) -> Option<VertexIndex> {
            match self.current_edge_index {
                None => None,
                Some(edge_num) => {
                    let edge = &self.graph.edges[edge_num];
                    self.current_edge_index = edge.next_outgoing_edge;
                    Some(edge.target)
            }
        }
    }
}

impl Graph {
    pub fn add_node(&mut self) -> VertexIndex {
        let index = self.vertices.len();
        self.vertices.push(VertexData {first_outgoing_edge: None});
        index
    }

    pub fn add_edge(&mut self, source: VertexIndex, target: VertexIndex) {
        let edge_index = self.edges.len();
        let vertex_data = &mut self.vertices[source];
        self.edges.push(EdgeData{
            target: target,
            next_outgoing_edge: vertex_data.first_outgoing_edge
        });
        vertex_data.first_outgoing_edge = Option::from(edge_index);
    }

    pub fn successors(&self, source: VertexIndex) -> Successors {
        let first_outgoing_edge = self.vertices[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
}


pub type VertexIndex = usize;

pub struct VertexData {
    first_outgoing_edge: Option<EdgeIndex>
}

pub type EdgeIndex = usize;

pub struct EdgeData {
    target: VertexIndex,
    next_outgoing_edge: Option<EdgeIndex>
}


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