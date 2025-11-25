// Ownership, Borrowing and References

// Ownership adalah sistem manajemen memori unik di Rust yang memastikan keamanan memori tanpa garbage collector.
// Setiap nilai di Rust memiliki "owner" (pemilik) yang bertanggung jawab atas memori nilai tersebut.
// Ketika owner keluar dari scope, memori yang dimiliki akan otomatis dibebaskan.
// Peraturan :
// 1. Setiap nilai memiliki satu owner.
// 2. Hanya ada satu owner pada satu waktu.
// 3. Ketika owner keluar dari scope, nilai akan di-drop (memori dibebaskan).

// Contoh rule pertama
fn main() {
    let s1 = String::from("hello"); // s1 adalah owner dari String "hello"
    let len = calculate_length(&s1); // Meminjam s1 dengan reference
    println!("The length of '{}' is {}.", s1, len); // s1 masih valid di sini
    println!("Ownership example:");
    ownership_example();
}

fn calculate_length(s: &String) -> usize {
    s.len() // Mengakses panjang String melalui reference
}


// Contoh rule kedua
fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 dipindahkan ke s2, s1 tidak lagi valid

    // println!("{}", s1); // Ini akan menyebabkan error karena s1 sudah tidak valid
    println!("{}", s2); // s2 adalah owner baru dari String "hello"
}

// Contoh rule ketiga
fn scope_example() {
    let sc1 = String::from("RUST");
    let len = scope_calculate(&sc1);
    println!("The length of '{}' is {}.", sc1, len);
} // sc1 keluar dari scope di sini, memori otomatis dibebaskan

fn printlost(s: &string) {
    println!("{}", &sc1);
} // Ini akan menyebabkan error karena sc1 sudah tidak valid

fn scope_calculate(s: &String) -> usize {
    s.len() // Mengakses panjang String melalui reference
}