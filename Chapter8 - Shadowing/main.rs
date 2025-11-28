// Shadowing adalah fitur di Rust yang memungkinkan kita untuk mendeklarasikan ulang variabel dengan nama yang sama.
// Ketika kita mendeklarasikan ulang variabel, variabel yang lama akan "tersembunyi" (shadowed) oleh variabel yang baru.
// Shadowing tidak sama dengan mutability; dengan shadowing, kita bisa mengubah tipe data variabel juga.

fn main() {
    let x = 5;

    let x = x + 1; // x yang baru shadowing x yang lama

    {
        let x = x * 2; // x yang baru di dalam scope ini shadowing x yang di luar scope
        println!("Nilai x di dalam scope adalah: {}", x); // Akan mencetak 12
    }

    println!("Nilai x di fungsi utama adalah: {}", x); // Akan mencetak 6
}