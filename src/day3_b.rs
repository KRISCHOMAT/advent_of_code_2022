use aoc_2022::crawl_input;

pub fn day3_b() {

let input = crawl_input("2022", "3").unwrap();
let mut lines = input.lines();
let mut result: i32 = 0;

loop {
    let first = lines.next();
    if first == None {break;}

    let first = first.unwrap().bytes();
    let second = lines.next().unwrap().as_bytes();
    let third = lines.next().unwrap().as_bytes();

    for e in first {
        if second.contains(&e) && third.contains(&e) {
            let mut common_char = e;
            if common_char <= 97 {
                common_char -= 38;
            } else {
                common_char -= 96;
            }
            
            result += i32::from(common_char);
            break;
        }
    }
}

println!("{}", result);

}