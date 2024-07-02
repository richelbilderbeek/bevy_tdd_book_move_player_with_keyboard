use crate::game_parameters::*;
use crate::player::*;
use bevy::prelude::*;

pub fn create_app(game_parameters: GameParameters) -> App {
    let mut app = App::new();
    let add_player_fn = move |/* no mut? */ commands: Commands| {
        add_player_from_parameters(commands, &game_parameters);
    };
    app.add_systems(Startup, add_player_fn);
    app.add_systems(Update, move_player);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    return app;
}

#[cfg(test)]
fn add_player(mut commands: Commands) {
    commands.spawn(create_default_player());
}

fn move_player(mut query: Query<(&mut Transform, &Player)>) {
    let (mut player_sprite, player) = query.single_mut();
    player_sprite.translation.x += player.velocity.x;
    player_sprite.translation.y += player.velocity.y;
}

fn add_player_from_parameters(mut commands: Commands, parameters: &GameParameters) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: parameters.initial_player_position,
                scale: parameters.initial_player_scale,
                ..default()
            },
            ..default()
        },
        Player {
            velocity: parameters.initial_player_velocity,
        },
    ));
}

#[cfg(test)]
fn count_n_players(app: &App) -> usize {
    let mut n = 0;
    for c in app.world.components().iter() {
        // The complete name will be '[crate_name]::Player'
        if c.name().contains("Player") {
            n += 1;
        }
    }
    return n;
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec3 {
    // Do 'app.update()' before calling this function,
    // else this assert goes off.
    assert_eq!(count_n_players(&app), 1);
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    return transform.translation;
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec3 {
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    return transform.scale;
}

#[cfg(test)]
fn print_all_components_names(app: &App) {
    for c in app.world.components().iter() {
        println!("{}", c.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app(create_default_game_parameters());
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let app = App::new();
        assert_eq!(count_n_players(&app), 0);
    }

    #[test]
    fn test_setup_player_adds_a_player() {
        let mut app = App::new();
        assert_eq!(count_n_players(&app), 0);
        app.add_systems(Startup, add_player);
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_player_is_at_custom_place() {
        let initial_coordinat = Vec3::new(1.2, 3.4, 5.6);
        let mut game_parameters = create_default_game_parameters();
        game_parameters.initial_player_position = initial_coordinat;
        let mut app = create_app(game_parameters);
        app.update();
        assert_eq!(get_player_coordinat(&mut app), initial_coordinat);
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let player_scale = Vec3::new(1.1, 2.2, 3.3);
        let mut game_parameters = create_default_game_parameters();
        game_parameters.initial_player_scale = player_scale;
        let mut app = create_app(game_parameters);
        app.update();
        assert_eq!(get_player_scale(&mut app), player_scale);
    }

    #[test]
    fn test_player_moves() {
        use create_default_game_parameters as create_params;
        let params = create_default_game_parameters_with_player_velocity(Vec2::new(1.1, 2.2));
        let mut app = create_app(params);
        app.update(); // Already moves the player
        assert_ne!(
            create_params().initial_player_position,
            get_player_coordinat(&mut app)
        );
    }

    #[test]
    fn test_player_moves_in_a_line() {
        use create_default_game_parameters as create_params;
        let velocity = Vec2::new(1.1, 2.2);
        let params = create_default_game_parameters_with_player_velocity(velocity);
        let mut app = create_app(params);
        app.update(); // Already moves the player
        let expected_pos =
            create_params().initial_player_position + Vec3::new(velocity.x, velocity.y, 0.0);
        assert_eq!(expected_pos, get_player_coordinat(&mut app));
    }

    #[test]
    fn test_print_all_components_names() {
        let mut app = create_app(create_default_game_parameters());
        app.update();
        print_all_components_names(&app);
    }
}
