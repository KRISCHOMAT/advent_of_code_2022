use aoc_2022::crawl_input;

pub fn day4_ab(){
    let input = crawl_input("2022","4").unwrap();
    let lines = input.lines();
    let mut count_a: i32 = 0;
    let mut count_b: i32 = 0; 
 
    for line in lines {
        let vec = line.split(",").collect::<Vec<&str>>();
        let range_a = vec[0].split("-").collect::<Vec<&str>>();
        let range_b = vec[1].split("-").collect::<Vec<&str>>();
        // Part 1
        if range_a[0].parse::<i32>().unwrap() <= range_b[0].parse::<i32>().unwrap() && 
            range_a[1].parse::<i32>().unwrap() >= range_b[1].parse::<i32>().unwrap() ||
            range_b[0].parse::<i32>().unwrap() <= range_a[0].parse::<i32>().unwrap() && 
            range_b[1].parse::<i32>().unwrap() >= range_a[1].parse::<i32>().unwrap() 
            {
                count_a += 1;
        }
        // Part 2
        if range_a[0].parse::<i32>().unwrap() <= range_b[0].parse::<i32>().unwrap() &&
            range_a[1].parse::<i32>().unwrap() >= range_b[0].parse::<i32>().unwrap() ||
            range_a[0].parse::<i32>().unwrap() >= range_b[0].parse::<i32>().unwrap() &&
            range_a[0].parse::<i32>().unwrap() <= range_b[1].parse::<i32>().unwrap() 
            {
                count_b += 1;
        }
    }
    println!("Part 1: {}, Part 2: {}", count_a, count_b);
}