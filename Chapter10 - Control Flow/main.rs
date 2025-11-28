
#![allow(warnings)] // Menonaktifkan peringatan pada kode
// If Else Expression
fn main() {
    let age : u16 = 18;
    if age < 18 {
        println!("Kamu masih di bawah umur.");
    } else if age == 18 {
        println!("Kamu baru saja menjadi dewasa.");
    } else {
        println!("Kamu sudah dewasa.");
    }

    // using if in let statement
    let condition = true;
    let number = if condition {5} 
    // else {"six"} // ini akan error karena tipe data harus sama dengan tipe data kondisi sebelumnya.
    else {6}; // karena if else mengembalikan nilai, kita bisa menggunakannya dalam let statement.
    println!("The value of number is: {}", number); // Output: The value of number is: 5 karena condition bernilai true, jika false pasti akan menghasilkan 6.
}