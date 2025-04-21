use serde::Deserialize;

use super::world::*;

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

#[derive(Debug)]
pub enum OpenRecipesError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

pub fn load_recipes(
    world: &mut World,
    path: impl AsRef<std::path::Path>,
) -> Result<Vec<RecipeId>, OpenRecipesError> {
    let json = std::fs::read_to_string(path).map_err(|e| OpenRecipesError::Io(e))?;

    let parsed_json: RecipeListJson =
        serde_json::from_str(&json).map_err(|e| OpenRecipesError::Json(e))?;

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

            world.insert_recipe(recipe)
        })
        .collect())
}
