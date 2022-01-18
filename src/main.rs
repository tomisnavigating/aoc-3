use std::{io::{BufReader, BufRead}, fs::File};


fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut n_lines = 0;
    let mut totals = vec![0; 12];


    for line in reader.lines() {
        let line = line.unwrap();
        n_lines += 1;

        for (i,c) in line.chars().enumerate(){
            
            if c=='1' {
                totals[i] += 1
            }

        }
    }
    println!("{:?}", totals);
    println!("{:?}", n_lines);
    println!("{:?}", n_lines/2);



    // let mut i = (totals.len() - 1) as i32;
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    for i in 0..(totals.len()) {
        let bit = totals[totals.len() - 1 - i];
        if bit > n_lines/2 {
            println!("{} -> {}", i, true);
            gamma += 2i32.pow(i as u32);
        } else {
            println!("{} -> {}", i, false);
            epsilon += 2i32.pow(i as u32);
        }

    }
    println!("Power consumption of the submarine: {}", gamma * epsilon);

}
