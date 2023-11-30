use crate::memory::printing_virtual_addresses;

pub mod bits;
pub mod cpu;
pub mod memory;

fn main() {
    printing_virtual_addresses();
}
