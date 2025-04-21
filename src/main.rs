use bevy::{color::palettes::css::*, prelude::*, ui::FocusPolicy};
use drag_plane::create_drag_box;

pub mod drag_plane;
pub mod factory;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    drag_plane::build(&mut app);

    app.insert_resource(ClearColor(Color::BLACK));
    app.add_systems(Startup, create_ui);

    app.run();
}

#[derive(Resource)]
pub struct ControlPanelUi {
    pub control_panel_entity: Entity,
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
        .spawn((
            Node {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            FocusPolicy::Block,
        ))
        .set_parent(root_ui_entity)
        .id();

    commands.insert_resource(ControlPanelUi {
        control_panel_entity,
    });

    let root_plane_entity = commands
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
        .set_parent(root_ui_entity)
        .id();

    let box_a = commands.spawn_empty().set_parent(root_plane_entity).id();
    let box_a_ui = create_drag_box(&mut commands, box_a);
    commands.entity(box_a_ui).with_children(|builder| {
        builder.spawn((
            Node {
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            BackgroundColor(GREEN.into()),
        ));
    });

    let box_b = commands.spawn_empty().set_parent(root_plane_entity).id();
    let box_b_ui = create_drag_box(&mut commands, box_b);
    commands.entity(box_b_ui).with_children(|builder| {
        builder.spawn((
            Node {
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            BackgroundColor(ORANGE.into()),
        ));
    });
}
