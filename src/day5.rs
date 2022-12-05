use aoc_2022::{crawl_input};
use std::collections::BTreeMap;

struct Instructions {
    amount: usize,
    from: usize,
    to: usize
}


pub fn day5(){
    let input = crawl_input("2022", "5").unwrap();
    let lines = input.lines();
    let mut stacks: BTreeMap<usize, Vec<char>> = init_stacks(&input);
   
    // stacks before
    for (key, value) in &stacks {
        println!("{} {:?}", key, value)
    }

    for line in lines {
    if !line.contains("move") {continue;}
    let instructions = get_instructions(line);

    for _i in 0..instructions.amount{
            let element = stacks.get_mut(&instructions.from).unwrap().pop().unwrap();
            stacks.get_mut(&instructions.to).unwrap().push(element);
        }
    }
    
    println!("---");
    // stacks after
    for (key, value) in &stacks {
        println!("{} {:?}", key, value)
    }

    let mut result = String::new();

    for (_key, value) in &stacks {
        result.push(*value.last().unwrap());
    }

    // result
    println!("RESULT: {}", result);
}

fn init_stacks(input: &str) -> BTreeMap<usize, Vec<char>>{
    let mut stacks: BTreeMap<usize, Vec<char>> = BTreeMap::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next(){
        if line.contains("1") {break;}
        let mut prev = 0;
        let mut chars = line.chars();
        chars.next();
        for i in 0..line.chars().count(){
            if i / 4 == prev {
                let char = chars.next().unwrap();
                if char == ' ' {

                } else if !stacks.contains_key(&(i/4 + 1)){
                    stacks.insert(i / 4 + 1,vec![char]);
                } else {
                    stacks.get_mut(&(i/4 + 1)).unwrap().insert(0,char);
                }
                prev += 1;
            } else {
                chars.next();
            }
        }   
    }
    stacks
}

fn get_instructions(line: &str) -> Instructions {
    let mut instructions = Instructions{amount: 0, from:0, to: 0};
     let mut split = line.split_whitespace();
        split.next();
        instructions.amount = split.next().unwrap().parse().unwrap();
        split.next();
        instructions.from = split.next().unwrap().parse().unwrap();
        split.next();
        instructions.to = split.next().unwrap().parse().unwrap();

        instructions
}