use crate::int::interpreter::run;
use crate::ui::resources::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{EguiContexts, egui};

use super::{
    components::{Printedtext, TextUI},
    enemy::{
        enemy_components::Enemy,
        enemy_systems::{despawn_enemies, spawn_enemies},
    },
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
    egui::Window::new("AUTO")
        .default_pos((20.0, 20.0))
        .show(contexts.ctx_mut(), |ui| {
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
    writer_print: EventWriter<PrintEvent>,
    writer_spawn: EventWriter<SpawnEvent>,
    mut commands: Commands,
    text_query: Query<Entity, With<TextUI>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if input.run_requested {
        // despawn all entity before running the code
        for entity in text_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        despawn_enemies(commands.reborrow(), enemy_query);
        run(input.code.clone(), writer_print, writer_spawn, commands);
        input.run_requested = false;
    }
}

pub fn handle_print_event(mut commands: Commands, mut events: EventReader<PrintEvent>) {
    for ev in events.read() {
        spawn_text(&mut commands, ev.message.clone());
    }
}

pub fn handle_spawn_event(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut events: EventReader<SpawnEvent>,
) {
    if events.read().next().is_some() {
        spawn_enemies(&mut commands, &window_query, &asset_server);
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
