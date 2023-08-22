use rand::Rng;
use crate::person::information::randomised_information::RandomisedInformation;

pub struct PhoneNumber {
    number: i32
}

impl RandomisedInformation for PhoneNumber {
    fn get_random() -> Self {
        let mut rng = rand::thread_rng();

        let lower_bound = 80000000;
        let upper_bound = 99999999;

        let number: i32 = rng.gen_range(lower_bound..=upper_bound);
        Self { number }
    }
}