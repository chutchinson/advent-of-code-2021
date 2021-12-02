mod prelude;
use prelude::*;

fn main() {
    let course = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("invalid line"))
        .collect();

    println!("{}", simulate_course(&course, basic));
    println!("{}", simulate_course(&course, aim));
}

struct Simulation {
    x: i32,
    depth: i32,
    aim: i32
}

type Simulator = fn(&mut Simulation, direction: &str, value: i32);

fn basic(sim: &mut Simulation, direction: &str, value: i32) {
    match direction {
        "forward" => { sim.x += value; },
        "up" => { sim.depth -= value; },
        "down" => { sim.depth += value; },
        _ => {}
    }
}

fn aim(sim: &mut Simulation, direction: &str, value: i32) {
    match direction {
        "forward" => { sim.x += value;  sim.depth += sim.aim * value; },
        "up" => { sim.aim -= value; },
        "down" => { sim.aim += value; },
        _ => {}
    }
}

fn simulate_course<T: AsRef<str>>(v: &Vec<T>, simulate: Simulator) -> i32 {
    let mut state = Simulation {
        x: 0,
        depth: 0,
        aim: 0
    };
    for v in v.iter() {
        let mut parts = v.as_ref().split(" ");
        let direction = parts.next().expect("missing direction");
        let value = parts.next().map(|p| p.parse::<i32>().expect("invalid value")).expect("missing value");
        simulate(&mut state, direction, value);
    }
    state.x * state.depth
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let v = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ];
        assert_eq!(150, simulate_course(&v, basic));
    }

    #[test]
    fn example_2() {
        let v = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ];
        assert_eq!(900, simulate_course(&v, aim));
    }
}
