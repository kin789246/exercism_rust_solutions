pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    use std::collections::HashMap;
    let students: Vec<&str> = 
        vec!["Alice", "Bob", "Charlie", "David",
             "Eve", "Fred", "Ginny", "Harriet", 
             "Ileana", "Joseph", "Kincaid", "Larry"];
    let seeds = 
        HashMap::from([('G', "grass"),
                       ('C', "clover"), 
                       ('R', "radishes"),
                       ('V', "violets")
                      ]);
    let s = students.iter().position(|x| *x == _student).unwrap()*2;
    let e = s + 1;
    let mut r: Vec<&str> = Vec::new();
    for row in _diagram.split('\n') {
        let p = seeds.get(&(row.chars().nth(s).unwrap())).unwrap();
        r.push(p);
        let p = seeds.get(&(row.chars().nth(e).unwrap())).unwrap();
        r.push(p);
    }
    r
}
