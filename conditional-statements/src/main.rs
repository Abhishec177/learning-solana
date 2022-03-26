fn main() {
    let n = 3;
    if n > 0 {
        println!("greater than 0");
    } else if n < 0 {
        println!("less than 0");
    } else {
        println!("is 0");
    }

    //printing from 0 to 5
    for i in 0..6 {
        println!("{}", i);
    }

    //while loop for i<4:
    let mut j = 0;
    while j < 4 {
        println!("{}", j);
        j += 1;
        if j == 3 {
            println!("exit");
            break 
            // similar to returning a value, the break and continue statements dont end with semicolon.
            // continue also works just the way it is expected
        }
    }

    // match statements:
    let i = 4;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        _ => println!("default")
    }
    
}
