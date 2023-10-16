pub fn find<A,K>(array: A, key: K) -> Option<usize> 
where
    A: AsRef<[K]>,
    K: Ord
{
    if array.as_ref().len() == 0 { return None }
    let mut lo:i32 = 0;
    let mut hi:i32 = (array.as_ref().len()) as i32 - 1;
    while lo <= hi {
        let mi = (lo + (hi-lo)/2) as usize;
        match array.as_ref()[mi].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mi),
            std::cmp::Ordering::Greater => hi = mi as i32 - 1,
            std::cmp::Ordering::Less =>  lo = mi as i32 + 1,
        }
    }
    None
}