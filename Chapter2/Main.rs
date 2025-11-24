// Compound Data Types
// Arrays, Tuples, Slices dan Strings (Slice String)

// Arrays (any array should contain the same data type inside it (string with string, integer with integer, etc))
fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array ke-1: {}", fruits[0]);



// Tuples (a tuple can contain different data types inside it with fixed length)
// "Alice" bukanlah string, tetapi string slice (&str). Untuk membuatnya menjadi String, kita perlu menggunakan to_string() atau String::from()

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

// Slices (slice adalah referensi ke bagian dari array)
// (Slices let you reference a contiguous sequence of elements in a collection. A slice is a kind of reference, so it does not have ownership.)
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Dog", "Cat", "Bird"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Dune".to_string(), &"1984".to_string()];
    println!("Book Slices: {:?}", book_slices);
    
// Strings vs String Slices
// Strings itu tipe data yang dinamis dan dapat diubah ukurannya, sedangkan string slices 
    let mut stone_cold: String = String::from("Hell, "); // mut digunakan untuk membuat variabel dapat diubah, karena defaultnya sendiri immutable di rust untuk semua variabel
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");

// String slices (&str) adalah referensi ke bagian dari string yang sudah ada dan bersifat tidak dapat diubah (immutable).
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5]; // Mengambil slice dari indeks 0 sampai 5 (tidak termasuk indeks 5)
    println!("String Slice: {}", slice);
}

// rust akan membersihkan memori secara otomatis ketika variabel keluar dari scope (tidak digunakan lagi), jadi ketika dikeluarkan dari fungsi main, semua variabel akan dihapus dari memori.



