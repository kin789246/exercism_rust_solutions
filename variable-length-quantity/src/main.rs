fn main() {
    let a = variable_length_quantity::to_bytes(&[0x1000_0000]);
    println!("{:?}", a);
    // let a: u32 = 0x4000;
    // /*
    //     1010 (10)
    //     a[0] = 0
    //     a[1] -> 0101 & 1 -> 0001 = 1
    //     a[2] -> 0010 & 1 -> 0000 = 0
    //     a[3] -> 0001 & 1 -> 0001 = 1
    //  */
    // let mut arr: Vec<u8> = Vec::new();
    // for n in 0..32 {
    //     arr.push((a>>n & 1).try_into().unwrap());
    // }
    // println!("{:?}", arr);
}