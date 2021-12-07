use std::fs;
use std::io::Error;
use std::str::FromStr;


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
    solve_4_1("input/input-4-1.txt");
    solve_4_2("input/input-4-1.txt");
    solve_5_1("input/input-5-1.txt");
    solve_5_2("input/input-5-1.txt");
    solve_6_1("input/input-6-1.txt");
    solve_6_2("input/test-6-1.txt");
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
struct Day5 {
    x_1: usize,
    y_1: usize,
    x_2: usize,
    y_2: usize,
}

impl FromStr for Day5 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = s.split("->").map(|s| s.trim().to_string()).collect::<Vec<String>>().join(",").split(',').map(|s| s.parse::<usize>().expect("Should be able to parse to usize")).collect();

        Ok(Day5 {
            x_1: numbers[0],
            y_1: numbers[1],
            x_2: numbers[2],
            y_2: numbers[3],
        })
    }
}

impl Day5 {
    fn find_points_on_line(&self) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();
        if self.x_1 == self.x_2 {
            if self.y_1 < self.y_2 {
               for e in self.y_1..self.y_2+1 {
                    vec.push((self.x_1, e));
               }
            } else {
                for e in self.y_2..self.y_1+1 {
                    vec.push((self.x_1, e));
                }
            }
        } 
        if self.y_1 == self.y_2 {
            if self.x_1 < self.x_2 {
                for e in self.x_1..self.x_2+1 {
                    vec.push((e, self.y_1));
                }
            } else {
                for e in self.x_2..self.x_1+1 {
                    vec.push((e, self.y_1));
                }
            }
        }
    vec
    }


    fn find_points_crossed(&self) -> Vec<(usize, usize)> {
        if self.x_1 == self.x_2 || self.y_1 == self.y_2 {
            self.find_points_on_line()
        } else {
            let mut size = 0;
            let dir_x: i32 = if self.x_1 < self.x_2 {
                size = self.x_2 - self.x_1;
                1
            } else {
                size = self.x_1 - self.x_2;
                -1
            };

            let dir_y: i32 = if self.y_1 < self.y_2 {
                1
            } else {
                -1
            };


            let mut result = Vec::new();
            for x in 0..size+1 {
                result.push(((self.x_1 as i32 + x as i32 * dir_x) as usize, (self.y_1 as i32 + x as i32 * dir_y) as usize));
            }

            result
        }
    }
}

// Maybe use a matrix
#[derive(Debug, Clone)]
struct Day4 {
    checked: Vec<bool>,
    numbers: Vec<String>
}

impl Day4 {
    fn apply_number(&self, i: &str) -> Day4 {
        let position = self.numbers.iter().position(|s| s == i);
        let mut new_checked = self.checked.clone();
        match position {
            Some(i) => {
                new_checked[i] = true;
            },
            _ => {}
        }
        Day4 {
            checked: new_checked,
            numbers: self.numbers.clone(),
        }
    }

    fn check_win_condition(&self) -> bool {
        let win_conditions = vec!(vec!(0, 1, 2, 3, 4), vec!(5, 6, 7, 8, 9), vec!(10, 11, 12, 13, 14), vec!(15, 16, 17, 18, 19), vec!(20, 21, 22, 23, 24),
                                    vec!(0, 5, 10, 15, 20), vec!(1, 6, 11, 16, 21), vec!(2, 7, 12, 17, 22), vec!(3, 8, 13, 18, 23), vec!(4, 9, 14, 19, 24));
        let wins = win_conditions.iter().filter(|x| x.iter().all(|i| self.checked[*i]) == true).collect::<Vec<&Vec<usize>>>().len();
        wins > 0
    }

    fn get_sum(&self) -> usize {
        self.numbers.iter().zip(self.checked.iter()).fold(0, |mut sum, (value, checked)| if *checked == false { sum += value.parse::<usize>().expect("Should  be able to parse number"); sum } else { sum })
    }
}

impl FromStr for Day4 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strings: Vec<String> = s.split('\n').map(|s| s.to_string()).collect();
        let numbers = strings.join(" ").split(' ').filter(|s| s.len() > 0).map(|s| s.to_string()).collect::<Vec<String>>();


        Ok(Day4 {
            checked: vec![false; 25],
            numbers
        })
    }
}

fn solve_6_1(filename: &str) {
    let mut input: Vec<usize> = fs::read_to_string(filename).expect("Should be able to read input file").split(',').filter(|s| s.len() > 0).map(|x| x.trim().parse::<usize>().expect("Should be able to parse number")).collect();
    for _ in 0..80 {
        let new_fish = input.iter().filter(|&e| *e == 0).count();
        input = input.iter().map(|e| if *e == 0 { 6 } else {e - 1}).collect::<Vec<usize>>();
        for _ in 0..new_fish {
            input.push(8);
        }
    }
    println!("6-1: Found {} as resulrt", input.len());
}

fn solve_6_2(filename: &str) {
    let input: Vec<usize> = fs::read_to_string(filename).expect("Should be able to read input file").split(',').filter(|s| s.len() > 0).map(|x| x.trim().parse::<usize>().expect("Should be able to parse number")).collect();
    let window = 256;
    let sums = vec!(calculate_specific_fish(0, window),
                    calculate_specific_fish(1, window),
                    calculate_specific_fish(2, window),
                    calculate_specific_fish(3, window),
                    calculate_specific_fish(4, window),
                    calculate_specific_fish(5, window));
    let counts = vec!(input.iter().filter(|&e| *e == 0).count(),
                      input.iter().filter(|&e| *e == 1).count(),
                      input.iter().filter(|&e| *e == 2).count(),
                      input.iter().filter(|&e| *e == 3).count(),
                      input.iter().filter(|&e| *e == 4).count(),
                      input.iter().filter(|&e| *e == 5).count());
    let result = sums.iter().zip(counts.iter()).fold(0, |mut res, (sum, count)| {res += *sum * *count; res} );

    println!("6-2: Found {} as result", result);

}

