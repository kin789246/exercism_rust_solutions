pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    println!("input {:?}", input);
    if input.is_empty() {
        return Some(vec![]);
    }
    
    if input.len() == 1 {
        if input[0].0 != input[0].1 {
            return None;
        }
        return Some(input.to_vec());
    }
    let mut res: Vec<(u8, u8)> = Vec::new();    
    let mut matrix: Vec<((u8, u8),(u8, u8))> = Vec::new();
    input.iter().for_each(|(a, b)| { 
        if *a != *b {
            matrix.push(((*a, *b), (*b, *a))); 
        }
        else {
            matrix.insert(0, ((*a, *b), (*b, *a))); 
        }
    });
    println!("matrix {:?}", matrix);
    for (i, dominoes) in matrix.iter().enumerate() {
        res.push(dominoes.0);
        let mut v = matrix.clone();
        v.remove(i);
        find(v.clone(), &mut res); 
        println!("1st res {:?}", res);
        if res.len() == input.len() && res[0].0 == res.last().unwrap().1 {
            return Some(res);
        }
        else {
            res = Vec::new();
            res.push(dominoes.1);
            find(v, &mut res); 
            println!("2nd res {:?}", res);
            if res.len() == input.len() && res[0].0 == res.last().unwrap().1 {
                return Some(res);
            }
        }
        res = Vec::new();
    }
    
    None
}

fn find(input: Vec<((u8, u8), (u8, u8))>, res: &mut Vec<(u8, u8)>) {
    //println!("res {:?} input {:?}", res, input); 
    if input.is_empty() {
        return;
    }
    let d1 = res.last().unwrap();
    for (i, &d2) in input.iter().enumerate() {
        // println!("loop {i} res {:?} input {:?}", res, input); 
        let mut v = input.clone();
        v.remove(i);
        if d1.1 == d2.0.0 {
            res.push(d2.0);
            // println!("find res {:?}", res); 
            find(v, res);
            return;
        }
        else if d1.1 == d2.1.0 {
            res.push(d2.1);
            // println!("find res {:?}", res); 
            find(v, res);
            return;
        }
    }
}
