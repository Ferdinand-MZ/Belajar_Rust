// Jadi? apa itu Structs di Rust?
// Structs adalah cara untuk mengelompokkan beberapa nilai menjadi satu kesatuan yang lebih kompleks.
// Misalnya, kita ingin merepresentasikan sebuah titik dalam ruang 2D dengan koordinat x dan y.
// Kita bisa membuat struct Point yang memiliki dua field: x dan y.

fn main(){
    // tuple
    let rect = (200,500);

    // struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("user123"),
        email: String::from("user123@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@gmail.com");
    // kegunaan ini adalah untuk mengubah nilai field email pada struct user1, jadinya user123@gmail.com menjadi anotheremail@gmail.com
    println!("User email adalah {}", user1.email);

    // Return Struct dari fungsi
    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // Membuat instance baru dari instance yang lain
    let user2 = User{
        email: String::from("another@m.com") // disini anda telah membuat instance baru yaitu user2 dari instance user1, dan disini memakai email user1 hanya saja field emailnya berbeda
        ..user1 //jika ingin menyetel semua field dari instance user1 sama seperti user1
    };

    // Tuples Structs (digunakan untuk membuat struct yang mirip dengan tuple, tapi dengan nama yang berbeda untuk tipe data yang berbeda)
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-Like Structs (ini spesifik, mereka ga punya field apapun dan hanya digunakan ketika kita butuh tipe yang unik tapi ga perlu menyimpan data apapun)
    struct AlwaysEqual;
    let subject = AlwaysEqual;


}