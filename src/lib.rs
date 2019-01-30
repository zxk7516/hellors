pub mod futures;
pub mod traits;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub use traits::gi32;
pub use traits::gi64;
