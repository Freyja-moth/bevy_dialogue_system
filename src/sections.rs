use crate::section::DialogueSection;
use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct DialogueSections {
    sections: Vec<DialogueSection>,
    current_section: usize,
    position: Option<UiRect>,
    width: Option<Val>,
}
impl DialogueSections {
    pub fn new(sections: Vec<DialogueSection>) -> Self {
        Self {
            sections,
            ..Default::default()
        }
    }

    /// Changes the position
    /// ///
    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::ui::{Val, UiRect};
    ///
    /// let mut sections = DialogueSections::default().change_position(UiRect::all(Val::Percent(30.)));
    ///
    /// assert_eq!(sections.position().unwrap(), UiRect::all(Val::Percent(30.)));
    /// ```
    pub fn change_position(mut self, position: UiRect) -> Self {
        self.position = Some(position);
        self
    }
    pub fn remove_position(mut self) -> Self {
        self.position = None;
        self
    }
    /// Changes the position without moving ownership
    ///
    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::ui::{Val, UiRect};
    ///
    /// let mut sections = DialogueSections::default();
    ///
    /// sections.set_position(UiRect::left(Val::Percent(20.)));
    ///
    /// assert_eq!(sections.position().unwrap(), UiRect::left(Val::Percent(20.)));
    /// ```
    pub fn set_position(&mut self, position: UiRect) {
        self.position = Some(position);
    }
    pub fn reset_position(&mut self) {
        self.position = None;
    }
    ///
    ///
    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::ui::Val;
    ///
    /// let sections = DialogueSections::default();
    ///
    /// assert!(sections.position().is_none());
    /// ```
    pub fn position(&self) -> Option<UiRect> {
        self.position
    }

    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::ui::Val;
    ///
    /// let mut sections = DialogueSections::default();
    ///
    /// let new_sections = sections.change_width(Val::Px(200.));
    ///
    /// assert_eq!(new_sections.width().unwrap(), Val::Px(200.));
    /// ```
    pub fn change_width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }
    pub fn remove_width(mut self) -> Self {
        self.width = None;
        self
    }
    /// ```rust
    /// # use bevy_dialogue_system::prelude::*;
    /// # use bevy::ui::Val;
    ///
    /// let mut sections = DialogueSections::default();
    ///
    /// sections.set_width(Val::Px(200.));
    ///
    /// assert_eq!(sections.width().unwrap(), Val::Px(200.));
    /// ```
    pub fn set_width(&mut self, width: Val) {
        self.width = Some(width);
    }
    pub fn reset_width(&mut self) {
        self.width = None;
    }

    pub fn width(&self) -> Option<Val> {
        self.width
    }

    pub fn advance_section(&mut self) {
        self.current_section += 1;
    }
    pub fn current_section(&self) -> usize {
        self.current_section
    }
    pub fn get_current_section(&self) -> Option<&DialogueSection> {
        self.sections.get(self.current_section)
    }
    pub fn get_current_section_mut(&mut self) -> Option<&mut DialogueSection> {
        self.sections.get_mut(self.current_section)
    }

    pub fn update_typwriter(&mut self, amount: f32) {
        if let Some(section) = self.get_current_section_mut() {
            section.typewriter.advance(amount);
        }
    }

    pub fn all_sections_visible(&self) -> bool {
        self.current_section + 1 == self.sections.len()
    }
    pub fn all_characters_displayed(&self) -> bool {
        self.get_current_section()
            .is_some_and(|section| section.is_typwriter_finished())
    }

    pub fn as_text_sections(&self) -> impl Iterator<Item = TextSection> + '_ {
        self.sections
            .iter()
            .take(self.current_section + 1)
            .map(|section| section.as_text_section())
    }
}
