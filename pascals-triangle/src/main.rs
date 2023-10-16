fn main() {
    fn formula1(n: u32, k: u32) -> u32 {
        (1..=n).product::<u32>() / ((1..=k).product::<u32>()*(1..=(n-k)).product::<u32>())
    }
    let x = formula1(0, 0);
    println!("{x}");
}