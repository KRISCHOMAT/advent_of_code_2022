use aoc_2022::crawl_input;

pub fn day6() {
    let input = crawl_input("2022", "6").unwrap();
    let mut vec: Vec<u8> = Vec::new();
    let mut count = 0;

    // Part 1
    // let num_chars = 4;
    // Part 2
    let num_chars = 14;

    for _byte in input.as_bytes(){
        vec = vec![input.as_bytes()[count]];

        for i in 1..num_chars {
            let element = input.as_bytes()[count+i];
            if !vec.contains(&element){
                vec.push(element);
            }
        }

        if vec.len() == num_chars {
            break;
        }

        count += 1;

}
    println!("{:?} {}",vec, count + num_chars);
}