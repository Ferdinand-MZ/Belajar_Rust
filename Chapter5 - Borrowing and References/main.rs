// References and Borrowing
// Safety and Performance
// Borrowing and References are powerful concepts

// Apa itu Safety? Safety adalah jaminan bahwa program tidak akan melakukan hal-hal yang tidak diinginkan seperti mengakses memori yang tidak valid yang sering
// terjadi di bahasa lain seperti C atau C++. Rust menjamin safety melalui sistem ownership dan borrowing-nya. (null pointer, dangling pointer, data race, buffer overflow)

// Memahami konsep References
// References dan Borrowing itu sama, basically kamu membuat references dengan borrowing dari pemilik aslid dari data tersebut. seperti ownership dimana kita hanya punya
// 1 pemilik data, itulah kenapa kita harus meminjam data tersebut dengan references.
// References juga bisa bersifat mutable dan immutable, mutable dapat di modify sedangkan immutable tidak bisa di modify.

fn main() {
    let mut _x = 5;
    let _r = &mut _x;
    
    *_r += 1; // Saya bertujuan untuk menambahkan 1 ke 5. Ini akan menyebabkan error karena _r adalah reference immutable,
    *_r -= 3;

    println!("Value dari _X adalah: {}", _x);
    // println!("Value dari _X adalah: {}", _r); ini akan menyebabkan error karena _r adalah reference mutable, dan hanya boleh 1 reference mutable pada satu waktu.
    // jadi boleh hanya 1 mutable reference pada satu waktu, atau banyak immutable reference pada satu waktu, tapi tidak boleh keduanya.

    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow untk mengecek balance
    account.check_balance();

    // Mutable borrow untuk withdraw uang
    account.withdraw(50.25);

}

// struct adalah data structure yang memungkinkan kita untuk mengelompokkan beberapa field menjadi satu kesatuan.
struct BankAccount {
    owner: String,
    balance: f64,
}

// Implementasi method untuk struct BankAccount
impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}