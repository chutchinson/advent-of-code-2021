mod prelude;
use prelude::*;

fn main() {
    let measurements = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.parse::<u32>().expect("invalid input"))
        .collect();

    println!("{}", count_increases(&measurements));
    println!("{}", count_increases_sliding(&measurements));
}

fn count_increases(v: &Vec<u32>) -> u32 {
    v.iter()
        .zip(v.iter().skip(1))
        .fold(0, |increased, x| {
            if *x.1 > *x.0 { increased + 1 } else { increased }
        })
}

fn count_increases_sliding(v: &Vec<u32>) -> u32 {
    let windows = v.windows(3)
        .map(|x| x.iter().sum())
        .collect();
    return count_increases(&windows);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_increases(&v));
    }

    #[test]
    fn example_2() {
        let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_increases_sliding(&v));
    }
}
