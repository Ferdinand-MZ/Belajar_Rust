// Hash Map adalah koleksi tipe data yang menyimpan pasangan kunci-nilai (key-value pairs).
// Hash Map memungkinkan kita untuk mengasosiasikan nilai tertentu dengan kunci unik, sehingga kita dapat dengan mudah mengambil nilai tersebut menggunakan kuncinya.
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); // membuat hash map baru yang kosong

    scores.insert(String::from("Blue"), 10); // menambahkan pasangan kunci-nilai ke dalam hash map
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue"); // mendefinisikan kunci yang ingin kita cari nilainya
    let score = scores.get(&team_name).copied().unwrap_or(0); // mengambil nilai berdasarkan kunci, jika kunci tidak ditemukan, mengembalikan 0.
    // copied() digunakan untuk menyalin nilai dari referensi yang dikembalikan oleh get()
    // unwrap_or(0) digunakan untuk memberikan nilai default 0 jika get() mengembalikan None

    for (key, value) in &scores { // iterasi melalui setiap pasangan kunci-nilai dalam hash map
        println!("{key}: {value}");
    }
}