use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod int;
mod ui;
use ui::resources::{CodeInput, PrintEvent};
use ui::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(CodeInput::default())
        .add_event::<PrintEvent>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, floating_code_editor.after(spawn_camera))
        .add_systems(Update, run_code)
        .add_systems(Update, handle_print_event)
        .run();
}
