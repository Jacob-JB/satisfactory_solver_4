use bevy::{
    color::palettes::css::*,
    prelude::*,
    tasks::{IoTaskPool, Task, block_on, poll_once},
};

use crate::{
    ControlPanelUi, CreateBaseUi, MainMenuItem, MainMenuUI,
    drag_plane::{DragBox, DragBoxPlane, PlaneScroll},
    factory_world::*,
};

use crate::recipe_json::load_recipes;

pub fn build(app: &mut App) {
    app.add_systems(Startup, create_recipe_ui.after(CreateBaseUi));
    app.add_systems(
        Update,
        (
            start_load_recipes_dialogue,
            finish_load_recipes_dialogue,
            remove_recipe_ui,
            insert_recipe_ui,
            despawn_recipes,
        ),
    );
}

#[derive(Resource)]
pub struct RecipeList {
    pub recipe_list_entity: Entity,
}

#[derive(Component)]
#[require(Button)]
struct LoadRecipesButton;

#[derive(Component)]
#[require(Button)]
struct RemoveRecipeButton {
    list_item_entity: Entity,
    recipe_id: RecipeId,
}

#[derive(Component)]
#[require(Button)]
struct InsertRecipeButton {
    recipe_id: RecipeId,
}

#[derive(Component)]
#[require(Button)]
struct DespawnRecipeButton {
    box_entity: Entity,
}

fn create_recipe_ui(
    mut commands: Commands,
    main_menu: Res<MainMenuUI>,
    control_panel: Res<ControlPanelUi>,
) {
    let sub_menu_entity = commands
        .spawn((Node {
            display: Display::None,
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .set_parent(control_panel.control_panel_entity)
        .id();

    commands
        .spawn((
            MainMenuItem { sub_menu_entity },
            Text::new("Recipes"),
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
        ))
        .set_parent(main_menu.main_menu_entity);

    commands
        .spawn((LoadRecipesButton, Text::new("Load Recipes")))
        .set_parent(sub_menu_entity);

    let recipe_list_entity = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .set_parent(sub_menu_entity)
        .id();

    commands.insert_resource(RecipeList { recipe_list_entity });
}

#[derive(Component)]
struct LoadRecipesTask {
    task: Task<Option<String>>,
}

fn start_load_recipes_dialogue(
    mut commands: Commands,
    button_q: Query<&Interaction, (Changed<Interaction>, With<LoadRecipesButton>)>,
) {
    let Ok(Interaction::Pressed) = button_q.get_single() else {
        return;
    };

    info!("Loading recipes");

    commands.spawn(LoadRecipesTask {
        task: IoTaskPool::get().spawn(load_recipes_async()),
    });
}

async fn load_recipes_async() -> Option<String> {
    let file_handle = rfd::AsyncFileDialog::new().pick_file().await;

    if let Some(file_handle) = file_handle {
        let bytes = file_handle.read().await;

        String::from_utf8(bytes).ok()
    } else {
        None
    }
}

fn finish_load_recipes_dialogue(
    mut commands: Commands,
    mut task_q: Query<(Entity, &mut LoadRecipesTask)>,
    mut world: ResMut<FactoryWorld>,
    recipe_list: Res<RecipeList>,
) {
    for (task_entity, mut task) in task_q.iter_mut() {
        if task.task.is_finished() {
            let Some(task_result) = block_on(poll_once(&mut task.task)) else {
                error!("Expected task to be finished");
                continue;
            };

            commands.entity(task_entity).despawn_recursive();

            let Some(json) = task_result else {
                continue;
            };

            let Ok(load_results) = load_recipes(world.as_mut(), &json) else {
                error!("Invalid JSON");
                continue;
            };

            for load_result in load_results {
                let recipe_id = match load_result {
                    Ok(recipe_id) => recipe_id,
                    Err(recipe_name) => {
                        error!("Failed to load recipe \"{}\"", recipe_name);
                        continue;
                    }
                };

                let recipe = world.get_recipe(recipe_id).expect("Recipe was just added");

                commands
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            RemoveRecipeButton {
                                list_item_entity: builder.parent_entity(),
                                recipe_id,
                            },
                            Text::new("X"),
                        ));

                        builder.spawn((InsertRecipeButton { recipe_id }, Text::new(&recipe.name)));
                    })
                    .set_parent(recipe_list.recipe_list_entity);
            }
        }
    }
}

fn remove_recipe_ui(
    mut commands: Commands,
    button_q: Query<(&Interaction, &RemoveRecipeButton), Changed<Interaction>>,
    mut world: ResMut<FactoryWorld>,
) {
    for (interaction, button) in button_q.iter() {
        let Interaction::Pressed = interaction else {
            continue;
        };

        world.remove_recipe(button.recipe_id);

        commands.entity(button.list_item_entity).despawn_recursive();
    }
}

fn insert_recipe_ui(
    mut commands: Commands,
    button_q: Query<(&Interaction, &InsertRecipeButton), Changed<Interaction>>,
    world: Res<FactoryWorld>,
    plane_q: Query<Entity, With<DragBoxPlane>>,
    plane_scroll: Res<PlaneScroll>,
) {
    for (interaction, button) in button_q.iter() {
        let Interaction::Pressed = interaction else {
            continue;
        };

        info!("inserting recipe {:?}", button.recipe_id);

        let recipe = world
            .get_recipe(button.recipe_id)
            .expect("Recipe should be in world");

        let root_plane_entity = plane_q.get_single().expect("Should be one root plane");

        commands
            .spawn((
                DragBox {
                    position: -plane_scroll.scroll_position,
                },
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ))
            .set_parent(root_plane_entity)
            .with_children(|builder| {
                let box_entity = builder.parent_entity();

                builder
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|builder| {
                        // header
                        builder
                            .spawn(Node {
                                width: Val::Percent(100.),
                                flex_direction: FlexDirection::Row,
                                ..default()
                            })
                            .with_children(|builder| {
                                builder.spawn((
                                    Text::new(&recipe.name),
                                    TextLayout {
                                        justify: JustifyText::Center,
                                        ..default()
                                    },
                                    Node {
                                        flex_grow: 1.,
                                        ..default()
                                    },
                                    BackgroundColor(RED.into()),
                                ));

                                builder.spawn((DespawnRecipeButton { box_entity }, Text::new("X")));
                            });

                        // content
                        builder
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                BackgroundColor(DARK_GREY.into()),
                            ))
                            .with_children(|builder| {
                                for side in [false, true] {
                                    builder
                                        .spawn(Node {
                                            display: Display::Grid,
                                            grid_template_columns: RepeatedGridTrack::auto(2),
                                            ..default()
                                        })
                                        .with_children(|builder| {
                                            for (resource_id, ratio) in recipe.iter_ratios() {
                                                let resource_name = world
                                                    .get_resource_name(resource_id)
                                                    .expect("Resource name should exist");

                                                if (ratio < 0.) ^ side {
                                                    builder.spawn(Text::new(resource_name));

                                                    builder
                                                        .spawn(Text::new(format!("{:.2}", ratio)));
                                                }
                                            }
                                        });
                                }
                            });
                    });
            });
    }
}

fn despawn_recipes(
    mut commands: Commands,
    button_q: Query<(&DespawnRecipeButton, &Interaction), Changed<Interaction>>,
) {
    for (button, interaction) in button_q.iter() {
        let Interaction::Pressed = interaction else {
            continue;
        };

        commands.entity(button.box_entity).despawn_recursive();
    }
}
