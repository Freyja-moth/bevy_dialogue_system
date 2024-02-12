use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Paragraph {
    sentences: Vec<Sentence>,
    current_sentence: usize,
    position: Option<UiRect>,
    width: Option<Val>,
}
impl Paragraph {
    pub fn new(sections: Vec<Sentence>) -> Self {
        Self {
            sentences: sections,
            ..Default::default()
        }
    }

    pub fn add_sentence(&mut self, section: Sentence) {
        self.sentences.push(section);
    }
    pub fn add_sentences(&mut self, mut section: Vec<Sentence>) {
        self.sentences.reserve_exact(section.len());
        self.sentences.append(&mut section);
    }
    pub fn get_current_sentence(&self) -> Option<&Sentence> {
        self.sentences.get(self.current_sentence)
    }
    pub fn get_current_sentence_mut(&mut self) -> Option<&mut Sentence> {
        self.sentences.get_mut(self.current_sentence)
    }

    pub fn advance_sentence(&mut self) {
        self.current_sentence += 1;
    }
    pub fn current_sentence(&self) -> usize {
        self.current_sentence
    }
    pub fn sentences(&self) -> usize {
        self.sentences.len()
    }

    pub fn change_position(mut self, position: UiRect) -> Self {
        self.position = Some(position);
        self
    }
    pub fn remove_position(mut self) -> Self {
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

    pub fn change_width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }
    pub fn remove_width(mut self) -> Self {
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
