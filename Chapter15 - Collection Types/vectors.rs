// vectors adalah koleksi yang menyimpan nilai-nilai dari tipe data yang sama dalam urutan tertentu. 
// Vectors memungkinkan kita untuk menyimpan sejumlah nilai yang dapat bertambah atau berkurang selama runtime program kita.

fn main() {

    // let _v:Vec<i32> = Vec::new(); // cara membuat vector baru yang kosong

    // let mut _v:Vec<i32> = Vec::new(); // sedangkan, untuk membuat vector yang bisa diubah

    // let mut _v:Vec<i32> = vec![1,2,3]; // cara membuat vector dengan nilai awal

    // _v.push(5); // menambahkan nilai ke dalam vector
    // _v.push(6);
    // _v.push(7);
    // _v.push(8);
    // _v.push(9);

    // println!("{:?}", _v); // menampilkan isi vector, {:?} digunakan untuk menampilkan debug format

    // Dibawah ini cara mengakses nilai dalam vector
    let _v = vec![1,2,3,4,5];

    let third: &i32 = &_v[2]; // Direct Indexing, kalau indexnya ga ada, program bakal panic

    println!("Nilai ketiga adalah: {}", third);

    // Kemudian ada cara kedua untuk mengakses nilai dalam vector
    let third3: Option<&i32> = _v.get(2); // Menggunakan get method, kalau indexnya ga ada, bakal return none
    match third3 { // fungsi match untuk ngecek apakah ada nilai atau ga
        Some(third3) => println!("Nilai ketiga: {}", third3),
        None => println!("Ga ada nilai ketiga."),
    }
}