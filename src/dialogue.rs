use crate::sections::DialogueSections;
use bevy::prelude::{Component, KeyCode};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

#[derive(Component)]
pub struct Dialogue {
    sections: VecDeque<DialogueSections>,
    skip_keys: Box<[KeyCode]>,
}

impl Default for Dialogue {
    fn default() -> Self {
        Self {
            sections: vec![].into(),
            skip_keys: Box::new([KeyCode::Space, KeyCode::Return]),
        }
    }
}

impl Deref for Dialogue {
    type Target = VecDeque<DialogueSections>;

    fn deref(&self) -> &Self::Target {
        &self.sections
    }
}
impl DerefMut for Dialogue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sections
    }
}

impl Dialogue {
    pub fn new(sections: Vec<DialogueSections>, skip_keys: Box<[KeyCode]>) -> Self {
        Self {
            sections: sections.into(),
            skip_keys,
        }
    }
    pub fn empty(skip_keys: Box<[KeyCode]>) -> Self {
        Self {
            skip_keys,
            ..Default::default()
        }
    }

    pub fn skip_keys(&self) -> &[KeyCode] {
        &self.skip_keys
    }
}
