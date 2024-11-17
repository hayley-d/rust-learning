mod back_of_the_hosue {
    // Struct must be pub to access
    pub struct Breakfast {
        eggs: u8,
        pub toast: String, // must be a public field to access from outside
        coffee: DrinkType,
    }

    impl Breakfast {
        pub fn scrambled(toast: &str) -> Breakfast {
            return Breakfast {
                eggs: 4,
                toast: String::from(toast),
                coffee: DrinkType::Coffee,
            };
        }

        fn the_same(&self, other: &Breakfast) -> bool {
            return self.eggs == other.eggs;
        }
    }

    pub enum DrinkType {
        Coffee,
        OrageJuice,
        Matcha,
    }
}

fn main() {
    let mut meal: back_of_the_hosue::Breakfast = back_of_the_hosue::Breakfast::scrambled("Brown");
    meal.toast = String::from("Rye");
}
