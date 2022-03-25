use std::collections::HashSet;

pub struct Allergies {
    allergens: HashSet<Allergen>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        // Horner schema
        let mut power_of_2s = vec![];
        let mut curr_score = score;
        let mut curr_exponent = 0;
        while curr_score != 0 {
            let is_odd = curr_score % 2 == 1;

            if is_odd {
                power_of_2s.push(curr_exponent);
            }

            curr_exponent += 1;
            curr_score /= 2;
        }

        let allergens = power_of_2s
            .iter()
            .flat_map(|x| match x {
                0 => Some(Allergen::Eggs),
                1 => Some(Allergen::Peanuts),
                2 => Some(Allergen::Shellfish),
                3 => Some(Allergen::Strawberries),
                4 => Some(Allergen::Tomatoes),
                5 => Some(Allergen::Chocolate),
                6 => Some(Allergen::Pollen),
                7 => Some(Allergen::Cats),
                _ => None,
            })
            .collect();

        Allergies { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.iter().map(|x| *x).collect()
    }
}
