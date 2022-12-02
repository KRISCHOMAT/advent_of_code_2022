use aoc_2022::crawl_input;

pub fn day2() {
let input = crawl_input("2022", "2");

// opponent
let a: i64 = 1; //Rock
let b: i64 = 2; //Paper
let c: i64 = 3; //Scissors

// me
let x: i64 = a; //Rock
let y: i64 = b; //Paper
let z: i64 = c; //Scissors

// outcomes
let lost: i64 = 0;
let draw: i64 = 3;
let won: i64 = 6;

let mut my_score: i64 = 0;

for lines in input.as_ref().unwrap().lines() {
    let mut line = lines.split_whitespace();
    let opponent = line.next().unwrap();
    let me = line.next().unwrap();

    let opponent =  match opponent {
    "A" => a,
    "B" => b,
    "C" => c,
    &_ => 0
    }; 

    let me =  match me {
    "X" => x,
    "Y" => y,
    "Z" => z,
    &_ => 0
    };
    
    if opponent == me {
        my_score += me + draw;
    } else if 
        opponent == a && me == y || 
        opponent == b && me == z || 
        opponent == c && me == x {
        my_score += me + won;
    } else {
        my_score += me + lost;
    }
}
    println!("{}", my_score);
}