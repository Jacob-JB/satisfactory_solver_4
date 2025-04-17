use bevy::utils::HashMap;

#[derive(Default)]
pub struct World {
    next_resource_id: u32,
    resource_id_map: HashMap<String, ResourceId>,
    next_recipe_id: u32,
    recipes: HashMap<RecipeId, Recipe>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecipeId(u32);

pub struct Recipe {
    pub ratios: Vec<(ResourceId, f32)>,
}

impl World {
    pub fn get_resource_id(&mut self, resource_name: &str) -> ResourceId {
        if let Some(id) = self.resource_id_map.get(resource_name) {
            *id
        } else {
            let id = ResourceId(self.next_resource_id);
            self.next_resource_id += 1;
            self.resource_id_map.insert(resource_name.to_string(), id);
            id
        }
    }

    pub fn insert_recipe(&mut self, recipe: Recipe) -> RecipeId {
        let id = RecipeId(self.next_recipe_id);
        self.next_recipe_id += 1;
        self.recipes.insert(id, recipe);
        id
    }

    pub fn get_recipe(&self, recipe_id: RecipeId) -> Result<&Recipe, InvalidRecipeError> {
        self.recipes.get(&recipe_id).ok_or(InvalidRecipeError)
    }
}

impl Recipe {
    pub fn iter_ratios(&self) -> impl Iterator<Item = (ResourceId, f32)> {
        self.ratios.iter().copied()
    }
}

pub struct InvalidRecipeError;

impl std::fmt::Debug for InvalidRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InvalidRecipeError")
    }
}
