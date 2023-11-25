// Hierarchy of kinds of pointer (from simpler to more sophisticated):
// 1. A MEMORY ADDRESS, or just ADDRESS, is a number of size usize (typically 64 bits)
//    that is interpreted as referring to a single byte in memory.
// 2. A RAW POINTER, or just POINTER, is a memory address that points to a value of
//    some type.
// 3. A REFERENCE is a raw pointer (or in the case of dynamically sized types, a pointer
//    and integer with extra guarantees) with the following advantages over raw pointers:
//    - references always point to valid data (this is what the borrow checker guarantees).
//    - references are correctly aligned to multiples of usize (by applying padding).
//    - references provide these guarantees to dynamically sized types as well.

use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 194, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn references_and_memory() {
    let a: usize     = 42;
    
    let b: &[u8; 10] = &B;

    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} bytes", size_of::<usize>());
    println!("  value:    {:?}", a);
    println!();
    
    println!("b (a reference to B):");
    println!("  location:  {:p}", &b);
    println!("  size:      {:?} bytes", size_of::<&[u8; 10]>());
    println!("  points to: {:p}", b);
    println!();
   
    println!("c (a 'box' for C):");
    println!("  location:  {:p}", &c);
    println!("  size:      {:?} bytes", size_of::<Box<[u8]>>());
    println!("  points to: {:p}", c);
    println!();
    
    println!("B (an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size:     {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:    {:?}", B);
    println!();
    
    println!("C (an array of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size:     {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:    {:?}", C);
}

// RAW POINTERS
// The difference between immutable raw pointers (*const T) and mutable ones (*mut T)
// is really just for the developer (each can easily be cast to the other). Also, rust
// references compile down to raw pointers, so you often get the performance advantages
// of raw pointers without having to use them (and go unsafe).
//
// From the compiler's perspective, a raw pointer to a value in memory always points
// to where the first byte of that value is stored, and it always knows the type of the
// value, meaning it knows its size (i.e. the number of bytes of memory after the start
// address the value uses up).
//
// The process of fetching data from RAM via a pointer is called dereferencing a pointer.
//
pub fn raw_pointers() {
    let a: i64 = 42;
    
    // turn a reference into a raw pointer
    let a_ptr = &a as *const i64; 
    
    // turn a raw pointer into an address
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}