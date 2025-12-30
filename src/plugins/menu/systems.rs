use bevy::prelude::*;
use super::components::*;

// Main menu plugin with keyboard and mouse navigation
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_menu)
            .add_systems(Update, 
                (handle_menu_input, handle_menu_hover, update_menu_cursor_position).chain()
        );
    }
}

/// Sets up the menu UI with title, menu items, and cursor
pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Sudoku"),
        Transform::from_translation(Vec3::new(0.0, TITLE_Y, 0.0)),
    ));

    let menu_entities: Vec<Entity> = MENU_ITEMS
        .iter()
        .enumerate()
        .map(|(index, &item_text)| {
            let y_pos = MENU_START_Y - (index as f32 * MENU_SPACING);
            commands
                .spawn((
                    Text2d::new(item_text),
                    Transform::from_translation(Vec3::new(0.0, y_pos, 0.0)),
                    MenuItem,
                ))
                .id()
        })
        .collect();

    commands.insert_resource(MenuItems(menu_entities));

    commands.spawn((
        Text2d::new(">"),
        Transform::from_translation(Vec3::new(CURSOR_X_OFFSET, MENU_START_Y, 0.0)),
        MenuCursor { selected_index: 0 },
    ));
}

/// Handles keyboard navigation (arrow keys and WASD)
pub fn handle_menu_input(keyboard: Res<ButtonInput<KeyCode>>, mut cursor: Single<&mut MenuCursor>) {
    use KeyCode::*;
    let max_index = MENU_ITEMS.len() - 1;

    if keyboard.any_just_pressed([ArrowDown, KeyS]) {
        cursor.selected_index = (cursor.selected_index + 1).min(max_index);
    }

    if keyboard.any_just_pressed([ArrowUp, KeyW]) {
        cursor.selected_index = cursor.selected_index.saturating_sub(1);
    }
}

/// Handles mouse hover detection for menu items
pub fn handle_menu_hover(
    mut cursor_moved: MessageReader<CursorMoved>,
    camera: Single<(&Camera, &GlobalTransform)>,
    menu_items: Res<MenuItems>,
    transforms: Query<&Transform, With<MenuItem>>,
    mut menu_cursor: Single<&mut MenuCursor>,
) {
    let Some(cursor_event) = cursor_moved.read().last() else {
        return;
    };

    let world_pos = camera
        .0
        .viewport_to_world_2d(camera.1, cursor_event.position)
        .expect("viewport conversion should succeed");

    for (index, &entity) in menu_items.0.iter().enumerate() {
        if let Ok(item_transform) = transforms.get(entity) {
            let item_pos = item_transform.translation.truncate();

            if world_pos.distance(item_pos) < HOVER_THRESHOLD {
                menu_cursor.selected_index = index;
                break;
            }
        }
    }
}

/// Updates cursor position when selected menu item changes
pub fn update_menu_cursor_position(
    mut cursor_query: Query<(&MenuCursor, &mut Transform), Changed<MenuCursor>>,
    menu_items: Res<MenuItems>,
    transforms: Query<&Transform, Without<MenuCursor>>,
) {
    let Ok((menu_cursor, mut cursor_transform)) = cursor_query.single_mut() else {
        return;
    };

    // O(1) entity lookup instead of O(n) iteration
    if let Some(&entity) = menu_items.0.get(menu_cursor.selected_index) {
        if let Ok(item_transform) = transforms.get(entity) {
            cursor_transform.translation.y = item_transform.translation.y;
        }
    }
}

