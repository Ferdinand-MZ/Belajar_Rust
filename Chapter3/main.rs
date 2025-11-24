// sebuah fungsi/variabel harus ditulis di dalam snake case, yaitu semua huruf kecil dan dipisahkan dengan garis bawah (_)
// contoh : fn hello_world() {}
// jangan memakai kebab case (HelloWorld) atau camel case (helloWorld) untuk penamaan fungsi/variabel di rust

// rust menganut hoisting, jadi fungsi/variabel bisa dipanggil sebelum dideklarasikan (mirip javascript)
fn main (){
    hello_world();
}

fn hello_world() {
    println!("Hello, world from hello_world function!");
}