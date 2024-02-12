#[cfg(feature = "doc_testing")]
pub use crate::plugin::CurrentAction;
pub use crate::{
    dialogue::Dialogue,
    plugin::DialoguePlugin,
    section::{Sentence, TypeWriter},
    sections::Paragraph,
};
pub(crate) use bevy::{ecs::system::RunSystemOnce, prelude::*};
pub(crate) use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};
