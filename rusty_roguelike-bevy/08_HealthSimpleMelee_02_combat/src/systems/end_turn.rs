use crate::prelude::*;

pub fn end_turn(mut commands: Commands, turn_state: Res<CurrentState<TurnState>>) {
    let new_state = match turn_state.0 {
        // In the source project, AwaitingInput returns AwaitingInput, however, it's actually an unreachable
        // case, because the change to the next state (PlayerTurn) is performed in the `player_input` system.
        TurnState::AwaitingInput => unreachable!(),
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    commands.insert_resource(NextState(new_state));
}
