// Hierarchy of kinds of pointer (from simpler to more sophisticated):
// 1. A MEMORY ADDRESS, or just ADDRESS, is a number of size usize (typically 64 bits)
//    that is interpreted as referring to a single byte in memory.
// 2. A RAW POINTER, or just POINTER, is a memory address interpreted as pointing to a
//    value of some type.
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
// value.
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

// The STACK is a region of program memory organised into a LIFO stack data structure consisting of STACK
// FRAMES. A new stack frame is created and pushed onto the bottom of the stack every time a function is
// called (it's pushed onto the bottom instead of the top because the stack grows down in memory). As the
// program progresses, a register in the CPU called the STACK POINTER is updated to reflect the address
// of the current stack frame. As functions are called inside other functions, new frames are pushed onto the
// stack, and so the stack pointer decrease. And as functions return, their corresponding frames are popped
// off the stack, and the stack pointer increases. Frames below the current frame (i.e. the frame being pointed
// at by the stack pointer) cannot be accessed; their state is effectively frozen until the frames above it are
// popped off. Frames can be different sizes. They just need enough space for the function arguments, local
// variables, and a pointer to the original calling function.
//
// However, not all data associated with a function call can be stored on the stack. The stack can only store
// types of data whose size is known at compile time. To store dynamic types of data, like Vec<T> and String,
// which can grow and shrink to arbitrary sizes during runtime, as well dynamically allocated types, like slices
// ([T]) and trait objects, whose size is fixed once allocated, but not known at compile time, the system uses
// another region of program memory called the HEAP.
//
// Suppose a function contains a local variable whose type is Vec<i32>. When this function gets called, enough
// space will be provided in the stack frame for a pointer to type Vec<i32>. Then when the variable is
// initialised the system will find some location in the heap to store the Vec<i32>. Then it will update the
// pointer stored inside the stack frame to hold the address of this location. Dynamic types, like Vec<i32>,
// are initialised with some capacity (you can actually set this in Rust using Vec::with_capacity) and
// that capacity is how much space on the heap it will initially be allocated. Then if the Vec<i32> fills its
// capacity, the system has to find a new location in the with more space, copy over all the data from the old
// location, and update the pointer on the stack to contain the new memory address.
use std::mem::drop;

pub fn stack_and_heap() {
    // Box<T> is a smart pointer that will store T on the heap instead of the stack.
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
}


