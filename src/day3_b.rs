use aoc_2022::crawl_input;

pub fn day3_b() {

let input = crawl_input("2022", "3").unwrap();
let mut lines = input.lines();
let mut result: i32 = 0;

loop {

    let first = lines.next();
    let second = lines.next();
    let third = lines.next();
    
    if first == None {
        break;
    }
    
    let first = first.unwrap().as_bytes();
    let second = second.unwrap().as_bytes();
    let third = third.unwrap().as_bytes();
    let mut common_char: u8 = 0;
    
    for i in 0..first.len(){

        for x in 0..second.len(){
            if first[i] == second[x]{
                for y in 0..third.len(){
                    if second[x] == third[y]{
                        common_char = third[y];
                        break;
                    }
                }
            }
        }
    }
     if common_char <= 97 {
        common_char -= 38;
    } else {
        common_char -= 96;
    }
    result += i32::from(common_char);
}

println!("{}", result);

}