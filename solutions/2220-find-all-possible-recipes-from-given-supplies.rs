use std::collections::HashSet;

impl Solution {
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        let n = recipes.len();
        let mut supplies = HashSet::<String>::from_iter(supplies.into_iter());
        let mut creatable = vec![false; n];

        let mut new_supplies = true;
        while new_supplies {
            new_supplies = false;
            
            for i in 0..n {
                if creatable[i] {
                    continue;
                }

                if ingredients[i].iter().all(|ingredient| supplies.contains(ingredient)) {
                    creatable[i] = true;
                    new_supplies = true;
                    supplies.insert(recipes[i].clone());
                }
            }
        }

        recipes.into_iter()
            .zip(creatable.into_iter())
            .filter(|(_, creatable)| *creatable)
            .map(|(recipe, _)| recipe)
            .collect()
    }
}