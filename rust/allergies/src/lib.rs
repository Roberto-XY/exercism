use std::{collections::HashSet, iter};

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
        let mut curr_score = score;
        let mut curr_exponent = 0u8;

        // Horner schema: 34_(10) == 100010_(2) == 2⁵ + 2¹
        let indices_of_set_bits_in_binary_number = iter::from_fn(|| {
            if curr_score == 0 {
                None
            } else {
                let is_odd = curr_score % 2 == 1;

                let next = if is_odd {
                    Some(Some(curr_exponent))
                } else {
                    Some(None)
                };

                curr_exponent += 1;
                curr_score /= 2;

                next
            }
        })
        .fuse()
        .flatten();

        let allergens = indices_of_set_bits_in_binary_number
            .take(8)
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
