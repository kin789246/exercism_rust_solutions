pub fn build_proverb(list: &[&str]) -> String {
    //unimplemented!("build a proverb from this list of items: {list:?}")
    if list.len() == 0 {
        return String::new()
    }
    if list.len() == 1 {
        return String::from(format!("And all for the want of a {}.", list[0]))
    }
    let mut s = String::new();
    for i in 1..list.len() {
        s = s + "For want of a " + list[i-1] + " the " + list[i] + " was lost.\n";
    }
    s = s + "And all for the want of a " + list[0] + ".";
    s
}
