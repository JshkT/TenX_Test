use crate::{PriceUpdate, vertex};

pub fn vertex_factory(price_update: PriceUpdate) -> [vertex;2] {
    let source_vertex = vertex{
        exchange: price_update.exchange.clone(),
        currency: price_update.source_currency.clone()
    };
    let destination_vertex = vertex {
        exchange: price_update.exchange.clone(),
        currency: price_update.destination_currency.clone()
    };
    return [source_vertex, destination_vertex];
}

pub fn edge_factory() {

}