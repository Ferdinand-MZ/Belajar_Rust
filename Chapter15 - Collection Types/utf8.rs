// apa itu utf8?
// UTF-8 (8-bit Unicode Transformation Format) adalah format pengkodean karakter yang digunakan untuk merepresentasikan teks dalam komputer dan sistem komunikasi.
// UTF-8 dirancang untuk kompatibilitas dengan ASCII dan dapat merepresentasikan semua karakter
// dalam standar Unicode. Ini adalah format pengkodean yang paling umum digunakan di web dan banyak sistem lainnya karena efisiensinya dalam menyimpan teks.
// di collection types, string adalah koleksi karakter yang disimpan dalam format UTF-8.

fn main() {
    // let s = "whatever"; // string literal, disimpan di stack, bersifat immutable
    let _s = "ferdi ganteng".to_string(); // mengubah string literal menjadi String type
    let _s = String::from("ferdi ganteng"); // String type, disimpan di heap, bersifat mutable
    let mut s = String::from("ferdi ganteng ");
    s.push_str("banget"); // menambahkan string ke string yang sudah ada
    s.push('!'); // menambahkan karakter ke string yang sudah ada
    println!("{}", s); // menampilkan isi string


    let salam = String::from("salam epep");
    let salut = String::from("hidup epep");

    // kalo mau nge combine strings, bisa pake + operator atau format! macro
    let s1 = String::from("ferdi ");
    let s2 = String::from("ganteng");
    let s3 = s1 + &s2; // s1 udah ga bisa dipake lagi karena ownershipnya udah pindah ke s3

    println!("nilai dari s3 itu adalah: {}", s3);

    // Formatting Settings
    let full_message = format!("{salam} {salut}");
    println!("{full_message}");

}