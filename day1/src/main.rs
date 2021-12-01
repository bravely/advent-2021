use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_buffered_input() -> BufReader<File> {
    let file = File::open("input.txt").unwrap();
    BufReader::new(file)
}

fn main() {
    let mut num_of_increases = 0;
    let mut buffered = read_buffered_input();

    // Part 1
    // let mut previous_num: Option<i32> = Option::None;
    // for line in buffered.lines() {
    //     let line = line.expect("Could not read line");
    //     let num = line.parse::<i32>().expect("Could not parse line");
    //     if previous_num.is_none() {
    //         previous_num = Option::Some(num);
    //         continue;
    //     }
    //     let greater = num > previous_num.unwrap();
    //     println!("{} > {} : {}", num, previous_num.unwrap(), greater);
    //     if greater {
    //         num_of_increases += 1;
    //     }
    //     previous_num = Option::Some(num);
    // }
    // println!("Depth increases {} times", num_of_increases);
    // -------------------------------------------------------------------------

    // Part 2
    // Same as above, but now with the sum of a 3-part sliding window
    let mut window = DepthWindow::new();
    let mut buf = String::new();
    while let Ok(n) = buffered.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        trim_newline(&mut buf);
        let num = buf.trim().parse::<i32>().expect("Could not parse line");
        if window.sum.is_none() {
            window.push(num);
            buf.clear();
            continue;
        }
        let previous_sum = window.sum.unwrap();
        window.push(num);
        if window.sum.unwrap() > previous_sum {
            num_of_increases += 1;
        }
        buf.clear();
    }
    println!("Depth increases {} times", num_of_increases);
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

struct DepthWindow {
    first: Option<i32>,
    second: Option<i32>,
    third: Option<i32>,
    sum: Option<i32>,
}

impl DepthWindow {
    fn new() -> DepthWindow {
        DepthWindow {
            first: Option::None,
            second: Option::None,
            third: Option::None,
            sum: Option::None,
        }
    }

    pub fn push(&mut self, num: i32) {
        if self.first.is_none() {
            self.first = Option::Some(num);
            return;
        }
        if self.second.is_none() {
            self.second = Option::Some(num);
            return;
        }
        if self.third.is_none() {
            self.third = Option::Some(num);
            self.sum();
            return;
        }
        self.first = self.second;
        self.second = self.third;
        self.third = Option::Some(num);
        self.sum()
    }

    fn sum(&mut self) {
        if self.first.is_none() || self.second.is_none() || self.third.is_none() {
            return;
        }
        self.sum = Option::Some(self.first.unwrap() + self.second.unwrap() + self.third.unwrap())
    }
}
