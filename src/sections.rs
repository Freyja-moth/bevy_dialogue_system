use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Paragrah {
    sentances: Vec<Sentance>,
    current_sentance: usize,
    position: Option<UiRect>,
    width: Option<Val>,
}
impl Paragrah {
    pub fn new(sections: Vec<Sentance>) -> Self {
        Self {
            sentances: sections,
            ..Default::default()
        }
    }

    pub fn add_sentance(&mut self, section: Sentance) {
        self.sentances.push(section);
    }
    pub fn add_sentances(&mut self, mut section: Vec<Sentance>) {
        self.sentances.reserve_exact(section.len());
        self.sentances.append(&mut section);
    }
    pub fn get_current_sentance(&self) -> Option<&Sentance> {
        self.sentances.get(self.current_sentance)
    }
    pub fn get_current_sentance_mut(&mut self) -> Option<&mut Sentance> {
        self.sentances.get_mut(self.current_sentance)
    }

    pub fn advance_sentance(&mut self) {
        self.current_sentance += 1;
    }
    pub fn current_sentance(&self) -> usize {
        self.current_sentance
    }
    pub fn sentances(&self) -> usize {
        self.sentances.len()
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

    pub fn update_typwriter(&mut self, amount: f32) {
        if let Some(section) = self.get_current_sentance_mut() {
            section.mut_typewriter().advance(amount);
        }
    }

    pub fn all_paragraphs_visible(&self) -> bool {
        self.current_sentance + 1 == self.sentances.len()
    }
    pub fn all_characters_displayed(&self) -> bool {
        self.get_current_sentance()
            .is_some_and(|section| section.is_typwriter_finished())
    }

    pub fn as_text_sections(&self) -> impl Iterator<Item = TextSection> + '_ {
        self.sentances
            .iter()
            .take(self.current_sentance + 1)
            .map(|section| section.as_text_section())
    }
}
