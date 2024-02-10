use crate::prelude::*;

#[derive(Component)]
pub struct Dialogue {
    chapters: VecDeque<Paragrah>,
    skip_keys: Vec<KeyCode>,
}

impl Default for Dialogue {
    fn default() -> Self {
        Self {
            chapters: vec![].into(),
            skip_keys: vec![KeyCode::Space, KeyCode::Return],
        }
    }
}

impl Deref for Dialogue {
    type Target = VecDeque<Paragrah>;

    fn deref(&self) -> &Self::Target {
        &self.chapters
    }
}
impl DerefMut for Dialogue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chapters
    }
}

impl Dialogue {
    pub fn new(sections: Vec<Paragrah>) -> Self {
        Self {
            chapters: sections.into(),
            skip_keys: vec![KeyCode::Space, KeyCode::Return],
        }
    }

    pub fn change_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.skip_keys = keys;
        self
    }
    pub fn set_keys(&mut self, keys: Vec<KeyCode>) {
        self.skip_keys = keys;
    }
    pub fn skip_keys(&self) -> impl Iterator<Item = &KeyCode> {
        self.skip_keys.iter()
    }
    pub fn mut_skip_keys(&mut self) -> impl Iterator<Item = &mut KeyCode> {
        self.skip_keys.iter_mut()
    }
}
