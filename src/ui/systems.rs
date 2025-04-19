use crate::int::interpreter::run;
use crate::ui::resources::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{EguiContexts, egui};

use super::{
    components::{Printedtext, TextUI},
    style::{sample_ui_style, text_sample_ui_style},
};

pub fn spawn_camera(mut commands: Commands, windows_query: Query<&Window, With<PrimaryWindow>>) {
    let window = windows_query.get_single().unwrap();

    commands.spawn((
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Camera2d { ..default() },
    ));
}

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

pub fn run_code(
    mut input: ResMut<CodeInput>,
    writer: EventWriter<PrintEvent>,
    mut commands: Commands,
    text_query: Query<Entity, With<TextUI>>,
) {
    if input.run_requested {
        // despawn all entity before running the code
        for entity in text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        run(input.code.clone(), writer);
        input.run_requested = false;
    }
}

pub fn handle_print_event(mut commands: Commands, mut events: EventReader<PrintEvent>) {
    for ev in events.read() {
        spawn_text(&mut commands, ev.message.clone());
    }
}

pub fn spawn_text(commands: &mut Commands, text: impl Into<String>) -> Entity {
    let text = text.into();
    commands
        .spawn((sample_ui_style(), TextUI {}))
        .with_children(|parent| {
            parent
                .spawn((text_sample_ui_style(), BorderRadius::all(Val::Px(10.0))))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(&text),
                        TextFont {
                            font_size: 32.0,
                            ..Default::default()
                        },
                        TextColor::WHITE,
                        TextLayout::new_with_justify(JustifyText::Left),
                        Printedtext {},
                    ));
                });
        })
        .id()
}
