
    // Approach 2
    // enum Result<T, E> { // result bakal ngecek sesuatu, bisa sukses atau gagal
    //     Ok(T), // kalau sukses, bakal ke return dengan ok dengan tipe data t
    //     Err(E), // kalau gagal, bakal ke return err dengan tipe data e
    // }

fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Tidak dapat dibagi dengan o".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    match divideResult(100.23, 73.98) {
        Ok(result) => println!("Hasil pembagian: {}", result),
        Err(err) => println!("Terjadi kesalahan: {}", err),
    }
}