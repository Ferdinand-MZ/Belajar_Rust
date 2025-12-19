// ada 3 tipe loop di rust, yaitu loop, while, dan for.
// salah satu kegunaan loop adalah untuk mecoba sebuah operasi yang mungkin gagal, seperti koneksi jaringan atau membaca file.

fn main() {
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1;
    }
}

// Yang terjadi pada kode di atas adalah:
// 1. Loop luar diberi label 'counting_up.
// 2. Ketika remaining mencapai 9, loop dalam dihentikan dengan break.
// 3. Ketika count mencapai 2, loop luar dihentikan dengan break 'counting_up.
// 4. Program akan mencetak nilai count dan remaining sesuai dengan logika di atas hingga loop dihentikan.
// 5. Setelah loop dihentikan, program akan melanjutkan eksekusi setelah loop tersebut.
// Dengan menggunakan label pada loop, kita dapat mengontrol loop mana yang ingin kita hentikan, terutama ketika ada loop bersarang.

// Apa itu label pada loop?
// Label pada loop adalah cara untuk memberi nama pada sebuah loop sehingga kita dapat merujuknya secara spesifik ketika menggunakan perintah break atau continue.
// Label diawali dengan tanda kutip tunggal (') diikuti oleh nama yang kita pilih untuk loop tersebut.
// Contoh penggunaan label pada loop dapat dilihat pada kode di atas, yaitu 'counting_up.