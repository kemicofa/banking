use infrastructure::Container;
use infrastructure::transport_layer::cli::run;

pub mod application;
pub mod infrastructure;

pub fn main() {
    let container = Container::new();
    run(container);
}
