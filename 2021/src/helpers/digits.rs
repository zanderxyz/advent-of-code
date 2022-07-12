pub fn from_digits(digits: impl Iterator<Item = isize>) -> i64 {
    digits.into_iter().fold(0_i64, |mut sum: i64, number| {
        sum *= 10;
        sum += number as i64;
        sum
    })
}
