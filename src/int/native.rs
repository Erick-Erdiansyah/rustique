use crate::int::lexeme::{NativeFn, Value};
use crate::ui::resources::{PrintEvent, SpawnEvent};
use bevy::ecs::{event::EventWriter, system::Commands};
use std::collections::HashMap;

#[macro_export]
macro_rules! native {
    ($name:ident) => {
        $name
            as fn(
                Vec<Value>,
                &mut EventWriter<PrintEvent>,
                &mut EventWriter<SpawnEvent>,
                &mut Commands,
            ) -> Option<Value>
    };
}

pub fn native_spawn_ball(
    _args: Vec<Value>,
    _print: &mut EventWriter<PrintEvent>,
    spawn: &mut EventWriter<SpawnEvent>,
    _commands: &mut Commands,
) -> Option<Value> {
    spawn.send(SpawnEvent {});
    None
}

pub fn build_native_fn_table() -> HashMap<String, NativeFn> {
    let mut table = HashMap::new();
    table.insert("spawn_ball".to_string(), native!(native_spawn_ball));
    table
}
