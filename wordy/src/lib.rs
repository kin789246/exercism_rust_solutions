pub fn answer(command: &str) -> Option<i32> {
    if command.find("What is") != Some(0) {
        return None;
    }

    let mut words: Vec<&str> = command.split(' ').collect();
    if words.len() < 3 {
        println!("yyy");
        return None;
    }

    if let Some(w) = words.last_mut() {
        if w.find("?") == Some(w.len()-1) {
            if let Some(ww) = w.strip_suffix("?") {
                if let Ok(_num) = ww.parse::<i32>() {
                    *w = ww;
                }
                else {
                    return None;
                }
            }
        }
    }
    //println!("{:?}", words);
    if let Ok(mut ans) = words[2].parse::<i32>() {
        //println!("{:?}", ans);
        let mut i = 3;
        while i < words.len() {
            //println!("{:?}, i={:?}", words[i], i);
                match words[i] {
                    "plus" => {
                        if let Ok(n) = words[i+1].parse::<i32>() {
                            ans += n; 
                            i += 2;
                        }
                        else { return None; }
                    },
                    "minus" => {
                        if let Ok(n) = words[i+1].parse::<i32>() {
                            ans -= n; 
                            i += 2;
                        }
                        else { return None; }
                    },
                    "multiplied" => {
                        if words[i+1] == "by" {
                            if let Ok(nn) = words[i+2].parse::<i32>() {
                                ans *= nn;
                                i += 3;
                            }
                        }
                        else { return None; }
                    },
                    "divided" => {
                        if let Ok(nn) = words[i+2].parse::<i32>() {
                            ans /= nn;
                            i += 3;
                        }
                        else { return None; }
                    },
                    _ => return None
                }
        }
        return Some(ans);
    }        
    None
}
