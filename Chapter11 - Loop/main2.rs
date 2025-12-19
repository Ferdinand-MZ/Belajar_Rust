// while loop adalah loop yang akan terus berjalan selama kondisi bernilai true.

fn main(){
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
        break; //ketika ditambahkan break, maka loop akan berhenti setelah iterasi pertama. karena break menghentikan eksekusi loop secara langsung.
    }
    println!("Yo!!!");

    // Looping through a collection with for loop

    let a = [1,2,3,4,5,6];
    let b = ["a", "b", "c", "d", "e"];

    for element in a{
        println!("the value is: {element}");
    }

    for letter in b{
        println!("the letter is: {letter}");
    }
    // kenapa di a memakai element dan di b memakai letter?
    // karena agar lebih mudah dimengerti apa isi dari array tersebut.
    // jadi letter digunakan untuk array yang berisi string, sedangkan element digunakan untuk array yang berisi angka.
}


// jadi while loop digunakan ketika kita tidak tahu berapa kali loop akan berjalan,
// sedangkan for loop digunakan ketika kita tahu berapa kali loop akan berjalan atau ketika kita ingin mengiterasi elemen-elemen dalam sebuah koleksi seperti array atau vector.