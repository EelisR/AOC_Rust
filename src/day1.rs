pub fn solve_day_one(str: &String) {
    let lines = str.lines();
    let mut max_calories = 0;
    let mut calorie_count = 0;

    for line in lines {
        if line.is_empty() {
            max_calories = if calorie_count > max_calories {
                calorie_count
            } else {
                max_calories
            };
            calorie_count = 0;
        } else {
            let current = line.parse::<i32>().unwrap();
            calorie_count += current;
        }
    }

    print!("{max_calories}")
}

pub fn solve_day_one_second(str: &String) {
    let lines = str.lines();
    let mut calorie_count = 0;
    let mut top_three = [0; 3];

    for line in lines {
        if line.is_empty() {
            top_three.sort_unstable();
            let smallest = if calorie_count > top_three[0] {
                calorie_count
            } else {
                top_three[0]
            };

            top_three[0] = smallest;
            calorie_count = 0;
        } else {
            let current = &line.parse::<i32>().unwrap();
            calorie_count += current;
        }
    }

    let total: i32 = top_three.iter().sum();
    print!("{total}")
}
