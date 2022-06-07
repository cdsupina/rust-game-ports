use std::any::Any;

use crate::prelude::*;

pub trait Targetable: MyActor + Any {
    fn as_any(&self) -> &dyn Any;
    fn active(&self, ball: &Ball) -> bool;
}
