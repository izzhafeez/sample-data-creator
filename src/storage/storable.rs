use std::error::Error;

pub trait Storable {
    fn get() -> Result<Vec<Self>, Box<dyn Error>>
    where
        Self: Sized;

    fn transform(f: fn(Self) -> Self) -> Self {

    }
}