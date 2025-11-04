pub fn part1(content: &str) -> i32 {
    let mut result = 0;
    for ch in content.chars() {
        result += get_monster_score(ch);
    }
    result
}

pub fn part2(content: &str) -> i32 {
    let mut result = 0;
    let mut chars = content.chars();
    while let (Some(first), Some(second)) = (chars.next(), chars.next()) {
        if first != 'x' && second != 'x' {
            result += 2;
        }
        result += get_monster_score(first);
        result += get_monster_score(second);
    }
    if chars.next().is_some() {
        unimplemented!("Monster ungrouped")
    }
    result
}

pub fn part3(content: &str) -> i32 {
    let mut result = 0;
    let mut chars = content.chars();
    while let (Some(first), Some(second), Some(third)) = (chars.next(), chars.next(), chars.next())
    {
        let under_test = [first, second, third];
        let number_of_x = under_test.iter().filter(|&&ch| ch == 'x').count();
        result += match number_of_x {
            0 => 6,
            1 => 2,
            _ => 0,
        };
        result += get_monster_score(first);
        result += get_monster_score(second);
        result += get_monster_score(third);
    }
    if chars.next().is_some() {
        unimplemented!("Monster(s) ungrouped")
    }
    result
}

fn get_monster_score(monster: char) -> i32 {
    match monster {
        'A' | 'x' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => unimplemented!("Can't find monster {}", monster),
    }
}
