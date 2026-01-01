use bevy::prelude::*;
use bevy::hierarchy::ChildBuilder;
use super::components::*;

/// Helper to spawn the menu title
pub fn spawn_title(parent: &mut ChildBuilder) {
    parent.spawn((
        Text::new("Sudoku"),
        TextFont {
            font_size: TITLE_FONT_SIZE,
            ..default()
        },
        Node {
            margin: UiRect::bottom(Val::Px(60.0)),
            ..default()
        },
    ));
}

/// Helper to spawn a single menu item with cursor
pub fn spawn_menu_item(parent: &mut ChildBuilder, index: usize, text: &str) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::all(MENU_SPACING),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            MenuItem { index },
            Interaction::default(),
        ))
        .with_children(|item_parent: &mut ChildBuilder| {
            // Cursor indicator
            item_parent.spawn((
                Text::new("> "),
                TextFont {
                    font_size: MENU_FONT_SIZE,
                    ..default()
                },
                TextColor(if index == 0 { Color::WHITE } else { Color::NONE }),
                MenuCursor { item_index: index },
            ));
            
            // Menu item text
            item_parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: MENU_FONT_SIZE,
                    ..default()
                },
            ));
        });
}

/// Helper to create the root menu container node
pub fn root_container() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

