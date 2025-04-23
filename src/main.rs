use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod int;
mod ui;
use ui::enemy::EnemyPlugin;
use ui::resources::{CodeInput, PrintEvent, SpawnEvent};
use ui::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(CodeInput::default())
        .add_event::<PrintEvent>()
        .add_event::<SpawnEvent>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, floating_code_editor.after(spawn_camera))
        .add_systems(Update, run_code)
        .add_systems(Update, handle_print_event)
        .add_systems(Update, handle_spawn_event)
        .add_plugins(EnemyPlugin)
        .run();
}
