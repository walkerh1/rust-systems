#![allow(dead_code, unused_imports, invalid_nan_comparisons)]

use std::mem::transmute;

// Data type determines what value a sequence of bits represents:
// `a` and `b` have the same bit pattern, but represent different
// values because of their different types. The type indicates which 
// encoding should be used to map between bit strings and chars.
pub fn int_vs_int() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

// ints and floating points are also just different encodings of bit
// patterns: here an f32 bit pattern gets copied and interpreted as a u32,
// resulting in a different value.
pub fn f32_as_u32() {
    let a: f32 = 42.42;
    let b: u32 = unsafe { std::mem::transmute(a) };
    
    println!("{:032b}", b);
    println!("{}", b);

    let c = unsafe { std::mem::transmute(b) };
    println!("{}", c);
    assert_eq!(a, c);
}

// Signed integers (i8, i16, i32, i64) are represented internally using TWOS
// COMPLEMENT: a method for encoding bit patterns as negative integers without
// requiring an explicit sign bit. To get the negation of a signed int take
// its ones complement (i.e. flip every bit) then increment. Another property of
// twos complement is that the first bit of a negative number is always set
// (even though it is not an explicit sign bit). This can be useful way of quickly
// testing whether a bit pattern represents a negative or positive integer.
pub fn twos_complement() {
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
pub fn int_overflow() {
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
pub fn endianness() {
    let c: u32 = 0xAABBCCDD;
    println!("big end -> AA BB CC DD <- little end");
    println!("big endian:    {:?}", c.to_be_bytes());
    println!("little endian: {:?}", c.to_le_bytes());
}

// When represented in scientific notation FLOATING POINT NUMBERS, like 2.498 x 10^18
// have 4 components: the SIGN, which indicates whether it is positive or
// negative; the MANTISSA, which is the value (here 2.498); the RADIX, which is the
// base (here 10); and the EXPONENT, which is the value that the RADIX is raised to
// (here 18). So in a system floating points can be represented as a container with
// 3 fields: a sign bit, a mantissa, and an exponent (radix is always 2, so no need to encode
// that). For an f32, the first bit is the sign bit, the subsequent 8 bits represent the
// exponent, and the remaining 23 bits represent the mantissa.
pub fn floating_point_deconstruction(n: f32) {
    // reinterpret f32 as 32 bits
    let n_bits: u32 = n.to_bits();

    // separate 32 bits of f32 into its components:
    let sign_ = (n_bits >> 31) & 1;         // shift 31 bits then 1-bit AND mask
    let exponent_ = (n_bits >> 23) & 0xff;  // shift 23 bits then 8-bit AND mask
    let fraction = n_bits & 0x7fffff;      // 23-bit AND mask

    // decode sign bit by mapping 0 to -1.0 and 1 to 1.0
    let sign = (-1.0_f32).powf(sign_ as f32);
    
    // decode exponent by subtracting the bias and raising it to the power of
    // the radix, which is 2.
    let exponent = (exponent_ as i32) - 127;
    let exponent = 2_f32.powf(exponent as f32);
    
    // decode the mantissa by multiplying each bit by its weight and summing the result;
    // the first bit's weight is 2^-1, the second is 2^-2, and so on down to 2^-23, halving
    // for each bit.
    let mut mantissa: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    println!("field    | as bits   | as real number");
    println!("sign     | {:01b}         | {}", sign_, sign);
    println!("exponent | {:08b}  | {}", exponent_, exponent);
    println!("mantissa | {:023b} | {}", fraction, mantissa);

}

// In Rust, f64 and f32 only implement the PartialEq trait and not Eq,
// as these types include values for which == is not mathematically valid.
pub fn floating_point_partial_eq() {
    // Some f32/f64 values with different bit patterns are treated as equal:
    let m: f32 = -0.0;
    let n: f32 = 0.0;
    assert!(m == n);
    println!("0.0 == -0.0\n   {:032b}\n== {:032b}", m.to_bits(), n.to_bits());

    // And some with the same bit pattern are treated as unequal:
    assert!(f32::NAN != f32::NAN);
    println!("\nNAN != NAN\n   {:032b}\n!= {:032b}", f32::NAN.to_bits(), f32::NAN.to_bits());
    // (though many different bit patterns count as NAN, no two NANs are
    // ever equal, even if they really do have the same bit pattern, which is
    // what is being shown here.)
}