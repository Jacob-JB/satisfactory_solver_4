use serde::Deserialize;

use crate::factory_world::*;

#[derive(Deserialize)]
pub struct RecipeListJson<'a> {
    #[serde(borrow)]
    pub recipes: Vec<RecipeJson<'a>>,
}

#[derive(Deserialize)]
pub struct RecipeJson<'a> {
    pub name: &'a str,
    pub rates: Vec<(&'a str, f32)>,
}

pub fn load_recipes<'json>(
    world: &mut FactoryWorld,
    json: &'json str,
) -> Result<Vec<Result<RecipeId, &'json str>>, serde_json::Error> {
    let parsed_json: RecipeListJson = serde_json::from_str(json)?;

    Ok(parsed_json
        .recipes
        .into_iter()
        .map(|RecipeJson { name, rates }| {
            let recipe = Recipe {
                name: name.to_string(),
                ratios: rates
                    .into_iter()
                    .map(|(resource_name, rate)| {
                        let resource_id = world.get_resource_id(resource_name);
                        (resource_id, rate)
                    })
                    .collect(),
            };

            match world.insert_recipe(recipe) {
                Some(id) => Ok(id),
                None => Err(name),
            }
        })
        .collect())
}
