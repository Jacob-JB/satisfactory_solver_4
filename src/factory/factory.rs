use bevy::utils::HashMap;

use super::world::*;

pub struct Factory {
    pub recipes: HashMap<RecipeId, f32>,
}

impl Factory {
    pub fn calculate_net_production(
        &self,
        world: &World,
    ) -> Result<HashMap<ResourceId, f32>, InvalidRecipeError> {
        let mut net_production = HashMap::new();

        for (&recipe_id, recipe_ratio) in &self.recipes {
            let recipe = world.get_recipe(recipe_id)?;

            for (resource_id, resource_ratio) in recipe.iter_ratios() {
                *net_production.entry(resource_id).or_default() += recipe_ratio * resource_ratio;
            }
        }

        Ok(net_production)
    }
}
