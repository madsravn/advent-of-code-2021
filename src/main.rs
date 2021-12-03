use std::fs;

fn main() {
    // TODO: Can be solved more idiomatic with a window
    solve_1_1("input/input-1-1.txt");
    solve_1_2("input/input-1-1.txt");
    // Seems OK
    solve_2_1("input/input-2-1.txt");
    // Maybe with a zip or something? 
    solve_2_2("input/input-2-1.txt");
    solve_3_1("input/input-3-1.txt");
    solve_3_2("input/input-3-1.txt");
}

const BINARIES: [i32; 12] = [0b100000000000,
                             0b010000000000,
                             0b001000000000,
                             0b000100000000,
                             0b000010000000,
                             0b000001000000,
                             0b000000100000,
                             0b000000010000,
                             0b000000001000,
                             0b000000000100,
                             0b000000000010,
                             0b000000000001];

fn solve_3_2(filename: &str) {
    let vec: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let counts = count_ones(&vec, 12);
    let reading_count = vec.len();
    let ones: Vec<bool> = counts.iter().map(|x| x*2 >= reading_count).collect();
    let mut vec_2: Vec<String> = vec.clone();
    for i in 0..12 {
        vec_2 = vec_2.iter()
            .filter(|x| x.chars().skip(i).take(1).collect::<String>().contains("1") == ones[i])
            .map(|s| s.to_string()).collect::<Vec<String>>();
        if vec_2.len() == 1 {
            break;
        }
    }
    let oxygen_generator_rating = &vec_2[0];
    //let oxygen_generator_rating = vec_2[0].chars().zip(BINARIES.iter()).fold(0, |mut sum, (value, binary)| if value == '1' { sum = sum | binary; sum} else { sum});

    let mut vec_2 = vec.clone();
    for i in 0..12 {
        vec_2 = vec_2.iter()
            .filter(|x| x.chars().skip(i).take(1).collect::<String>().contains("1") != ones[i])
            .map(|s| s.to_string()).collect::<Vec<String>>();
        if vec_2.len() == 1 {
            break;
        }
    }
    //let co2_scrubber_rating  = &vec_2[0].chars().zip(BINARIES.iter()).fold(0, |mut sum, (value, binary)| if value == '1' { sum = sum | binary; sum} else { sum});
    let co2_scrubber_rating  = &vec_2[0];
    //let result = oxygen_generator_rating * co2_scrubber_rating;
    println!("Middle results are {} and {}", oxygen_generator_rating, co2_scrubber_rating);
    //println!("3-2: Found {} as result", result);


}

fn solve_3_1(filename: &str) {
    let vec: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let reading_count = vec.len();
    let counts = count_ones(&vec, 12);
    let numbers = counts.iter().zip(BINARIES.iter()).fold((0, 0), |(mut gamma, mut epsilon), (c, b)| if *c*2 > reading_count { gamma = gamma | b; (gamma, epsilon) } else { epsilon = epsilon | b; (gamma, epsilon) } );
    let gamma = numbers.0;
    let epsilon = numbers.1;
    let result = gamma*epsilon;
    println!("3-1: Found {} as result", result);
}

fn count_ones(vec: &Vec<String>, count: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for place in 0..count {
        let count = vec.iter().map(|x| x.chars().skip(place).take(1).collect::<String>()).filter(|s: &String| s.contains("1") == true).collect::<String>().len();
        res.push(count);
    }
    res
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
