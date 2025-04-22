use bevy::{prelude::*, ui::FocusPolicy};

pub mod drag_plane;
pub mod factory_world;
pub mod recipe_json;
pub mod recipe_ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    drag_plane::build(&mut app);
    recipe_ui::build(&mut app);
    factory_world::build(&mut app);

    app.insert_resource(ClearColor(Color::BLACK));
    app.add_systems(Startup, create_ui.in_set(CreateBaseUi));
    app.add_systems(Update, select_top_menus);

    app.run();
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreateBaseUi;

/// This entity is a horizontal flexbox that contains control menus
#[derive(Resource)]
pub struct ControlPanelUi {
    pub control_panel_entity: Entity,
}

/// This entity is a vertical flexbox containing text buttons for different menus.
#[derive(Resource)]
pub struct MainMenuUI {
    pub main_menu_entity: Entity,
}

#[derive(Component)]
#[require(Button)]
pub struct MainMenuItem {
    pub sub_menu_entity: Entity,
}

fn create_ui(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let root_ui_entity = commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .id();

    let control_panel_entity = commands
        .spawn((Node {
            height: Val::Percent(100.),
            overflow: Overflow::scroll_y(),
            ..default()
        },))
        .set_parent(root_ui_entity)
        .id();

    commands.insert_resource(ControlPanelUi {
        control_panel_entity,
    });

    commands
        .spawn((
            drag_plane::DragBoxPlane,
            Node {
                height: Val::Percent(100.),
                flex_grow: 1.,
                overflow: Overflow::clip(),
                ..default()
            },
            FocusPolicy::Block,
            BackgroundColor(Srgba::rgb(0.1, 0.1, 0.1).into()),
        ))
        .set_parent(root_ui_entity);

    let main_menu_entity = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .set_parent(control_panel_entity)
        .id();

    commands.insert_resource(MainMenuUI { main_menu_entity });
}

fn select_top_menus(
    interaction_q: Query<(Entity, &Interaction), (Changed<Interaction>, With<MainMenuItem>)>,
    menu_item_q: Query<&MainMenuItem>,
    mut node_q: Query<&mut Node>,
    mut active_item: Local<Option<Entity>>,
) {
    for (menu_item_entity, interaction) in interaction_q.iter() {
        let Interaction::Pressed = interaction else {
            continue;
        };

        if let Some(previous_active_item_entity) = active_item.take() {
            let menu_item = menu_item_q
                .get(previous_active_item_entity)
                .expect("Must be menu item");

            let mut node = node_q
                .get_mut(menu_item.sub_menu_entity)
                .expect("Must be a node");

            node.display = Display::None;

            if previous_active_item_entity == menu_item_entity {
                continue;
            }
        }

        let menu_item = menu_item_q
            .get(menu_item_entity)
            .expect("Must be menu item");

        let mut node = node_q
            .get_mut(menu_item.sub_menu_entity)
            .expect("Must be a node");

        node.display = Display::DEFAULT;

        *active_item = Some(menu_item_entity);
    }
}
