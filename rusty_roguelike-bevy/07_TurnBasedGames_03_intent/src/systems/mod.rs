use crate::prelude::*;

mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod movement;
mod player_input;
mod random_move;

pub fn build_system_sets(app: &mut App) {
    use GameStage::*;
    use TurnState::*;

    // As of v0.7, it's not possible to flush commands on-demand, like Legion does; Bevy flushes the
    // commands only at the end of each stage. Although this approach is not so immediate, it still
    // allows a cleanly structured model; we isolate each block of systems that performs a change, and
    // put it inside a system set.
    //
    // It's techically possible to write multiple schedulers like Legion, and update them on the fly
    // (since we're in charge of the main loop), however, it's more complicated than Legion, and additionally,
    // it doesn't offer any concrete advantage over system sets.
    //
    // A very serious limitation in the current Bevy version is that states/stages are essentially unusable
    // together (and also in other conditions, e.g. with FixedTimeStep), therefore, it's necessary to
    // use the crate `iyes_loopless`, which implements this functionality without limitations.
    //
    // The states modeled in the port are the same three as the source project, although technically,
    // only two states are required, since PlayerTurn and MonsterTurn always execute together.
    //
    // There are a few concepts that are modeled differently here, which are all interrelated:
    //
    // - instead of using a machine state that swaps schedulers (see the source project's `State#tick`),
    //   we use filters on systems/sets
    // - rendering is performed in the first stage (of each frame); it does not make a difference from
    //   the user perspective, but it's clear from a design one; this is possible due to the single
    //   scheduler model
    // - the end_turn system is part of the last stage (of each frame); it's not necessary to keep it
    //   separated in an indipendent stage, and it's not worth doing so.

    app.add_system_set(
        SystemSet::new()
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render),
    );

    app.add_system(player_input::player_input.run_in_state(AwaitingInput));

    app.add_system_set_to_stage(
        MovePlayer,
        ConditionSet::new()
            .run_in_state(PlayerTurn)
            .with_system(movement::movement)
            .into(),
    );

    app.add_system_set_to_stage(
        Collisions,
        ConditionSet::new()
            .run_in_state(PlayerTurn)
            .with_system(collisions::collisions)
            .with_system(end_turn::end_turn)
            .into(),
    );

    app.add_system_set_to_stage(
        GenerateMonsterMoves,
        ConditionSet::new()
            .run_in_state(MonsterTurn)
            .with_system(random_move::random_move)
            .into(),
    );

    app.add_system_set_to_stage(
        MoveMonsters,
        ConditionSet::new()
            .run_in_state(MonsterTurn)
            .with_system(movement::movement)
            .with_system(end_turn::end_turn)
            .into(),
    );
}
