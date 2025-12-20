// Error Handling merupakan konsep penting dalam pemrograman yang memungkinkan kita untuk menangani situasi tak terduga atau kesalahan yang mungkin terjadi selama eksekusi program.
// Rust menyediakan beberapa mekanisme untuk menangani error, termasuk penggunaan tipe Result dan panic!.

// Approach 1, yang kita gunakan saat ini
    // enum Option<T> { //option bakal mengecek sesuatu
    //     Some(T), // kalau sesuatu itu positif, bakal ke return dengan some dengan tipe data t
    //     None, // ini kalau ga ada, yang di return none
    // }

fn divide (numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        Option::None 
    } else {
        Option::Some(numerator / denominator)
    }
}

fn main() {
    
    let result = divide(10.0, 2.0);

    match result{
        Some(x) => println!("result: {}", x),
        None => println!("Ga bisa dibagi dengan 0 !"),
    }
}