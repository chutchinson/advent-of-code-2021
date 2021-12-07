use std::io::prelude::*;

fn main() {
    let world: Vec<u32> = std::io::stdin().lock().lines()
        .map(|line| line.expect("failed to read line from input"))
        .flat_map(|line| line.split(",").map(|x| x.parse::<u32>().expect("failed to parse integer")).collect::<Vec<_>>())
        .collect();
    
    println!("{}", simulate(&world, 80));
    println!("{}", simulate(&world, 256));
}

fn simulate(world: &Vec<u32>, days: usize) -> usize {
    let len = 9;
    let mut schedule = world
        .iter()
        .fold(vec![0; len], |mut schedule, &x| {
        schedule[x as usize] += 1;
        schedule
    });

    for _ in 0..days {
        let count = schedule[0];

        schedule[0] = schedule[1];
        schedule[1] = schedule[2];
        schedule[2] = schedule[3];
        schedule[3] = schedule[4];
        schedule[4] = schedule[5];
        schedule[5] = schedule[6];
        schedule[6] = schedule[7];
        schedule[7] = schedule[8];
        
        schedule[8] = count;
        schedule[6] += count;
    }

    return schedule.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let world = vec![3,4,3,1,2];
        assert_eq!(26, simulate(&world, 18));
        assert_eq!(5934, simulate(&world, 80));
    }

    #[test]
    fn example_2() {
        let world = vec![3,4,3,1,2];
        assert_eq!(26984457539, simulate(&world, 256));
    }
}
