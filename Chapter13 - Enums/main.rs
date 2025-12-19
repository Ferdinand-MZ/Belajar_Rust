// Enum adalah alat versatile yang digunakan untuk mendefinisikan tipe data yang bisa memiliki beberapa varian berbeda.
// Setiap varian dalam enum bisa memiliki data yang berbeda, memungkinkan kita untuk mengelompokkan nilai-nilai yang terkait di bawah satu tipe yang sama.

fn main(){

    enum IpAddrKind{
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(_ip_kind: IpAddrKind){}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Menggunakan Structs untuk menyimpan data terkait dengan varian enum
    // enum IpAddrKind {
    //     V4,
    //     V6
    // }

    // // Menggunakan Struct bersama Enum
    // struct IpAddr{
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // },

    // let loopback = IpAddr{
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // Tanpa Menggunakan Structs
    // enum IpAddr{
    //     V4(String),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // Menggunakan Enhanced Enum dengan tipe data yang berbeda untuk setiap varian
    enum IpAddr{
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

}