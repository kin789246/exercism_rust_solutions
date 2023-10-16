use allyourbase::convert;

fn main() {
    let input_base = 2;
    let input_digits = &[1, 1, 0, 1];
    let output_base = 10;
    println!("{:?}", convert(input_digits, input_base, output_base));
}

/*
5 % 2 = 1
2 % 2 = 0
1 % 2 = 1


11 % 2 = 1
5 % 2 = 1
2 % 2 = 0
1 % 2 = 1

11 % 3 = 2
3 % 3 = 0
1 % 3 = 1
2*1 + 0*3 + 1*9 = 11

11 % 5 = 1
2 % 5 = 2
 */