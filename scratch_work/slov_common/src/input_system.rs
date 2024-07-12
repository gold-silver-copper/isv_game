use crate::*;
pub fn keyboard_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut player_eid: Query<(Entity), With<Player>>,
    mut commands: Commands,
) {
    let char_up = input.any_pressed([KeyCode::KeyW]);
    let char_down = input.any_pressed([KeyCode::KeyS]);
    let char_left = input.any_pressed([KeyCode::KeyA]);
    let char_right = input.any_pressed([KeyCode::KeyD]);

    let cursor_up = input.any_just_pressed([KeyCode::KeyI]);
    let cursor_down = input.any_just_pressed([KeyCode::KeyK]);
    let cursor_left = input.any_just_pressed([KeyCode::KeyJ]);
    let cursor_right = input.any_just_pressed([KeyCode::KeyL]);

    let char_attack = input.any_just_pressed([KeyCode::KeyY]); // jęti (jme) / vzeti
    let char_take = input.any_just_pressed([KeyCode::KeyG]); // jęti (jme) / vzeti metnuti  imej target range do ktorogo mozno metati dla praktiki zeby povysati skil be ubijstva
    let char_drop = input.any_just_pressed([KeyCode::KeyB]); //izbaviti se
    let char_help = input.any_just_pressed([KeyCode::KeyT]); //pokazati pomoc ?

    let char_one = input.any_just_pressed([KeyCode::Digit1]);
    let char_two = input.any_just_pressed([KeyCode::Digit2]);
    let char_three = input.any_just_pressed([KeyCode::Digit3]);
    let char_four = input.any_just_pressed([KeyCode::Digit4]);
    let char_five = input.any_just_pressed([KeyCode::Digit5]);
    let char_six = input.any_just_pressed([KeyCode::Digit6]);
    let char_seven = input.any_just_pressed([KeyCode::Digit7]);
    let char_eight = input.any_just_pressed([KeyCode::Digit8]);
    let char_nine = input.any_just_pressed([KeyCode::Digit9]);
    let char_zero = input.any_just_pressed([KeyCode::Digit0]);

    let char_backspace = input.any_pressed([KeyCode::Backspace, KeyCode::Delete]);
    let char_quit = input.any_just_pressed([KeyCode::KeyQ]);

    let mut p_eid = player_eid.single();

    if char_up {
        commands
            .entity(p_eid)
            .insert(ActionComponent::Go(CardinalDirection::North));
    }
    if char_down {
        commands
            .entity(p_eid)
            .insert(ActionComponent::Go(CardinalDirection::South));
    }
    if char_left {
        commands
            .entity(p_eid)
            .insert(ActionComponent::Go(CardinalDirection::West));
    }
    if char_right {
        commands
            .entity(p_eid)
            .insert(ActionComponent::Go(CardinalDirection::East));
    }

    if char_quit {
        panic!("BYE");
    }
}
