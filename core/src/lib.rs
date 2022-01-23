mod compare;
mod decoder;
mod types;
mod webp;

pub use compare::*;
pub use decoder::*;
pub use types::*;
pub use webp::*;

pub fn version() -> i32 {
    webp_version()
}
