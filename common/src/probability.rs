use std::marker::PhantomData;

use rand::Rng;

pub struct Probability<T> {
    percent: u8,
    marker: PhantomData<T>,
}

impl<T> Probability<T> {
    pub fn new(percent: u8) -> Probability<T> {
        Probability {
            percent,
            marker: PhantomData,
        }
    }

    pub fn eval(&self, on_success: T, or_else: T) -> T {
        let mut rnd = rand::thread_rng();
        let res: u8 = rnd.gen_range(1..=100);
        if res <= self.percent {
            on_success
        } else {
            or_else
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50_percent_probability() {
        for _ in 0..1000 {
            let p: Probability<bool> = Probability::new(50);
            run_probability_test(p, 1000, 400, 600);
        }
    }

    #[test]
    fn test_100_percent_probability() {
        for _ in 0..1000 {
            let p: Probability<bool> = Probability::new(100);
            run_probability_test(p, 1000, 1000, 1000);
        }
    }

    #[test]
    fn test_0_percent_probability() {
        for _ in 0..1000 {
            let p: Probability<bool> = Probability::new(0);
            run_probability_test(p, 1000, 0, 0);
        }
    }

    fn run_probability_test(p: Probability<bool>, n: i32, lower: i32, upper: i32) {
        let mut ctr = 0;
        for _ in 0..n {
            let res = p.eval(true, false);
            if res {
                ctr = ctr + 1;
            }
        }
        println!("ctr = {}", ctr);
        assert!(ctr >= lower);
        assert!(ctr <= upper);
    }
}
