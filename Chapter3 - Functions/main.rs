// sebuah fungsi/variabel harus ditulis di dalam snake case, yaitu semua huruf kecil dan dipisahkan dengan garis bawah (_)
// contoh : fn hello_world() {}
// jangan memakai kebab case (HelloWorld) atau camel case (helloWorld) untuk penamaan fungsi/variabel di rust

// rust menganut hoisting, jadi fungsi/variabel bisa dipanggil sebelum dideklarasikan (mirip javascript)

// for global variabels, use const or static keyword
// also the variabels must be in upper case with underscore (_)
// example : const MAX_SCORE: u32 = 100;

fn main (){
    hello_world();
    tell_height(180);
    human_id("John", 25, 175.5);

    let _X = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Total Price: { }", _X);
    // add(4, 6);
    let y = add(4, 6);
    println!("Value dari y adalah: {}", y);
    println!("Value dari fungsi 'add' adalah: {}", add(4,6));

    // memanggil fungsi calculate_bmi
    let weight = 44.7; // dalam kilogram
    let height = 1.62; // dalam meter
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, world from hello_world function!");
}

// input value
fn tell_height(height: i32) {
    println!("Your height is: {} cm", height);
}

// insert more than one value
fn human_id(name: &str, age: u8, height: f32) {
    println!("My name is {}, I am {} years old and my height is {} cm.",  name, age, height);
}

// functions returning values
// menggunakan arrow (->) untuk mendefinisikan tipe data yang dikembalikan oleh fungsi
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions dan Statements, Expressions menghasilkan nilai sedangkan Statements tidak menghasilkan nilai
// contoh Expressions :
// 5
// true & false
// add(3,4)
// if condition { "yes" } else { "no" }

// contoh Statements :
// hampir segalanya yang tidak menggunakan semicolon
// let y = let x = 10
// 1 deklarasi variabel : let x = 5;
// 2 deklarasi fungsi : fn hello() {}
// 3 control flow : if, loop, while, for

// Final Example
// BMI = height(kg)/height(m)^2 <- ini rumusnya

fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    weight_kg / (height_m * height_m)
}