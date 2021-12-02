use std::fs;

fn main() {
    solve_1_1("input/input-1-1.txt");
    solve_1_2("input/input-1-1.txt");
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
