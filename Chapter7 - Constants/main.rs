fn main(){
    println!("Hello, World!");
    let mut x = 5;
    // const mut y = 10; | constant defaultnya immutable dan akan selalu menjadi immutable, membuatnya menjadi mutable akan menimbulkan error.
    // const juga harus diinisalisasi dengan huruf kapital, dan harus diperjelas tipe datanya apa.

    const Y: u16 = 10;

    println!("Nilai x adalah: {}", x);
    println!("Nilai y adalah: {}", Y);
    println!("Nilai PI adalah: {}", PI);
    println!("Nilai 3 Jam dalam detik adalah: {}", THREE_HOURS_IN_SECONDS);
}


// Anda juga bisa mendeklarasikan constant di luar fungsi main (global scope) karena constant bersifat global secara default.

const PI : f32 = 3.14;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;