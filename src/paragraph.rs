use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Paragraph {
    sentences: Vec<Sentence>,
    current_sentence: usize,
    position: Option<UiRect>,
    width: Option<Val>,
}
impl Paragraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the sentences in the Paragraph to be equal to `sentences`
    /// ```
    /// # use bevy_dialogue_system::prelude::*;
    /// let mut paragraph = Paragraph::new().with_sentences(vec![Sentence::new(), Sentence::new()]);
    ///
    /// assert_eq!(paragraph.sentences().len(), 2);
    /// ```
    pub fn with_sentences(mut self, sentences: Vec<Sentence>) -> Self {
        self.sentences = sentences;
        self
    }
    pub fn push_sentences(mut self, mut sentences: Vec<Sentence>) -> Self {
        self.sentences.append(&mut sentences);
        self
    }
    pub fn push_sentence(mut self, sentence: Sentence) -> Self {
        self.sentences.push(sentence);
        self
    }
    pub fn set_sentences(&mut self, sentences: Vec<Sentence>) {
        self.sentences = sentences;
    }
    pub fn add_sentences(&mut self, mut sentences: Vec<Sentence>) {
        self.sentences.append(&mut sentences);
    }
    pub fn add_sentence(&mut self, sentence: Sentence) {
        self.sentences.push(sentence);
    }
    pub fn sentences(&self) -> &Vec<Sentence> {
        &self.sentences
    }
    pub fn mut_sentences(&mut self) -> &mut Vec<Sentence> {
        &mut self.sentences
    }
    pub fn get_current_sentence(&self) -> Option<&Sentence> {
        self.sentences.get(self.current_sentence)
    }
    pub fn get_current_sentence_mut(&mut self) -> Option<&mut Sentence> {
        self.sentences.get_mut(self.current_sentence)
    }

    pub fn with_position(mut self, position: UiRect) -> Self {
        self.position = Some(position);
        self
    }
    pub fn without_position(mut self) -> Self {
        self.position = None;
        self
    }
    pub fn set_position(&mut self, position: UiRect) {
        self.position = Some(position);
    }
    pub fn reset_position(&mut self) {
        self.position = None;
    }
    pub fn get_position(&self) -> Option<&UiRect> {
        self.position.as_ref()
    }
    pub fn get_position_mut(&mut self) -> Option<&mut UiRect> {
        self.position.as_mut()
    }

    pub fn with_width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }
    pub fn without_width(mut self) -> Self {
        self.width = None;
        self
    }
    pub fn set_width(&mut self, width: Val) {
        self.width = Some(width);
    }
    pub fn reset_width(&mut self) {
        self.width = None;
    }
    pub fn get_width(&self) -> Option<&Val> {
        self.width.as_ref()
    }
    pub fn get_width_mut(&mut self) -> Option<&mut Val> {
        self.width.as_mut()
    }

    pub fn advance_sentence(&mut self) {
        self.current_sentence += 1;
    }

    pub fn update_typewriter(&mut self, amount: f32) {
        if let Some(section) = self.get_current_sentence_mut() {
            section.mut_typewriter().advance(amount);
        }
    }

    pub fn all_paragraphs_visible(&self) -> bool {
        self.current_sentence + 1 == self.sentences.len()
    }
    pub fn all_characters_displayed(&self) -> bool {
        self.get_current_sentence()
            .is_some_and(|section| section.is_typewriter_finished())
    }

    pub fn as_text_sections(&self) -> impl Iterator<Item = TextSection> + '_ {
        self.sentences
            .iter()
            .take(self.current_sentence + 1)
            .map(|section| section.as_text_section())
    }
}
