/// What should the type of _function be?
pub fn map<F, A, B>(input: Vec<A>, mut _function: F) -> Vec<B> where 
F: FnMut(A) -> B {
    let mut r = Vec::new();
    for x in input {
        r.push(_function(x));
    }
    r
}
