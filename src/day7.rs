mod prelude;
use prelude::*;

fn main() {
    let measurements: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("failed to read line"))
        .flat_map(|line| line.split(",").map(|x| x.parse::<i32>().expect("failed to parse integer")).collect::<Vec<_>>())
        .collect();

    println!("{}", find_minimum_fuel(&measurements, constant_fuel_cost));
    println!("{}", find_minimum_fuel(&measurements, variable_fuel_cost));
}

fn diff(positions: &Vec<i32>, pos: i32, cost: fn(i32) -> i32) -> i32 {
    positions.iter()
        .map(|p| {
            let delta = (p - pos).abs();
            cost(delta)
        })
        .sum()
}

fn constant_fuel_cost (distance: i32) -> i32 {
    distance
}

fn variable_fuel_cost (distance: i32) -> i32 {
    distance
}

fn find_minimum_fuel(positions: &Vec<i32>, cost: fn(i32) -> i32) -> i32 {
    let pos = positions.clone();
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    let range = 0..(max-min);
    let min_pos = range.map(|x| diff(&pos, x, cost)).min().unwrap();
    min_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let v = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, find_minimum_fuel(&v, constant_fuel_cost));
    }

    #[test]
    fn example_2() {
        let v = vec![16,1,2,0,4,2,7,1,2,14];
        // 16 - 5 = 11
        assert_eq!(206, find_minimum_fuel(&v, variable_fuel_cost));
    }
}