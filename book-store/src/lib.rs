use std::cmp;
pub fn lowest_price(books: &[u32]) -> u32 {
    if books.len() == 0 {
        return 0u32;
    }
    let mut min_price = std::u32::MAX;
    let mut b: Vec<u32> = vec![0, 0, 0, 0, 0, 0]; 
    books.iter().for_each(|x| { 
        b[(*x) as usize] += 1;
    });
    //println!("{:?} --> {:?}", books, b);
    for i in (1..6).rev() {
        if b.iter().filter(|&v| *v > 0).count() < i {
            continue;
        }
        min_price = cmp::min(min_price, price(b.clone(), i));
        //println!("{}", min_price);
    }
    min_price
}

pub fn price(mut b: Vec<u32>, max: usize) -> u32 {
    let mut pri = 0.0;
    let mut d = max;
    let discounts: Vec<f32> = vec![0., 1., 0.95, 0.9, 0.8, 0.75];
    //println!("max = {max}");
    while d > 0 {
        b.sort_by(|a, b| b.cmp(a));
        //println!("d={d}, b={:?} => diffbooks={:?}", b, b.iter().enumerate().find(|(i, &x)| x == 0).unwrap());
        if b[d-1] > 0 {
            let mut x = d;
            let mut cur = b.iter_mut();
            while x > 0 {
                if let Some(v) = cur.next() {
                    println!("v={v}");
                    if *v > 0 {
                        *v -= 1;
                        x -= 1;
                    }
                }
            }
            pri += d as f32 * 800. * discounts[d];
            //println!("d={d} => price={:?}", d as f32*800. * discounts[d]);
            continue;
        }
        d -= 1;
    }
    pri as u32
}
