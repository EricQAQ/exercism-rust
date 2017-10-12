#[derive(PartialEq, Debug)]
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
    score: usize,
}

impl Allergies {
    pub fn new(score: usize) -> Self {
        Allergies { score: score }
    }

    fn get_allergen(&self) -> Vec<Allergen> {
        vec![
            Allergen::Eggs, Allergen::Peanuts, Allergen::Shellfish,
            Allergen::Strawberries, Allergen::Tomatoes, Allergen::Chocolate,
            Allergen::Pollen, Allergen::Cats,
        ]
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        if let Some(pos) = self.get_allergen().iter().position(|x: &Allergen| x == allergen) {
            return self.score & (1 << pos) != 0
        }
        return false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.get_allergen()
            .into_iter()
            .filter(|x| self.is_allergic_to(&x))
            .collect::<Vec<Allergen>>()
    }
}
