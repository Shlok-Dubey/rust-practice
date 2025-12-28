/* we have to intialise with the data type in RUST just like C++ 
 * all vaiables are immutalbe(Constant) in rust
 *
 * types of numbers(INT):-
 *  signed :-
 *      i8  --> -128 to 127
 *      i16 --> -32768 to 32767
 *      i32 --> −2147483648 to 2147483647
 *      i64 --> −9223372036854775808 to 9223372036854775807
 *      i128--> −2¹²⁷ to 2¹²⁷ − 1
 *
 *  unsigned :-
 *      u8  --> 0 to 255
 *      u16 --> 0 to 65535
 *      u32 --> 0 to 4294967295
 *      u64 --> 0 to 18446744073709551615
 *      u128--> 0 to 2¹²⁸ − 1
 *
 * 
 * types of floats:-
 *      f32 --> ±1.18×10⁻³⁸ to ±3.4×10³⁸
 *      f64 --> ±2.23×10⁻³⁰⁸ to ±1.8×10³⁰⁸
 * 
 * Booleans
 *      true or false
 *
 * */

fn main() {
    let x = 5 ; // by default it is i32
    let mut y : i8 = 55 ; // we can make it mutable by writing "mut"
    let z : bool = false
    print!{"x : {} , y : {}", x , y}
}
