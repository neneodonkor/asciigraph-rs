// Library entry

pub mod color;
pub mod options;
pub mod utils;
pub mod legend;
pub mod asciigraph;

pub use color::AnsiColor;
pub use options::{CharSet, Config};
pub use asciigraph::{plot, plot_many};