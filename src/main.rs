#![allow(dead_code, unused_imports)]

use std::mem::transmute;

fn main() {
    endianness();
}

// Data type determines what value a sequence of bits represents:
// `a` and `b` have the same bit pattern, but represent different
// values because of their different types. The type indicates which 
// encoding should be used to map between bit strings and chars.
fn int_vs_int() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

// ints and floating points are also just different encodings of bit
// patterns: here an f32 bit pattern gets copied and interpreted as a u32,
// resulting in a different value.
fn f32_as_u32() {
    let a: f32 = 42.42;
    let b: u32 = unsafe { std::mem::transmute(a) };
    
    println!("{:032b}", b);
    println!("{}", b);

    let c = unsafe { std::mem::transmute(b) };
    println!("{}", c);
    assert_eq!(a, c);
}

// Signed integers (i8, i32, i64) are handled internally using TWOS
// COMPLEMENT: a method for encoding bit patterns as negative numbers without
// requiring an explicit sign bit. Get the negation of a signed int by taking
// its ones complement (i.e. flipping every bit), then incrementing.
fn twos_complement() {
    let a: i32 = 50513;
    let neg_a = !a + 1;

    println!("a:  {:032b} {}", a, a);
    println!("-a: {:032b} {}", neg_a, neg_a);
}

// Unlike floating point numbers, ints cannot sacrifice precision to 
// extend their range. When an int goes above its upper bound it will
// overflow to its lower bound, and vice versa when it goes below its
// lower bound. This is called INTEGER OVERFLOW. For example, a u16
// can represent values 0 to 65,535 inclusive; going above or below this
// range will loop to the other end. Compile with -O flag (which means 
// optimised), otherwise program panics.
#[allow(arithmetic_overflow)]
fn int_overflow() {
    println!("Example 1: u16");
    let _0: u16 = 0b0000_0000_0000_0000;
    let _1:  u16 = 0b0000_0000_0000_0001;
    let _2:  u16 = 0b0000_0000_0000_0011;
    let _65533:  u16 = 0b1111_1111_1111_1100;
    let _65534:  u16 = 0b1111_1111_1111_1101;
    let _65535:  u16 = 0b1111_1111_1111_1111;
    print!("{}, {}, {}, ... , ", _0, _1, _2);
    println!("{}, {}, {}, ...", _65533, _65534, _65535);
    println!("{} + {} = {}", _1, _65535, _1 + _65535);

    println!("\nExample 2: u8");
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("{} + {} = {}", a, b, c);

    println!("\nExample 3: i8");
    let (a, b): (i8, i8) = (-128, 127);
    println!("{} - 1 = {}", a, a - 1);
    println!("{} + 1 = {}", b, b + 1);
}

// ENDIANNESS refers to the order in which systems represent multibyte sequences
// in memory. Little endian means multibyte sequences are stored from least
// to most significant (that is, the little end of the sequence is stored first).
// Big endian means storing them from most to least significant.
//
// Suppose we wanted to store a 4-byte sequence, AA BB CC DD, in byte-addressed
// memory beginning at address 100. In a little endian system we'd do this by putting
// DD at address 100, CC at address 101, BB at address 102, AA at address 103. In a
// big endian system we would store the 4 bytes in the reverse order, with the AA at
// address 100, BB at address 101, and so on.
//
// Most modern systems are little endian.
fn endianness() {
    let c: u32 = 0xAABBCCDD;
    println!("AA BB CC DD");
    println!("big endian:    {:?}", c.to_be_bytes());
    println!("little endian: {:?}", c.to_le_bytes());
}