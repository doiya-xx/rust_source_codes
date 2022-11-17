pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[cfg(test)]
mod tests {
    use crate::{Allergen, Allergies};
    
    #[test]
    fn test_1() {
        let score: u32 = 255;
        let score = score.to_be_bytes()[3];
        let score = (score & 0b00000010) == 0b00000010;
        println!("{:?}", score);
        
        let vec = Vec::<Allergen>::new();
    }
    
    #[test]
    fn test_2() {
        let mut allergies = Allergies {
            allergens: Vec::<Allergen>::new(),
        };
        let score: u32 = 255;
        let score = score.to_be_bytes()[3]; // 0b1111_1111
        let allergens = vec![
            Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish, Allergen::Strawberries,
            Allergen::Tomatoes, Allergen::Chocolate, Allergen::Pollen, Allergen::Cats,
        ];
        let mut iter = allergens.into_iter();
        let x = iter.next();
        for i in 0..=7 {
            let c = 0b00000001 << i;
            if (score & c) == c {
                allergies.allergens.push(iter.next().unwrap());
            }
        }
        // allergies
    }
    
    #[test]
    fn test_3(){

    }
}