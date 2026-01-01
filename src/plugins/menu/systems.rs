use bevy::prelude::*;
use super::components::*;
use super::layout::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_menu)
            .add_systems(Update, (handle_menu_input, handle_menu_hover, update_cursor));
    }
}

/// Sets up the menu UI using flexbox layout
fn setup_menu(mut commands: Commands) {
    commands
        .spawn(root_container())
        .with_children(|parent| {
            spawn_title(parent);
            
            for (index, &text) in MENU_ITEMS.iter().enumerate() {
                spawn_menu_item(parent, index, text);
            }
        });
    
    commands.insert_resource(SelectedMenuItem(0));
}

/// Handles keyboard navigation
fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedMenuItem>,
) {
    let max_index = MENU_ITEMS.len() - 1;

    if keyboard.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        selected.0 = (selected.0 + 1).min(max_index);
    }
    if keyboard.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        selected.0 = selected.0.saturating_sub(1);
    }
}

/// Handles mouse hover using UI interaction
fn handle_menu_hover(
    menu_items: Query<(&Interaction, &MenuItem), Changed<Interaction>>,
    mut selected: ResMut<SelectedMenuItem>,
) {
    for (interaction, item) in &menu_items {
        if matches!(interaction, Interaction::Hovered) {
            selected.0 = item.index;
        }
    }
}

/// Updates cursor visibility based on selection
fn update_cursor(
    selected: Res<SelectedMenuItem>,
    mut cursors: Query<(&MenuCursor, &mut TextColor)>,
) {
    if !selected.is_changed() {
        return;
    }
    
    // Directly update all cursors - each knows its item index
    for (cursor, mut color) in &mut cursors {
        color.0 = if cursor.item_index == selected.0 {
            Color::WHITE
        } else {
            Color::NONE
        };
    }
}

