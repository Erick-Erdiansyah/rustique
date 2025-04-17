use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod int;
mod ui;
use ui::components::CodeInput;
use ui::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(CodeInput::default())
        .add_systems(Update, floating_code_editor)
        .add_systems(Update, run_code)
        .add_systems(Startup, spawn_init_text)
        .run();
}
