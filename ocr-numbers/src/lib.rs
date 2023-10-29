// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

fn get_digits(y: usize, possible: &mut HashSet<usize>, line: &[u8; 3]) {
    const DIGITS: [[u8; 12]; 10] = [
        //   _     _  _     _  _  _  _  _ 
        //  | |  | _| _||_||_ |_   ||_||_|
        //  |_|  ||_  _|  | _||_|  ||_| _|
        // 
        [b' ', b'_', b' ', b'|', b' ', b'|', b'|', b'_', b'|', b' ', b' ', b' '], // 0
        [b' ', b' ', b' ', b' ', b' ', b'|', b' ', b' ', b'|', b' ', b' ', b' '], // 1
        [b' ', b'_', b' ', b' ', b'_', b'|', b'|', b'_', b' ', b' ', b' ', b' '], // 2
        [b' ', b'_', b' ', b' ', b'_', b'|', b' ', b'_', b'|', b' ', b' ', b' '], // 3
        [b' ', b' ', b' ', b'|', b'_', b'|', b' ', b' ', b'|', b' ', b' ', b' '], // 4
        [b' ', b'_', b' ', b'|', b'_', b' ', b' ', b'_', b'|', b' ', b' ', b' '], // 5
        [b' ', b'_', b' ', b'|', b'_', b' ', b'|', b'_', b'|', b' ', b' ', b' '], // 6
        [b' ', b'_', b' ', b' ', b' ', b'|', b' ', b' ', b'|', b' ', b' ', b' '], // 7
        [b' ', b'_', b' ', b'|', b'_', b'|', b'|', b'_', b'|', b' ', b' ', b' '], // 8
        [b' ', b'_', b' ', b'|', b'_', b'|', b' ', b'_', b'|', b' ', b' ', b' '], // 9
    ];
   for di in possible.clone().iter() {
        if DIGITS[*di][3*y] != line[0] 
            || DIGITS[*di][3*y+1] != line[1]
            || DIGITS[*di][3*y+2] != line[2]
            {
                possible.remove(di);
            }
    }
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut nums: Vec<HashSet<usize>> = Vec::new();
    match input.split('\n').count() {
        x if x % 4 != 0 => { 
            return Err(Error::InvalidRowCount(x));
        },
        _ => ()
    }
    let mut line_width: Vec<usize> = Vec::new();
    for (r, line) in input.split('\n').enumerate() {
        let mut c: usize = 0;
        let mut block = [0u8, 0u8, 0u8];
        let w = line.len() / 3;
        line_width.push(w);
        if line.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
        loop {
            let col = c / 3;
            for i in 0..3 {
                let ch = line.as_bytes().iter().nth(c).unwrap(); 
                block[i] = *ch;
                c += 1;
            }
            if r%4 == 0 {
                nums.push(HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
            }
            let i = r/4*w + col;
            get_digits(r%4, &mut nums[i], &block);
            if c >= line.len() {
                break;
            }
        }
    }
    let mut ans: String = String::new();
    let mut i: usize = 0;
    for w in line_width.iter() {
        while i < nums.len() {
            if nums[i].len() == 1 {
                ans += &nums[i].iter().nth(0).unwrap().to_string();
            }
            else {
                ans += "?";
            }
            if (i+1) % *w == 0 && i+1 != nums.len() {
                ans += ",";
            }
            i += 1;
        }
    }
    Ok(ans)
}

/*
#[test]
fn for_debug() {
    // let a = [32, 32, 9];
    // let mut p: HashSet<usize> = HashSet::from([1usize, 2usize, 5usize, 4usize]);
    // get_digits(2, &mut p, &a);
    // println!("{:?}", p); 
    #[rustfmt::skip]
    let input = "    _  _ \n".to_string() +
                "  | _| _|\n" +
                "  ||_  _|\n" +
                "         \n" +
                "    _  _ \n" +
                "|_||_ |_ \n" +
                "  | _||_|\n" +
                "         \n" +
                " _  _  _ \n" +
                "  ||_||_|\n" +
                "  ||_| _|\n" +
                "         ";

    // let input = "       _     _        _  _ \n".to_string() +
    //             "  |  || |  || |  |  || || |\n" +
    //             "  |  ||_|  ||_|  |  ||_||_|\n" +
    //             "                           ";

    assert_eq!(Ok("110101100".to_string()), convert(&input));
    assert!(false);
}
*/
// 0= 010 1= 000 2= 010 3= 010 4= 000 5= 010 6= 010 7= 010 8= 010 9= 010
//    101    001    011    011    111    110    110    001    111    111
//    111    001    110    011    001    011    111    001    111    011
//    000    000    000    000    000    000    000    000    000    000
