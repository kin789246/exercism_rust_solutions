/// Type implementing arbitrary-precision decimal arithmetic
use std::ops::{ Add, Sub, Mul };
use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
pub struct Decimal {
    negative: bool,
    data: Vec<u8>, // integer + decimal point from small to large
    point: usize  // digits of point
}

impl Decimal {
    pub fn new() -> Self {
        Self { negative: false, data: Vec::new(), point: 0 }
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut res: Decimal = Decimal::new();
        let mut start = 0;
        if input.starts_with('-') {
            res.negative = true;
            start = 1;
        }
        else if input.starts_with('+') {
            res.negative = false;
            start = 1;
        }
        input[start..].chars().rev().enumerate().for_each(|(i, c)|{
            if c == '.' {
                res.point = i;
                return;
            }
            res.data.push(c.to_digit(10).unwrap() as u8);
        }); 
        res.clear_zero();
        // println!("try_form {:?}", res);
        Some(res)
    }

    // e.g. 0012.4340 => [4, 3, 4, 2, 1] point=3
    pub fn clear_zero(&mut self) {
        while let Some(&v) = self.data.last() {
            if v != 0 || self.data.len() - self.point <= 1 {
                break;
            }
            self.data.pop();
        }
        // proceed with decimal point 
        while let Some(&v) = self.data.first() {
            if v != 0 || self.point < 1 {
                break;
            }
            self.data.remove(0);
            self.point -= 1;
        }
    }

    pub fn flip_signed(self) -> Decimal {
        Self { 
            negative: !self.negative,
            data: self.data.clone(),
            point: self.point
        }
    }

    pub fn make_digit_pairs(&self, other: &Self) -> Vec<(u8, u8)> {
        let mut result: Vec<(u8, u8)> = Vec::new();
        let a: Vec<u8>;
        let b: Vec<u8>;
        if self.point < other.point {
            a = std::iter::repeat(0)
                .take(other.point - self.point)
                .chain(self.data.clone()).collect();
            b = other.data.clone();
        }
        else {
            a = self.data.clone();
            b = std::iter::repeat(0)
                .take(self.point - other.point)
                .chain(other.data.clone()).collect();
        }
        // e.g  12.334      =>  00433.21
        //      333.99011   =>  11099.333
        //      [(0, 1), (0, 1), (4, 0), (3, 9), (3, 9), (2, 3), (1, 3), (0, 3)]
        for i in 0..std::cmp::max(a.len(), b.len()) {
            result.push((*(a.get(i).unwrap_or(&0)), *(b.get(i).unwrap_or(&0))));
        }
        result
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = Decimal::new();
        // positive plus positive
        // negative plus negative
        if self.negative == other.negative {
            // println!("pos + pos");
            result.negative = self.negative;
            if self.point > other.point {
                result.point = self.point;
            }
            else {
                result.point = other.point;
            }
            let mut ca = 0u8;
            self.make_digit_pairs(&other)
                .iter()
                .for_each(|v| {
                    let added = ca + v.0 + v.1;
                    result.data.push(added % 10);
                    ca = added / 10;
                });
        }
        // different signed
        else if self.negative && !other.negative {
            // println!("neg + pos");
            result = other.sub(self.flip_signed());
        }
        else {
            // println!("pos + neg");
            result = self.sub(other.flip_signed());
        }
        result.clear_zero();
        result
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Decimal::new();
        // positive minus positive
        // println!("self {:?}, other {:?}", self, other);
        if !self.negative && !other.negative {
            // println!("pos - pos");
            let mut comparison = self.make_digit_pairs(&other);
            
            if other > self {
                comparison = other.make_digit_pairs(&self);
                result.negative = true;
            }
            if self.point > other.point {
                result.point = self.point;
            }
            else {
                result.point = other.point;
            }
            let mut borrow = 0;
            // println!("comparison {:?}", comparison);
            comparison.iter().for_each(|&(a, b)| {
                let subt;
                if (a as i8 - borrow as i8) < b as i8 {
                    subt = a +10 - borrow - b;
                    borrow = 1;
                }
                else {
                    subt = a - borrow - b;
                    borrow = 0;
                }
                result.data.push(subt);
            });
        }
        // negative minus negative
        else if self.negative && other.negative {
            // println!("neg - neg");
            result = other.flip_signed().sub(self.flip_signed());
        }
        // negative minus positive
        else if self.negative && !other.negative {
            // println!("neg - pos");
            result = self.flip_signed().add(other).flip_signed();
        }
        // positive minus negative
        else {
            // println!("pos - neg");
            result = self.add(other.flip_signed());
        }
        result.clear_zero();
        result
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result: Decimal = Decimal::try_from("0").unwrap();
        for (i, b) in rhs.data.iter().enumerate() {
            let mut sum = Decimal::new();
            let mut carry = 0u8;
            for a in self.data.iter() {
                let added = (*b) * (*a) + carry;
                sum.data.push(added % 10);
                sum.data = std::iter::repeat(0).take(i)
                    .chain(sum.data.clone())
                    .collect();
                carry = added / 10;
            }
            if carry != 0 {
                sum.data.push(carry);
            }
            result = result + sum;
        }

        result.negative = self.negative ^ rhs.negative;
        result.point = self.point + rhs.point;
        result.clear_zero();
        result
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if !self.negative && other.negative {
            return Some(Ordering::Greater);
        }
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }
        if !self.negative && !other.negative {
            let comparison = self.make_digit_pairs(&other);
            for v in comparison.iter().rev() {
                if v.0 > v.1 {
                    return Some(Ordering::Greater);
                }
                else if v.0 < v.1 {
                    return Some(Ordering::Less);
                }
            }
            return Some(Ordering::Equal);
        }
        else {
            let comparison = self.make_digit_pairs(&other);
            for v in comparison.iter().rev() {
                if v.0 > v.1 {
                    return Some(Ordering::Less);
                }
                else if v.0 < v.1 {
                    return Some(Ordering::Greater);
                }
            }
            return Some(Ordering::Equal);
        }
    }
}
