pub struct Allergies {
    store: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        //unimplemented!("Given the '{score}' score, construct a new Allergies struct.");
        let mut s = score;
        let mut store: Vec<Allergen> = Vec::new();
        for a in vec![Allergen::Eggs, 
                      Allergen::Peanuts,
                      Allergen::Shellfish,
                      Allergen::Strawberries,
                      Allergen::Tomatoes,
                      Allergen::Chocolate,
                      Allergen::Pollen,
                      Allergen::Cats].into_iter() {
            
            if s % 2 != 0 {
                store.push(a);
            }
            s /= 2;
        }
        Self { store : store }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        //unimplemented!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
        self.store.iter().any(|x| x == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        //unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
        self.store.clone()
    }
}