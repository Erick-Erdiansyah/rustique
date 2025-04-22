use crate::int::lexeme::{NativeFn, Value};
use crate::ui::resources::PrintEvent;
use bevy::ecs::event::EventWriter;
use std::collections::HashMap;

pub fn native_move_left(_args: Vec<Value>, writer: &mut EventWriter<PrintEvent>) -> Option<Value> {
    println!("fucj");
    writer.send(PrintEvent {
      message: "moving left".to_string(),
    });
    println!("fucj after");
    None
}

pub fn build_native_fn_table() -> HashMap<String, NativeFn> {
  let mut table = HashMap::new();
  table.insert("move.left".to_string(), native_move_left as NativeFn);
  println!("fucdsj");
    table
}
