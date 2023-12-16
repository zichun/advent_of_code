pub mod grid;
pub mod input;
pub mod algo;

pub mod prelude {
    pub use crate::grid::*;
    pub use crate::input::*;
    pub use crate::algo::*;

    pub use num;
}
