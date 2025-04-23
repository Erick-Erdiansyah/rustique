use bevy::prelude::*;
#[derive(Resource, Default)]
pub struct CodeInput {
    pub code: String,
    pub run_requested: bool,
}

#[derive(Debug, Clone, Event)]
pub struct PrintEvent {
    pub message: String,
}
#[derive(Debug, Clone, Event)]
pub struct SpawnEvent {}
