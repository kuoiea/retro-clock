use bevy::prelude::*;
use chrono::Timelike;

fn clock_face(mut gizmos: Gizmos) {
    let time_now = chrono::Local::now();
    let hour = time_now.hour() as f32;
    let minute = time_now.minute() as f32;
    let second = time_now.second() as f32;

    let minute_angle = ((360.0/60.0) * minute).to_radians();
    let second_angle = ((360.0/60.0) * second).to_radians();
    let hour_angle = ((360.0/24.0) * hour).to_radians();

    // second
    gizmos.arc_2d(Vec2::ZERO, second_angle/2.0, second_angle, 100., Color::BISQUE)
        .segments(360 * 3);
    // minute
    gizmos.arc_2d(Vec2::ZERO, minute_angle/2.0, minute_angle, 120., Color::TEAL)
        .segments(360 * 3);
    //hour
    gizmos.arc_2d(Vec2::ZERO, hour_angle/2.0, hour_angle, 140., Color::ORANGE)
        .segments(360 * 3);
}


fn setup(mut commands: Commands, mut gizmo_config: ResMut<GizmoConfig>){
    commands.spawn(Camera2dBundle::default());
    gizmo_config.line_width = 20.0;


}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, clock_face)
        .run();
}
