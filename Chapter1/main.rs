// primitive data types : int, float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned integer (+ only) integers of various sizes.
// i8, i16, i32, i64, i128, isize (signed integers)
// u8, u16, u32, u64, u128, usize (unsigned integers)

// the difference between isize and usize is that they depend on the architecture of the computer
// (32 bits on 32-bit architecture and 64 bits on 64-bit architecture)

fn main() {
    let a: i32 = -10; // signed 32-bit integer
    let b: u32 = 20;  // unsigned 32-bit integer

    println!("Signed integer a: {}", a);
    println!("Unsigned integer b: {}", b);

    // diff between i32 and i64 is that i64 can store larger values
    // range :
    // i32: -2,147,483,648 to 2,147,483,647 (2 bit → 1 bit untuk tanda (positif/negatif), 31 bit sisanya untuk nilai → range = −2³¹ sampai 2³¹−1.)
    // i64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 (64 bit → 1 bit tanda, 63 bit nilai → range = −2⁶³ sampai 2⁶³−1.)
    
    // kalau dibandingkan antara i32 dan u32, i32 hanyalah -2 pangkat 31 sampai 2 pangkat 31 - 1, sedangakn u32 adalah 0 sampai 2 pangkat 32 - 1.

    // Float [floating data types]
    // hanya ada 2 tipe : f32 dan f64
    let x: f32 = 3.14; // 32-bit floating point
    let y: f64 = 2.718281828459045; // 64-bit floating point
    // f64 lebih presisi dibanding f32, karena f64 menggunakan 64 bit untuk menyimpan nilai desimalnya, sedangkan f32 hanya menggunakan 32 bit.

    println!("Float x: {}", x);
    println!("Float y: {}", y);

    // Boolean (true dan false, yk lah)
    let is_rust_fun: bool = true;
    let is_sky_green: bool = false;

    println!("Is Rust fun? {}", is_rust_fun);
    println!("Is the sky green? {}", is_sky_green);

    // Character
    let letter: char = 'R';
    let emoji: char = '😊';

    println!("Character letter: {}", letter);
    println!("Character emoji: {}", emoji);
}