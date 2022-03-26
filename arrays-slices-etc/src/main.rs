fn main() {
    let arr = [0, 1, 2, 3];
    let slice = &arr[1 .. 3]; //index 1 is inclusive and 3 is exclusive. Hence slice = [1, 2]
    borrowing_slice(arr, slice);
}

pub fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}",arr);
    println!("{:?}",slice);
    println!("{:?}", slice.len());
    println!("{} {}",slice[0], slice[1]);
}
