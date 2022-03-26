fn main() {

    // Two types of variables:
    // 1. Scalar Types: Unsigned & Signed Integer, float, char (letter and emoji)
    // 2. Compound Types: Arrays and tuples

    // 1. Scalar types:

    let a = 10;
    let b = 15;
    println!("Hello, world!, {} {}",a,b);

    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 10+100;

    // signed integer
    // i8, i16, i32, i64, i128
    let signed: i8 = 10;

    // float is used for decimals
    let float: f32 = 1.2;

    println!("The variables are: unsigned - {}, signed - {}, float - {}", unsigned, signed, float);
    
    // char - can only be
    let letter = "c";
    let emoji = "\u{1F600}"; // :Default

    println!("Letter - {}, emoji - {}", letter, emoji);
    
    let is_true: bool = true && false || true ;

    println!("isTrue: {}",is_true);


    // 2. Vector Types:
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    // print structure of array and other objects
    println!("{:?}",other_arr);

    //tuples - 
    let tuple: (u8,bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);

    println!("first {}, second{}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}",tuple2);

    let (a, b, c) = tuple;

    //destructuring
    println!("first {}, second {}, third {}", a, b, c);


}