fn calculate_specific_fish(i: usize, window: usize) -> usize {
    println!("Calculating for {}", i);
    let mut input = vec!(i);
    for i in 0..window {
    let new_fish = input.iter().filter(|&e| *e == 0).count();
        input = input.iter().map(|e| if *e == 0 { 6 } else {e - 1}).collect::<Vec<usize>>();
        for _ in 0..new_fish {
            input.push(8);
        }

    }
    println!("{} is returning {}", i, input.len());
    input.len()

}


fn solve_5_2(filename: &str) {
    let input: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let lines: Vec<Day5> = input.iter().map(|x| Day5::from_str(x).expect("Should be able to parse input data")).collect();
    let mut board = vec![vec![0; 1000]; 1000];
    for e in lines.iter() {
        for p in e.find_points_crossed().iter() {
            board[p.0][p.1] += 1;
        }
    }
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if board[x][y] > 1 {
                count += 1;
            }

        }
    }
    for x in 0..10 {
        for y in 0..10 {
            print!("{} ", board[x][y]);
        }
        println!("");
    }

    println!("5-2: Found {} as result", count);
}


fn solve_5_1(filename: &str) {
    let input: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let lines: Vec<Day5> = input.iter().map(|x| Day5::from_str(x).expect("Should be able to parse input data")).collect();
    let mut board = vec![vec![0; 1000]; 1000];
    for e in lines.iter() {
        for p in e.find_points_on_line().iter() {
            board[p.0][p.1] += 1;
        }
    }
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if board[x][y] > 1 {
                count += 1;
            }

        }
    }
    println!("5-1: Found {} as result", count);
}

fn solve_4_2(filename: &str) {
    let input: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split("\n\n").filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let numbers = &input[0];
    let mut boards: Vec<Day4> = input.iter().skip(1).map(|s| Day4::from_str(s).expect("Should be able to parse input")).collect();
    for e in numbers.split(',').map(|s| s.to_string()).collect::<Vec<String>>() {
        boards = boards.iter().map(|b| b.apply_number(&e)).collect();
        let board = &boards[0].clone();
        boards.retain(|b| b.check_win_condition() == false);
        if boards.len() == 0 {
            println!("4-2: Found {} as result", board.get_sum() * e.parse::<usize>().expect("Shoul be able to parse number"));
            break;
        }
    }

}




fn solve_4_1(filename: &str) {
    let input: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split("\n\n").filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
    let numbers = &input[0];
    let mut boards: Vec<Day4> = input.iter().skip(1).map(|s| Day4::from_str(s).expect("Should be able to parse input")).collect();
    for e in numbers.split(',').map(|s| s.to_string()).collect::<Vec<String>>() {
        boards = boards.iter().map(|b| b.apply_number(&e)).collect();
        let won: Vec<&Day4> = boards.iter().filter(|b| b.check_win_condition()).collect();
        if won.len() > 0 {
            println!("4-1: Found {} as result", won[0].get_sum() * e.parse::<usize>().expect("Shoul be able to parse number"));
            break;
        }
    }

}

fn solve_3_2(filename: &str) {
    let vec: Vec<String> = fs::read_to_string(filename).expect("Should be able to read input file").split('\n').filter(|x| x.len() > 0).map(|s| s.to_string()).collect();
      let mut vec_2: Vec<String> = vec.clone();
    for i in 0..12 {
        let counts = count_ones(&vec_2, 12);
        let reading_count = vec_2.len();
        let ones: Vec<bool> = counts.iter().map(|x| x*2 >= reading_count).collect();
        vec_2 = vec_2.iter()
            .filter(|x| x.chars().skip(i).take(1).collect::<String>().contains("1") == ones[i])
            .map(|s| s.to_string()).collect::<Vec<String>>();
        if vec_2.len() == 1 {
            break;
        }
    }
    //let oxygen_generator_rating = &vec_2[0];
    let oxygen_generator_rating = vec_2[0].chars().zip(BINARIES.iter()).fold(0, |mut sum, (value, binary)| if value == '1' { sum = sum | binary; sum} else { sum});

    let mut vec_2 = vec.clone();
    for i in 0..12 {
        let counts = count_ones(&vec_2, 12);
        let reading_count = vec_2.len();
        let ones: Vec<bool> = counts.iter().map(|x| x*2 >= reading_count).collect();

        vec_2 = vec_2.iter()
            .filter(|x| x.chars().skip(i).take(1).collect::<String>().contains("1") != ones[i])
            .map(|s| s.to_string()).collect::<Vec<String>>();
        if vec_2.len() == 1 {
            break;
        }
    }
    let co2_scrubber_rating  = &vec_2[0].chars().zip(BINARIES.iter()).fold(0, |mut sum, (value, binary)| if value == '1' { sum = sum | binary; sum} else { sum});
    //let co2_scrubber_rating  = &vec_2[0];
    let result = oxygen_generator_rating * co2_scrubber_rating;
    println!("3-2: Found {} as result", result);


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
