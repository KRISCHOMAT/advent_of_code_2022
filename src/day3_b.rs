use aoc_2022::crawl_input;

pub fn day3_b() {

let input = crawl_input("2022", "3").unwrap();
let mut lines = input.lines();
let mut result: i32 = 0;

loop {
    let first = lines.next();
    if first == None {break;}

    let first = first.unwrap().as_bytes();
    let second = lines.next().unwrap().as_bytes();
    let third = lines.next().unwrap().as_bytes();

    for e in first {
        if second.contains(e) && third.contains(e) {
            let common_char = Some(e);
            let common_char = match common_char {
                Some(v) => {
                    if v <= &97 {
                        v - &38
                    } else {
                        v - &96
                    }
                },
                None => panic!("something went wrong"),
            };

            result += i32::from(common_char);
            break;
        }
    }
}

println!("{}", result);

}