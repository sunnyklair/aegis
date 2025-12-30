use bevy::prelude::*;

// Menu constants
pub const MENU_ITEMS: &[&str] = &["Start Game", "Quit"];
pub const TITLE_Y: f32 = 200.0;
pub const MENU_START_Y: f32 = 50.0;
pub const MENU_SPACING: f32 = 50.0;
pub const CURSOR_X_OFFSET: f32 = -100.0;
pub const HOVER_THRESHOLD: f32 = 25.0;

// Tracks the currently selected menu item
#[derive(Component)]
pub struct MenuCursor {
    pub selected_index: usize,
}

// Marker component for menu item entities
#[derive(Component)]
pub struct MenuItem;

// Stores menu item entities in order (index = position in Vec)
#[derive(Resource)]
pub struct MenuItems(pub Vec<Entity>);

