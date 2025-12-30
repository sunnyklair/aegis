mod plugins;
use plugins::MenuPlugin;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    SplashIntro,
    MainMenu,
    //Game,
}

fn main() {
    let mut app = App::new();

     app.add_systems(Startup, setup)
        .add_plugins((DefaultPlugins, MenuPlugin))
        .init_state::<GameState>();

    #[cfg(debug_assertions)] { // Debug or dev builds only
        use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
        use bevy::text::{TextFont, FontSmoothing};

        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 24.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    min_fps: 30.0,
                    target_fps: 60.0,
                },
            },
        });
    }

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
