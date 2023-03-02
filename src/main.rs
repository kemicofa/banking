use infrastructure::Container;
use transport_layer::cli::run;

pub mod application;
pub mod data_sources;
pub mod infrastructure;
pub mod transport_layer;

pub fn main() {
    let container = Container::new();
    run(container);
}
