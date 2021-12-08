mod prelude;
use prelude::*;

fn main() {
    let measurements: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("failed to read line"))
        .flat_map(|line| line.split(",").map(|x| x.parse::<i32>().expect("failed to parse integer")).collect::<Vec<_>>())
        .collect();

    println!("{}", find_minimum_position(&measurements, constant_fuel_cost));
    println!("{}", find_minimum_position(&measurements, variable_fuel_cost));
}

fn calculate_fuel_cost(positions: &Vec<i32>, pos: i32, cost: fn(i32) -> i32) -> i32 {
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
    (distance * (distance + 1)) / 2
}

fn find_minimum_position(positions: &Vec<i32>, cost: fn(i32) -> i32) -> i32 {
    let pos = positions.clone();
    let max = positions.iter().max().unwrap();
    let range = 0..*max+1;
    range.map(|x| calculate_fuel_cost(&pos, x, cost)).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let v = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, find_minimum_position(&v, constant_fuel_cost));
    }

    #[test]
    fn example_2() {
        let v = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(206, calculate_fuel_cost(&v, 2, variable_fuel_cost));
        assert_eq!(168, find_minimum_position(&v, variable_fuel_cost));
    }
}