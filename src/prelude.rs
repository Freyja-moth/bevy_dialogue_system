pub use crate::{
    dialogue::Dialogue,
    paragraph::Paragraph,
    plugin::{CurrentAction, DialoguePlugin},
    sentence::{Sentence, TypeWriter},
};
pub(crate) use bevy::{ecs::system::RunSystemOnce, prelude::*};
pub(crate) use std::{collections::VecDeque, fmt::Display};
