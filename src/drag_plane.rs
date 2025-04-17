use bevy::{
    color::palettes::css::*, input::mouse::AccumulatedMouseScroll, prelude::*, ui::FocusPolicy,
    window::PrimaryWindow,
};

pub fn build(app: &mut App) {
    app.insert_resource(PlaneScroll {
        scroll_position: Vec2::ZERO,
    });

    app.add_systems(
        Update,
        (
            move_drag_boxes,
            update_plane_drag,
            update_plane_scroll,
            update_drag_box_positions,
        ),
    );
}

pub fn create_drag_box(commands: &mut Commands, drag_box_entity: Entity) -> Entity {
    commands.entity(drag_box_entity).insert((
        DragBox {
            position: default(),
        },
        FocusPolicy::Block,
        Node {
            position_type: PositionType::Absolute,
            ..default()
        },
    ));

    let mut child_ui_root = Entity::PLACEHOLDER;

    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .set_parent(drag_box_entity)
        .with_children(|builder| {
            builder.spawn((
                Text::new("Header"),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                Node {
                    width: Val::Percent(100.),
                    ..default()
                },
                BackgroundColor(RED.into()),
                BorderRadius::top(Val::Px(5.)),
            ));

            child_ui_root = builder.spawn((Node::default(), FocusPolicy::Block)).id();
        });

    child_ui_root
}

#[derive(Resource)]
pub struct PlaneScroll {
    pub scroll_position: Vec2,
}

#[derive(Component)]
#[require(Node, Interaction)]
pub struct DragBoxPlane;

#[derive(Component)]
#[require(Node, Interaction)]
pub struct DragBox {
    position: Vec2,
}

struct CurrentBoxDrag {
    drag_box_entity: Entity,
    start_mouse_position: Vec2,
    start_box_position: Vec2,
}

fn move_drag_boxes(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut drag_box_q: Query<(Entity, &mut DragBox, &Interaction)>,
    mut current_drag: Local<Option<CurrentBoxDrag>>,
) {
    let window = window_q.single();
    let mouse_position = window.cursor_position();

    if mouse_input.just_pressed(MouseButton::Left) {
        for (drag_box_entity, &DragBox { position, .. }, interaction) in drag_box_q.iter() {
            if let Interaction::Pressed = interaction {
                *current_drag = Some(CurrentBoxDrag {
                    drag_box_entity,
                    start_mouse_position: mouse_position.unwrap_or_default(),
                    start_box_position: position,
                });

                break;
            }
        }
    }

    if mouse_input.just_released(MouseButton::Left) {
        *current_drag = None;
    }

    if let Some(CurrentBoxDrag {
        drag_box_entity,
        start_mouse_position,
        start_box_position,
    }) = *current_drag
    {
        let Ok((_, mut drag_box, _)) = drag_box_q.get_mut(drag_box_entity) else {
            error!("Couldn't query drag box {}", drag_box_entity);
            return;
        };

        if let Some(mouse_position) = mouse_position {
            let mouse_delta = mouse_position - start_mouse_position;
            drag_box.position = start_box_position + mouse_delta;
        }
    }
}

fn update_drag_box_positions(
    mut drag_box_q: Query<(&DragBox, &mut Node)>,
    plane_scroll: Res<PlaneScroll>,
) {
    for (drag_box, mut node) in drag_box_q.iter_mut() {
        node.left = Val::Px(drag_box.position.x + plane_scroll.scroll_position.x);
        node.top = Val::Px(drag_box.position.y + plane_scroll.scroll_position.y);
    }
}

struct CurrentPlaneDrag {
    start_mouse_position: Vec2,
    start_plane_position: Vec2,
}

fn update_plane_drag(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    drag_box_plane_q: Query<&Interaction, With<DragBoxPlane>>,
    mut plane_scroll: ResMut<PlaneScroll>,
    mut current_drag: Local<Option<CurrentPlaneDrag>>,
) {
    let window = window_q.single();
    let mouse_position = window.cursor_position();

    if mouse_input.just_pressed(MouseButton::Left) {
        let Ok(interaction) = drag_box_plane_q.get_single() else {
            error!("Couldn't query drag box plane");
            return;
        };

        if let Interaction::Pressed = interaction {
            *current_drag = Some(CurrentPlaneDrag {
                start_mouse_position: mouse_position.unwrap_or_default(),
                start_plane_position: plane_scroll.scroll_position,
            });
        }
    }

    if mouse_input.just_released(MouseButton::Left) {
        *current_drag = None;
    }

    if let Some(CurrentPlaneDrag {
        start_mouse_position,
        start_plane_position,
    }) = *current_drag
    {
        if let Some(mouse_position) = mouse_position {
            let mouse_delta = mouse_position - start_mouse_position;
            plane_scroll.scroll_position = start_plane_position + mouse_delta;
        }
    }
}

fn update_plane_scroll(
    scroll_input: Res<AccumulatedMouseScroll>,
    drag_box_plane_q: Query<&Interaction, With<DragBoxPlane>>,
    mut plane_scroll: ResMut<PlaneScroll>,
) {
    if let Interaction::Hovered = drag_box_plane_q.single() {
        plane_scroll.scroll_position += scroll_input.delta;
    }
}
