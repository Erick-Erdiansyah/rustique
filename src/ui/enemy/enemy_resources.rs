use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 10.0;


#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub _timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            _timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
