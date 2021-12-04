import re
import sys
from typing import Dict

ingredients_re = re.compile(r'^(.*) \(contains (.*)\)$')

recipes = []
for line in sys.stdin:
    m = ingredients_re.match(line.rstrip())
    if m:
        ingredients = m.group(1).split(' ')
        allergens = m.group(2).split(', ')
        recipes.append((ingredients, allergens))

recipes.sort(key=lambda recipe: -len(recipe[1]))

allergen_to_possible_ingredients = {}
allergen_to_ingredient: Dict[str, str] = {}
while not allergen_to_possible_ingredients or any([len(x) > 1 for x in allergen_to_possible_ingredients.values()]):
    print(allergen_to_possible_ingredients)
    for ingredients, allergens in recipes:
        for a in allergens:
            if a in allergen_to_possible_ingredients:
                allergen_to_possible_ingredients[a] = allergen_to_possible_ingredients[a].intersection(
                    ingredients)
            else:
                allergen_to_possible_ingredients[a] = set(ingredients)
    for a in allergen_to_possible_ingredients.keys():
        if len(allergen_to_possible_ingredients[a]) == 1:
            allergen_ingredient = next(iter(allergen_to_possible_ingredients[a]))
            allergen_to_ingredient[a] = allergen_ingredient
            for other_ingredients in allergen_to_possible_ingredients.values():
                if allergen_ingredient in other_ingredients:
                    other_ingredients.remove(allergen_ingredient)

print(f"Solved: {allergen_to_ingredient}")

ingredient_to_allergen = {i: a for a, i in allergen_to_ingredient.items()}
unused_occurrences = 0
for ingredients, allergen in recipes:
    unused_occurrences += len([i for i in ingredients if i not in ingredient_to_allergen])
print(f"{unused_occurrences} unused.")

print(f"Dangerous: {','.join([i for _, i in sorted(list(allergen_to_ingredient.items()))])}")