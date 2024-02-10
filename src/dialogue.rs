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
            ..Default::default()
        }
    }

    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::prelude::*;
    /// # use itertools::Itertools;
    ///
    /// const NEW_KEYS: [KeyCode; 2] = [KeyCode::A, KeyCode::Space];
    ///
    /// let dialogue = Dialogue::default().change_keys(NEW_KEYS.to_vec());
    ///
    ///
    /// assert_eq!(dialogue.skip_keys().cloned().collect_vec(), NEW_KEYS.to_vec());
    /// ```
    pub fn change_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.skip_keys = keys;
        self
    }

    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::prelude::*;
    /// # use itertools::Itertools;
    ///
    /// const NEW_KEYS: [KeyCode; 2] = [KeyCode::A, KeyCode::Space];
    ///
    /// let mut dialogue = Dialogue::default();
    ///
    /// dialogue.set_keys(NEW_KEYS.to_vec());
    ///
    /// assert_eq!(dialogue.skip_keys().cloned().collect_vec(), NEW_KEYS.to_vec());
    /// ```
    pub fn set_keys(&mut self, keys: Vec<KeyCode>) {
        self.skip_keys = keys;
    }

    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::prelude::*;
    /// # use itertools::Itertools;
    ///
    /// let dialogue = Dialogue::default();
    ///
    /// assert_eq!(dialogue.skip_keys().cloned().collect_vec(), vec![KeyCode::Space, KeyCode::Return]);
    /// ```
    pub fn skip_keys(&self) -> impl Iterator<Item = &KeyCode> {
        self.skip_keys.iter()
    }
    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::prelude::*;
    /// # use itertools::Itertools;
    ///
    /// let mut dialogue = Dialogue::default();
    ///
    /// {
    ///     let mut keys = dialogue.mut_skip_keys();
    ///
    ///     if let Some(first) = keys.next() {
    ///         *first = KeyCode::Z;
    ///     }
    /// }
    ///
    /// assert_eq!(dialogue.skip_keys().cloned().collect_vec(), vec![KeyCode::Z, KeyCode::Return]);
    /// ```
    pub fn mut_skip_keys(&mut self) -> impl Iterator<Item = &mut KeyCode> {
        self.skip_keys.iter_mut()
    }
}
