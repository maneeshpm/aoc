use std::{fs::read_to_string, io::Error};
use std::str::FromStr;

pub fn read_lines<T>(filename: &str) -> Result<Vec<T>, Error> 
where T: FromStr
{
    Ok(
        read_to_string(filename)?
        .lines()
            .map(|iter| iter.parse::<T>())
            .filter_map(|iter| iter.ok())
            .collect()
    )
}
