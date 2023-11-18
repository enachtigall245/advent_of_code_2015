use std::error::Error;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::path::Path;
// This code presents one solution to the 2015 advent of code problem from day 2: Present wrapping
// I am using this one to learn and practice Rust! 


//Logic: We have a text list of input dimensions and a result of a total of wrapping paper. Lets do a running total for simplicity.
//This is NOT optimized and is pretty beginner code as a solution. I might come back and optimize in the future. This uses a ton of variables/memory it doesnt need to.
fn main() -> Result<(), std::io::Error> {

    //Init running total
    let mut total_paper = 0;
    let mut total_ribbon = 0;

    let path = Path::new("input2.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, <dyn Error>::to_string(&why)),
        Ok(file) => file,
    };

     // Lets read through the lines! 
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        
        //strip dims from current line
        let dims: Vec<i32> = line?.split('x').map(|x| x.parse::<i32>().unwrap()).collect();

        //Solve for paper - yes this can be optimized but I wanted it clear and legible at the cost of memory/time...
        let lw = dims[0]*dims[1];
        let lh = dims[0]*dims[2];
        let wh = dims[1]*dims[2];
        let v = vec![lw, lh, wh];
        let min_side = *v.iter().min().unwrap();
        total_paper += 2*(lw + lh + wh) + min_side;

        //Solve for ribbon
        let per1 = 2*dims[0] + 2*dims[1];
        let per2 = 2*dims[0] + 2*dims[2];
        let per3 = 2*dims[1] + 2*dims[2];
        let p = vec![per1, per2, per3];
        let min_per = *p.iter().min().unwrap();

        total_ribbon += min_per + dims[0]*dims[1]*dims[2];
    }

    println!("{}", total_paper);
    println!("{}", total_ribbon);
    Ok(())
}
