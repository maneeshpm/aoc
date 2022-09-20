use aoc21::read_lines;
use std::error::Error;

fn cnt<T>(dep: &Vec<T>, skip: usize) -> i32
where T: std::cmp::PartialOrd
{
    let mut ans = 0;

    let mut i = 0;
    while i + skip < dep.len() {
        if dep[i] < dep[i + skip] {
            ans += 1;
        }
        i += 1;
    }
    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let depths = read_lines::<i32>("src/bin/aoc1.input")?;
    // println!("{:#?}", depths);
    
    println!("without skip: {}", cnt(&depths, 1)); println!("with skip: {}", cnt(&depths, 3));
    Ok(()) 
}
