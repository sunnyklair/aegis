use bevy::prelude::*;

// Menu constants
pub const MENU_ITEMS: &[&str] = &["Start Game", "Quit"];
pub const TITLE_FONT_SIZE: f32 = 48.0;
pub const MENU_FONT_SIZE: f32 = 24.0;
pub const MENU_SPACING: Val = Val::Px(10.0);

// Marker component for cursor indicator ("> ")
#[derive(Component)]
pub struct MenuCursor {
    pub item_index: usize,
}

// Marker component for menu item entities
#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
}

// Resource to track currently selected menu item
#[derive(Resource)]
pub struct SelectedMenuItem(pub usize);

