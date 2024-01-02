pub mod scheduler;
pub mod tests;
pub mod trigger;

#[macro_use]
pub mod job;

pub fn increment(x: u32) -> u32 {
    x + 1
}
