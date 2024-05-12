use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        query::{With, Without},
        system::{Commands, Query},
    },
    hierarchy::BuildChildren,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        JustifyContent, Style, Val,
    },
    utils::default,
};

use crate::components::{
    player::Player,
    ui::{PlayerHealthCounter, PlayerPositionCounter},
    Health, Position,
};

pub struct LithiumUISystem;

impl Plugin for LithiumUISystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_ui)
            .add_systems(Update, update_ui);
    }
}

fn initialize_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            // Node of the entire screen
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Health
            parent.spawn((
                TextBundle::from_section(
                    "Health: -",
                    TextStyle {
                        font: Default::default(),
                        font_size: 30.0,
                        ..default()
                    },
                ),
                PlayerHealthCounter {},
            ));
            // Position
            parent.spawn((
                TextBundle::from_section(
                    "Position: (-, -)",
                    TextStyle {
                        font: Default::default(),
                        font_size: 30.0,
                        ..default()
                    },
                ),
                PlayerPositionCounter {},
            ));
        });
}

fn update_ui(
    // mut position_text_query: Query<&mut Text, (With<PlayerPositionCounter>, Without<PlayerHealthCounter>)>,
    mut health_text_query: Query<
        &mut Text,
        (With<PlayerHealthCounter>, Without<PlayerPositionCounter>),
    >,
    player_query: Query<(&Position, &Health), With<Player>>,
) {
    let (/*player_pos*/ _, player_health) = player_query.single();

    // NOTE: This fucks up the player rendering for some reason, use just if needed for debugging
    // let mut pos_text = position_text_query.single_mut();
    // *pos_text = Text::from_section(
    //     format!("Position: ({}, {})", player_pos.value.x, player_pos.value.y),
    //     TextStyle {
    //         font: Default::default(),
    //         font_size: 30.0,
    //         ..default()
    //     });

    let mut health_text = health_text_query.single_mut();
    *health_text = Text::from_section(
        format!("Health: {}", player_health.get_val()),
        TextStyle {
            font: Default::default(),
            font_size: 30.0,
            ..default()
        },
    )
}
