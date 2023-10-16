pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{n} make?")
    let mut s = String::new();
    if n%3 == 0 {
        s.push_str(&String::from("Pling"));
    }
    if n%5 == 0 {
        s.push_str(&String::from("Plang"));
    }
    if n%7 == 0 {
        s.push_str(&String::from("Plong"));    
    }
    if n%3 != 0 && n%5 != 0 && n%7 != 0 {  
        s.push_str(&n.to_string())
    }
    s
}
