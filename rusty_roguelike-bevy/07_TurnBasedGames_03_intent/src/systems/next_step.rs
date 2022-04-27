use crate::prelude::*;

// This is end_turn() in the source project; here, it's next_step(), because the semantics are different.
// Made Really Cool™ by using const generics.
//
pub fn next_step<const S: GameStep>(mut commands: Commands) {
    commands.insert_resource(NextState(S));
}
