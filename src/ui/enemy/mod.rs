pub mod enemy_components;
pub mod enemy_events;
pub mod enemy_resources;
pub mod enemy_systems;

use bevy::prelude::*;
use enemy_systems::*;

use super::systems::handle_spawn_event;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_movement.after(handle_spawn_event),
                update_enemy_movement.after(enemy_movement),
                confine_enemy_movement.after(update_enemy_movement),
            ),
        );
    }
}
