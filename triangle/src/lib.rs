use std::ops::Add;
#[derive(Debug)]
pub struct Triangle<T:Add+Copy>(T, T, T);

impl<T> Triangle<T>
    where T:Add + Copy + PartialEq
    {
        pub fn build(sides: [T; 3]) -> Option<Triangle<T>> 
        where <T as Add>::Output: PartialOrd<T> {
            if sides[0]+sides[1] <= sides[2] || sides[0]+sides[2] <= sides[1] || sides[1]+sides[2] <= sides[0] {
                return None
            }
            Some(Triangle(sides[0], sides[1], sides[2]))
        }

        pub fn is_equilateral(&self) -> bool {
            self.0==self.1 && self.1==self.2
        }

        pub fn is_scalene(&self) -> bool {
            !self.is_isosceles()
        }

        pub fn is_isosceles(&self) -> bool {
            self.0==self.1 || self.1==self.2 || self.0==self.2
        }
}
