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

pub fn run_code(mut input: ResMut<CodeInput>, writer: EventWriter<PrintEvent>) {
    if input.run_requested {
        run(input.code.clone(),writer);
        input.run_requested = false;
    }
}

pub fn handle_print_event(mut events: EventReader<PrintEvent>) {
    for ev in events.read() {
        
    }
}

pub fn spawn_text(mut commands: Commands, text: String) {
    let _text_entity = build_text(&mut commands, &text);
}
pub fn despawn_text(mut commands: Commands, text_query: Query<Entity, With<TextUI>>) {
    if let Ok(text_entity) = text_query.get_single() {
        commands.entity(text_entity).despawn_recursive();
    }
}
pub fn build_text(commands: &mut Commands, text: &String) -> Entity {
    let build_text = commands
        .spawn((sample_ui_style(), TextUI {}))
        .with_children(|parent| {
            parent
                .spawn((text_sample_ui_style(), BorderRadius::all(Val::Px(10.0))))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(text),
                        TextColor::WHITE,
                        TextLayout::new_with_justify(JustifyText::Left),
                        Printedtext {},
                    ));
                });
        })
        .id();
    build_text
}
