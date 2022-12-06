use aoc_2022::crawl_input;

pub fn day6() {
    let input = crawl_input("2022", "6").unwrap();
    println!("Part 1: {}", calc_result(4, &input) );
    println!("Part 2: {}", calc_result(14, &input) );
}

fn calc_result(num_chars: usize, input: &str) -> usize {
    let mut count = 0;

    for _byte in input.as_bytes(){
        let mut vec = vec![input.as_bytes()[count]];

        for i in 1..num_chars {
            let element = input.as_bytes()[count+i];
            if vec.contains(&element){
                break;
            } else {
                vec.push(element);
            } 
        }

        if vec.len() == num_chars {
            break;
        }

        count += 1;
    }
    count + num_chars
}