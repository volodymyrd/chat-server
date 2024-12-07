use crate::adjectives::ADJECTIVES;
use crate::animals::ANIMALS;

mod adjectives;
mod animals;

pub fn random_name() -> String {
    let adjective = fastrand::choice(ADJECTIVES).unwrap();
    let animal = fastrand::choice(ANIMALS).unwrap();
    format!("{adjective}{animal}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_name() {
        println!("{}", random_name())
    }
}
