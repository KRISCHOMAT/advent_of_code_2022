use aoc_2022::crawl_input;

pub fn day3() {
let input = crawl_input("2022", "3").unwrap();
let lines = input.lines();
let mut result: i32 = 0;
let mut common_char: u8 = 0;


for line in lines {
    let count = &line.chars().count();
    let halfs = (&line[..count/2].as_bytes(), &line[count/2..].as_bytes());
    for i in 0..halfs.0.len(){
        for x in 0..halfs.1.len(){
            if halfs.0[i] == halfs.1[x]{
                common_char = halfs.0[i];
                break;
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