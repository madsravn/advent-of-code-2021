use std::fs;

fn main() {
    // TODO: Can be solved more idiomatic with a window
    solve_1_1("input/input-1-1.txt");
    solve_1_2("input/input-1-1.txt");
    // Seems OK
    solve_2_1("input/input-2-1.txt");
    // Maybe with a zip or something? 
    solve_2_2("input/input-2-1.txt");
}

fn solve_2_1(filename: &str) {
    // Forward, down, up
    let vec: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let down: &Vec<String> = &vec.iter().filter(|x| x.contains("down")).map(|x| x.chars().skip(5).collect::<String>()).collect();
    let up: &Vec<String> = &vec.iter().filter(|x| x.contains("up")).map(|x| x.chars().skip(3).collect::<String>()).collect();
    let forward: &Vec<String> = &vec.iter().filter(|x| x.contains("forward")).map(|x| x.chars().skip(8).collect::<String>()).collect();
    let down_sum: i32 = down.iter().map(|x| x.parse::<i32>().expect("Should be able to parse number")).sum();
    let up_sum: i32 = up.iter().map(|x| x.parse::<i32>().expect("Should be able to parse number")).sum();
    let forward_sum: i32 = forward.iter().map(|x| x.parse::<i32>().expect("Should be able to parse number")).sum();
    let result = (down_sum - up_sum) * forward_sum;
    println!("2-1: Found {} as result", result);
}

fn solve_2_2(filename: &str) {
    let vec: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let mut aim = 0;
    let mut hor = 0;
    let mut depth = 0;
    for s in vec {
        if s.contains("down") {
            let value = s.chars().skip(5).collect::<String>().parse::<i32>().expect("Should be able to parse to number");
            aim += value;
        }
        if s.contains("up") {
            let value = s.chars().skip(3).collect::<String>().parse::<i32>().expect("Should be able to parse to number");
            aim -= value;
        }
        if s.contains("forward") {
            let value = s.chars().skip(8).collect::<String>().parse::<i32>().expect("Should be able to parse to number");
            hor += value;
            depth += aim * value;
        }
    }
    let result = hor * depth;
    println!("2-2: Found {} as result", result);

}

fn solve_1_1(filename: &str) {
    let vec: Vec<i32> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|x|  x.parse::<i32>().expect("Should be able to convert to integer")).collect();
    let mut last = 0;
    let mut count = -1;
    for element in vec {
        if element > last {
            count += 1;
        }
        last = element;
    }
    println!("1-1: Found {} increases", count);
}

fn solve_1_2(filename: &str) {
    let vec: Vec<i32> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|x|  x.parse::<i32>().expect("Should be able to convert to integer")).collect();
    let mut vec_2 = Vec::new();
    for e in 0..vec.len()-2 {
        vec_2.push(vec[e] + vec[e+1] + vec[e+2]);
    }
    let mut last = 0;
    let mut count = -1;
    for element in vec_2 {
        if element > last {
            count += 1;
        }
        last = element;
    }
    println!("1-2: Found {} increases", count);
}
