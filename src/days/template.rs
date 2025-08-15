//!day_01.rs

use anyhow::Result;

pub fn day_01() -> Result<()> {
    let _input = include_str!("../../assets/day_01.txt");

    /*
    let result_part1 =
    println!("result day_01 part 1: {result_part1}");
    assert_eq!(result_part1, XXX);

    let result_part2 =
    println!("result day_01 part 2: {result_part2}");
    assert_eq!(result_part2, YYY);
    */
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part() -> Result<()> {
        let _input = include_str!("../../assets/day_01_example.txt");

        /*
        let result_part1 =
        println!("result day_01 part 1: {result_part1}");
        assert_eq!(result_part1, XXX);

        let result_part2 =
        println!("result day_01 part 2: {result_part2}");
        assert_eq!(result_part2, YYY);
        */

        Ok(())
    }
}
