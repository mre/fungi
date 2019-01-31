pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone)]
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
        return Self { score };
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        return 0 != self.score & 2_u32.pow((*allergen).clone() as u32)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .into_iter()
        .filter(|allergen| self.is_allergic_to(allergen))
        .collect()
    }
}
