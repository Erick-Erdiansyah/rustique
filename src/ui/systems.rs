use crate::int::interpreter::run;
use crate::ui::components::*;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn floating_code_editor(mut contexts: EguiContexts, mut input: ResMut<CodeInput>) {
    egui::Window::new("Code Box")
        .default_pos((20.0, 20.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Enter code:");
            ui.add(
                egui::TextEdit::multiline(&mut input.code)
                    .desired_width(f32::INFINITY)
                    .desired_rows(10),
            );
            if ui.button("Run").clicked() {
                input.run_requested = true;
            }
        });
}

pub fn spawn_init_text(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("Text - the one"),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
    ));
}
pub fn _spawn_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Text looooooooooooooooooooooooooooooooooooooooooooong"),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
    ));
}

pub fn run_code(mut input: ResMut<CodeInput>) {
    if input.run_requested {
        run(input.code.clone());
        input.run_requested = false;
    }
}
