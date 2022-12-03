use aoc_2022::crawl_input;

#[derive(Debug)]
#[derive(PartialEq)]
enum Choise {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Other
}

pub fn day2_b() {
let input = crawl_input("2022", "2");
let mut my_score: i64 = 0;

let won = 6;
let draw = 3;

for lines in input.as_ref().unwrap().lines() {
    let mut line = lines.split_whitespace();
    let opponent = line.next();
    let me = line.next().unwrap();

    let opponent =  match opponent {
        Some("A") => Choise::Rock,
        Some("B") => Choise::Paper,
        Some("C") => Choise::Scissors,
        Some(&_) => Choise::Other,
        None => Choise::Other
    }; 
        
    let me = match opponent {
        Choise::Rock => {
            if me == "X" {
                Choise::Scissors
            } else if me == "Y" {
                Choise::Rock
            } else if me == "Z" {
                Choise::Paper
            } else {
                Choise::Other
            }
        },
        Choise::Paper => {
            if me == "X" {
                Choise::Rock
            } else if me == "Y" {
                Choise::Paper
            } else if me == "Z" {
                Choise::Scissors
            } else {
                Choise::Other
            }
        },
        Choise::Scissors => {
            if me == "X" {
                Choise::Paper
            } else if me == "Y" {
                Choise::Scissors
            } else if me == "Z" {
                Choise::Rock
            } else {
                Choise::Other
            }
        },
        Choise::Other => Choise::Other
    };
        
    if me == opponent{
        match me {
            Choise::Rock => my_score += me as i64 + draw,
            Choise::Paper => my_score += me as i64 + draw,
            Choise::Scissors => my_score += me as i64 + draw,
            Choise::Other => my_score += 0
        }    
    } else if 
        me == Choise::Rock && opponent == Choise::Scissors || 
        me == Choise::Paper && opponent == Choise::Rock || 
        me == Choise::Scissors && opponent == Choise::Paper {
        match me {
            Choise::Rock => my_score += me as i64 + won,
            Choise::Paper => my_score += me as i64 + won,
            Choise::Scissors => my_score += me as i64 + won,
            Choise::Other => my_score += 0
        }
    } else {
        match me {
            Choise::Rock => my_score += me as i64,
            Choise::Paper => my_score += me as i64,
            Choise::Scissors => my_score += me as i64,
            Choise::Other => my_score += 0
        }
    }
}
    println!("{}", my_score);
}