// Library entry

pub mod color;
pub mod options;
pub mod utils;
pub mod legend;
pub mod asciigraph;

pub use crate::color::AnsiColor;
pub use crate::options::{
    CharSet, Config, ZeroLine, Threshold, StatAnnotations, DEFAULT_CHAR_SET, create_char_set
};
pub use crate::asciigraph::{plot, plot_many};
