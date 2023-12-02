use std::fs;
use std::env;

fn main() {

    let file_path: String = String::from("in.txt");

    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    
    let mut total = 0;


    for n in 0..lines.len(){
        total += n+1;

        let mut line = lines[n];
        //ID Number (-_-) Why did I do this? Its literally index+1
        // let start = line.chars().position(|c| c == 'e').unwrap() + 2;
        let stop = line.chars().position(|c| c == ':').unwrap();
        // let id:i32 = line[start..stop].trim().parse().unwrap();

        line = &line[stop+2..line.len()];

        let lists = line.split(";");
        let mut bad = false;

        for list in lists{
            let draws = list.split(",");
            for draw in draws{
                
                if draw.contains("red"){
                    let r = draw.chars().position(|c: char| c == 'r').unwrap();
                    let num: i32 = draw.trim()[0..r-1].trim().parse().unwrap();
                    if num > 12{
                        total -= n+1;
                        // println!("{}" , num);
                        bad = true;
                        println!("game {} is bad", n+1);
                        break;
                    }
                }
                if draw.contains("green"){
                    let g = draw.chars().position(|c: char| c == 'g').unwrap();
                    let num: i32 = draw.trim()[0..g-1].trim().parse().unwrap();
                    if num > 13 {
                        total -= n+1;
                        println!("game {} is bad", n+1);
                        bad = true;
                        break;

                    }
                    // println!("{}", &draw.trim()[0..g-1].trim());
                }
                if draw.contains("blue"){
                    let b = draw.chars().position(|c: char| c == 'b').unwrap();
                    let num: i32 = draw.trim()[0..b-1].trim().parse().unwrap();
                    if num > 14 {
                        total -= n+1;
                        println!("game {} is bad", n+1);
                        bad = true;
                        break;
                    }
                    // println!("{}", &draw.trim()[0..b-1].trim());
                }
            }
            if(bad){
                break;
            }

        }        

    }
    println!("The total sum of the id's is {}",total);

}
