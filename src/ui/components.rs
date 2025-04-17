use bevy::prelude::*;
#[derive(Resource, Default)]
pub struct CodeInput {
    pub code: String,
    pub run_requested: bool,
}
