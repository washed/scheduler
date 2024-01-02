pub mod scheduler;
pub mod tests;
pub mod trigger;

pub fn increment(x: u32) -> u32 {
    x + 1
}

#[macro_use]
pub mod job;
