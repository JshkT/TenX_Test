use crate::{PriceUpdate, Vertex};

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

pub fn edge_factory() {

}