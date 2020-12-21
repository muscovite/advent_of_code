use std::collections::HashMap;
use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut allergen_to_ingredients = HashMap::new();
    let mut ingredient_count = HashMap::new();

    input.trim().lines().for_each(|l| {
        let mut parts = l.split(" (contains ");

        // ingredients
        let ingredients: HashSet<_> = parts.next().unwrap().split_whitespace().collect();
        ingredients.iter().for_each(|ingredient| {
            ingredient_count.entry(*ingredient).and_modify(|v| *v += 1).or_insert(1);
        });

        // allergens 
        let allergens = parts.next().unwrap().split_whitespace().map(|tok| &tok[..tok.len()-1]);
        allergens.for_each(|allergen| {
            // &*v -> have a &mut, so need to deref and re-ref to get just a reference for HashSet logic
            allergen_to_ingredients.entry(allergen).and_modify(|v| {*v = &*v & &ingredients}).or_insert(ingredients.clone());
        });
    });

    let mut bad_stuff = Vec::new();
    while allergen_to_ingredients.len() > 0 {
        // find allergen with one ingredient
        let (_, bad) = allergen_to_ingredients.iter().find(|&(_, v)| v.len() == 1).unwrap();
        let bad = *bad.iter().nth(0).unwrap();
        
        bad_stuff.push(bad);
        
        // remove ingredient from rest of ingredient sets
        allergen_to_ingredients.iter_mut().for_each(|(_, v)| {
            v.remove(bad);
        });

        // remove any allergens with an empty ingredients list
        allergen_to_ingredients.retain(|_, v| v.len() > 0);
    }

    ingredient_count.iter().filter_map(|(k, v)| {
        if bad_stuff.contains(k) {
            return None
        }
        return Some(*v)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(solve(input), 5);
    }
}

advent_2020::read_main!();
