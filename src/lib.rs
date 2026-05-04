// Library entry

pub mod colors;
pub mod options;
pub mod utils;
pub mod legend;
pub mod asciigraph;

pub use crate::colors::AnsiColor;
pub use crate::options::{
    CharSet, Config, ZeroLine, Threshold, StatAnnotations, DEFAULT_CHAR_SET, create_char_set
};
pub use crate::asciigraph::{plot, plot_many};
