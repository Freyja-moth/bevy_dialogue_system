use crate::prelude::*;

#[derive(Component)]
pub struct Dialogue {
    paragraphs: VecDeque<Paragraph>,
    skip_keys: Vec<KeyCode>,
    hide_on_empty: bool,
}

impl Default for Dialogue {
    fn default() -> Self {
        Self {
            paragraphs: VecDeque::new(),
            skip_keys: vec![KeyCode::Space, KeyCode::Enter],
            hide_on_empty: true,
        }
    }
}

impl Dialogue {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_paragraphs(mut self, paragraphs: Vec<Paragraph>) -> Self {
        self.paragraphs = paragraphs.into();
        self
    }
    pub fn push_paragraphs(mut self, paragraph: Paragraph) -> Self {
        self.paragraphs.push_back(paragraph);
        self
    }
    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs.into();
    }
    pub fn add_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs.reserve_exact(paragraphs.len());
        paragraphs
            .into_iter()
            .for_each(|paragraph| self.paragraphs.push_back(paragraph));
    }
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push_back(paragraph);
    }
    pub fn get_current_paragraph(&self) -> Option<&Paragraph> {
        self.paragraphs.front()
    }
    pub fn get_current_paragraph_mut(&mut self) -> Option<&mut Paragraph> {
        self.paragraphs.front_mut()
    }
    pub fn advance_paragraph(&mut self) {
        self.paragraphs.pop_front();
    }
    pub fn paragraphs(&self) -> &VecDeque<Paragraph> {
        &self.paragraphs
    }
    pub fn paragraphs_mut(&mut self) -> &mut VecDeque<Paragraph> {
        &mut self.paragraphs
    }
    pub fn len(&self) -> usize {
        self.paragraphs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.paragraphs.is_empty()
    }

    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::prelude::*;
    /// # use itertools::Itertools;
    ///
    /// const NEW_KEYS: [KeyCode; 2] = [KeyCode::A, KeyCode::Space];
    ///
    /// let dialogue = Dialogue::default().with_keys(NEW_KEYS.to_vec());
    ///
    ///
    /// assert_eq!(dialogue.skip_keys().cloned().collect_vec(), NEW_KEYS.to_vec());
    /// ```
    pub fn with_keys(mut self, keys: Vec<KeyCode>) -> Self {
        self.skip_keys = keys;
        self
    }
    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::prelude::*;
    /// # use itertools::Itertools;
    ///
    ///
    /// let dialogue = Dialogue::default().push_key(KeyCode::A);
    ///
    ///
    /// assert_eq!(dialogue.skip_keys().last().cloned(), Some(KeyCode::A));
    /// ```
    pub fn push_key(mut self, key: KeyCode) -> Self {
        self.skip_keys.push(key);
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
    /// let mut dialogue = Dialogue::default();
    ///
    /// dialogue.add_key(KeyCode::B);
    ///
    /// assert_eq!(dialogue.skip_keys().last().cloned(), Some(KeyCode::B));
    ///
    /// ```
    pub fn add_key(&mut self, key: KeyCode) {
        self.skip_keys.push(key);
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

    pub fn with_hide_on_empty(mut self, hide: bool) -> Self {
        self.hide_on_empty = hide;
        self
    }
    pub fn set_hide_on_empty(&mut self, hide: bool) {
        self.hide_on_empty = hide;
    }
    pub fn hide_on_empty(&self) -> bool {
        self.hide_on_empty
    }
}
