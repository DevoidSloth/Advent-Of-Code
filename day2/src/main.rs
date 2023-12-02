use std::fs;
use std::env;

fn main() {

    let file_path: String = String::from("in.txt");

    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    

    for n in 0..lines.len(){
        let game = &lines[n][..8];
        let red: i32 = 0;
        let green: i32 = 0;
        let blue: i32 = 0;
        

    }
}
