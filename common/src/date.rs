use chrono::{Datelike, Local, NaiveDate};
use rand::Rng;

use crate::Random;

#[derive(Debug)]
pub struct DateOfBirth {
    year: i32,
    month: u32,
    day: u32,
}

impl From<NaiveDate> for DateOfBirth {
    fn from(naive_date: NaiveDate) -> Self {
        DateOfBirth {
            year: naive_date.year(),
            month: naive_date.month(),
            day: naive_date.day(),
        }
    }
}

impl Random for DateOfBirth {
    fn random() -> Self {
        let mut rnd = rand::thread_rng();
        let start_year = Local::now().date_naive().year() - 30;
        let end_year = Local::now().date_naive().year() - 15;
        let year = rnd.gen_range(start_year..=end_year);
        let month = rnd.gen_range(1..=12);
        let day = rnd.gen_range(1..=28);
        DateOfBirth { year, month, day }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_random_date() {
        let date_of_birth = DateOfBirth::random();
        let min_year = Local::now().naive_local().year() - 30;
        let max_year = Local::now().naive_local().year() - 15;
        assert!(date_of_birth.year >= min_year);
        assert!(date_of_birth.year <= max_year);
        assert!(date_of_birth.month >= 1 && date_of_birth.month <= 12);
        assert!(date_of_birth.day >= 1 && date_of_birth.day <= 28);
    }

    #[test]
    fn test_from_naive_date() {
        let naive_date = Local::now().date_naive();
        let date_of_birth: DateOfBirth = naive_date.into();
        assert_eq!(date_of_birth.day, naive_date.day());
        assert_eq!(date_of_birth.month, naive_date.month());
        assert_eq!(date_of_birth.year, naive_date.year());
    }
}
