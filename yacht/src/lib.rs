pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
fn numbers(_dice: Dice, n: u8) -> u8 {
    _dice.iter().filter(|&v| v == &n).count() as u8 * n
}

pub fn score(_dice: Dice, _category: Category) -> u8 {
    use std::collections::HashMap;
    let mut map: HashMap<u8, u8> = HashMap::new();
    _dice.iter().for_each(|&v| {
        map.entry(v).and_modify(|n| *n += 1).or_insert(1);
    });
    
    match _category {
        Category::Ones => numbers(_dice, 1),
        Category::Twos => numbers(_dice, 2),
        Category::Threes => numbers(_dice, 3),
        Category::Fours => numbers(_dice, 4),
        Category::Fives => numbers(_dice, 5),
        Category::Sixes => numbers(_dice, 6),
        Category::FullHouse => {
            if map.keys().count() == 2 && map.values().all(|v| *v >=2) {
                return _dice.iter().sum()
            }
            else {
                return 0;
            }
        },
        Category::FourOfAKind => {
            let mut score: u8 = 0;
            if map.values().any(|v| *v >= 4) {
                for v in map.iter() {
                    if v.1 >= &4 {
                        score = v.0 * 4;
                        break;
                    }
                }
                return score;
            }
            return score;
        },
        Category::LittleStraight => {
            let mut sorted = _dice.to_vec();
            sorted.sort();
            for i in 0..4 {
                if i == 0 && sorted[i] != 1 {
                    return 0;
                }
                if sorted[i] + 1 != sorted[i+1] {
                    return 0;
                }
            }
            return 30;
        },
        Category::BigStraight => {
            let mut sorted = _dice.to_vec();
            sorted.sort();
            for i in 0..4 {
                if i == 0 && sorted[i] != 2 {
                    return 0;
                }
                if sorted[i] + 1 != sorted[i+1] {
                    return 0;
                }
            }
            return 30;
        },
        Category::Choice => _dice.iter().sum(),
        Category::Yacht => {
            if map.keys().count() == 1 {
                return 50;
            }
            return 0;
        },
    }
}
