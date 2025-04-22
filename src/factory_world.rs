use bevy::{prelude::*, utils::HashMap};

pub fn build(app: &mut App) {
    app.insert_resource(FactoryWorld::default());
}

#[derive(Default, Resource)]
pub struct FactoryWorld {
    next_resource_id: u32,
    resource_id_map: HashMap<String, ResourceId>,
    resource_name_map: HashMap<ResourceId, String>,
    next_recipe_id: u32,
    recipe_id_map: HashMap<String, RecipeId>,
    recipes: HashMap<RecipeId, Recipe>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecipeId(u32);

pub struct Recipe {
    pub name: String,
    pub ratios: Vec<(ResourceId, f32)>,
}

impl FactoryWorld {
    pub fn get_resource_id(&mut self, resource_name: &str) -> ResourceId {
        if let Some(id) = self.resource_id_map.get(resource_name) {
            *id
        } else {
            let id = ResourceId(self.next_resource_id);
            self.next_resource_id += 1;
            self.resource_id_map.insert(resource_name.to_string(), id);
            self.resource_name_map.insert(id, resource_name.to_string());
            id
        }
    }

    pub fn get_resource_name(&self, resource_id: ResourceId) -> Option<&str> {
        self.resource_name_map.get(&resource_id).map(String::as_str)
    }

    pub fn get_recipe_id(&mut self, recipe_name: &str) -> Option<RecipeId> {
        self.recipe_id_map.get(recipe_name).copied()
    }

    /// inserts a recipe, fails if there is already a recipe with the same name
    pub fn insert_recipe(&mut self, recipe: Recipe) -> Option<RecipeId> {
        let bevy::utils::Entry::Vacant(entry) = self.recipe_id_map.entry(recipe.name.clone())
        else {
            return None;
        };

        let id = RecipeId(self.next_recipe_id);
        self.next_recipe_id += 1;
        entry.insert(id);

        self.recipes.insert(id, recipe);

        Some(id)
    }

    pub fn get_recipe(&self, recipe_id: RecipeId) -> Result<&Recipe, InvalidRecipeError> {
        self.recipes.get(&recipe_id).ok_or(InvalidRecipeError)
    }

    pub fn remove_recipe(&mut self, recipe_id: RecipeId) -> Option<Recipe> {
        let recipe = self.recipes.remove(&recipe_id)?;

        self.recipe_id_map
            .remove(&recipe.name)
            .expect("Should have name in map");

        Some(recipe)
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
