use crate::prelude::*;

#[derive(Debug)]
pub struct TypeWriter {
    active: bool,
    time: f32,
    speed: f32,
}
impl Default for TypeWriter {
    fn default() -> Self {
        Self {
            active: false,
            time: 0.,
            speed: 0.5,
        }
    }
}
impl TypeWriter {
    pub fn new() -> Self {
        Self {
            active: true,
            ..Default::default()
        }
    }
    pub fn advance(&mut self, amount: f32) {
        self.time += amount * self.speed;

        self.time = self.time.clamp(0., 1.);
    }

    pub fn activate(&mut self) {
        self.active = true;
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn finish(&mut self) {
        self.time = 1.;
    }
    pub fn reset(&mut self) {
        self.time = 0.;
    }
    pub fn current_time(&self) -> f32 {
        self.time
    }

    pub fn change_speed(mut self, speed: f32) -> Self {
        self.speed = speed.clamp(0., 1.);
        self
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0., 1.);
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
}

#[derive(Default, Debug)]
pub struct Sentance {
    text_section: TextSection,
    action: Option<fn(&mut World)>,
    typewriter: TypeWriter,
}
impl ToString for Sentance {
    fn to_string(&self) -> String {
        if let Some(characters) = self.typewriter_characters() {
            self.text_section.value[0..characters].to_string()
        } else {
            self.text_section.value.to_string()
        }
    }
}
impl Sentance {
    pub fn new(value: impl ToString) -> Self {
        Self {
            text_section: TextSection {
                value: value.to_string(),
                style: TextStyle {
                    font_size: 32.,
                    ..Default::default()
                },
            },
            action: None,
            ..Default::default()
        }
    }

    pub fn change_text(mut self, value: impl ToString) -> Self {
        self.text_section.value = value.to_string();
        self
    }
    pub fn set_text(&mut self, value: impl ToString) {
        self.text_section.value = value.to_string();
    }
    pub fn text(&self) -> &str {
        &self.text_section.value
    }
    pub fn mut_text(&mut self) -> &mut str {
        &mut self.text_section.value
    }

    pub fn change_font(mut self, font: Handle<Font>) -> Self {
        self.text_section.style.font = font;
        self
    }
    pub fn set_font(&mut self, font: Handle<Font>) {
        self.text_section.style.font = font;
    }
    pub fn font(&self) -> &Handle<Font> {
        &self.text_section.style.font
    }
    pub fn mut_font(&mut self) -> &mut Handle<Font> {
        &mut self.text_section.style.font
    }

    pub fn change_font_size(mut self, size: f32) -> Self {
        self.text_section.style.font_size = size;
        self
    }
    pub fn set_font_size(&mut self, size: f32) {
        self.text_section.style.font_size = size;
    }
    pub fn font_size(&self) -> &f32 {
        &self.text_section.style.font_size
    }
    pub fn mut_font_size(&mut self) -> &mut f32 {
        &mut self.text_section.style.font_size
    }

    pub fn change_color(mut self, color: Color) -> Self {
        self.text_section.style.color = color;
        self
    }
    pub fn set_color(&mut self, color: Color) {
        self.text_section.style.color = color;
    }
    pub fn color(&self) -> &Color {
        &self.text_section.style.color
    }
    pub fn mut_color(&mut self) -> &mut Color {
        &mut self.text_section.style.color
    }

    pub fn change_action(mut self, action: fn(&mut World)) -> Self {
        self.action = Some(action);
        self
    }
    pub fn remove_action(mut self) -> Self {
        self.action = None;
        self
    }
    pub fn set_action(&mut self, action: fn(&mut World)) {
        self.action = Some(action);
    }
    pub fn get_action(&self) -> Option<&fn(&mut World)> {
        self.action.as_ref()
    }
    pub fn get_action_mut(&mut self) -> Option<&mut fn(&mut World)> {
        self.action.as_mut()
    }

    pub fn create_typewriter(mut self) -> Self {
        self.typewriter = TypeWriter::new();
        self
    }
    pub fn change_typewriter(mut self, typewriter: TypeWriter) -> Self {
        self.typewriter = typewriter;
        self
    }
    pub fn set_typewriter(&mut self, typewriter: TypeWriter) {
        self.typewriter = typewriter;
    }
    pub fn typewriter(&self) -> &TypeWriter {
        &self.typewriter
    }
    pub fn mut_typewriter(&mut self) -> &mut TypeWriter {
        &mut self.typewriter
    }

    pub fn typewriter_characters(&self) -> Option<usize> {
        if self.typewriter.active {
            Some((self.typewriter.time * self.text_section.value.len() as f32) as usize)
        } else {
            None
        }
    }
    pub fn is_typwriter_finished(&self) -> bool {
        !self.typewriter.active || self.typewriter.time == 1.
    }

    pub fn as_text_section(&self) -> TextSection {
        TextSection {
            value: self.to_string(),
            style: TextStyle {
                font: self.font().clone(),
                ..self.text_section.style
            },
        }
    }
}
