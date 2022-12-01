use aoc_2022::crawl_input;

pub struct TopThree {
    first: i64,
    second: i64,
    third: i64
}

impl TopThree {
    pub fn calc(&mut self, cal:i64){
         if cal > self.first {
                self.third = self.second;
                self.second = self.first;
                self.first = cal;
            } else if cal > self.second {
                self.third = self.second;
                self.second = cal;
            } else if cal > self.third {
                self.third = cal;
            }
    }
}

pub fn day1() {
    let input = crawl_input("2022", "1");
    let mut top = TopThree{first: 0, second: 0, third: 0};
    let mut cal: i64 = 0;

    for line in input.as_ref().unwrap().lines() {
        if line.is_empty() {
            top.calc(cal);
            cal = 0;
        } else {
            cal += line.parse::<i64>().unwrap();
        } 
    }
    top.calc(cal);

    println!("first:{} second:{} third:{}", top.first, top.second, top.third);
    println!("sum:{}", top.first + top.second + top.third);
}