#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let sum: u64 = sum_of_factor(num);
    
    if num == 0 {
        None
    }
    else if num == sum {
        Some(Classification::Perfect)
    }
    else if num < sum {
        Some(Classification::Abundant)
    }
    else {
        Some(Classification::Deficient)
    }
}

fn sum_of_factor(num: u64) -> u64{
    let mut sum: u64 = 0;
    for i in 1..num{
        if num % i == 0{
            sum += i;
        }
    }
    return sum;
}