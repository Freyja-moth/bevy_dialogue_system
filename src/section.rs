use bevy::prelude::*;

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
        dbg!(amount * self.speed);
        self.time = self.time.clamp(0., 1.);
    }

    pub fn change_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn finish(&mut self) {
        self.time = 1.;
    }
}

#[derive(Default, Debug)]
pub struct DialogueSection {
    pub value: String,
    pub action: Option<fn(&mut World)>,
    pub font: Handle<Font>,
    pub color: Color,
    pub size: f32,
    pub typewriter: TypeWriter,
}
impl ToString for DialogueSection {
    fn to_string(&self) -> String {
        if let Some(characters) = self.typewriter_characters() {
            self.value[0..characters].to_string()
        } else {
            self.value.to_string()
        }
    }
}
impl DialogueSection {
    pub fn new(value: impl ToString, font: Handle<Font>) -> Self {
        Self {
            value: value.to_string(),
            action: None,
            font,
            color: Color::GRAY,
            size: 32.,
            ..Default::default()
        }
    }
    pub fn new_without_font(value: impl ToString) -> Self {
        Self {
            value: value.to_string(),
            action: None,
            font: default(),
            color: Color::GRAY,
            size: 32.,
            ..Default::default()
        }
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
    pub fn reset_action(&mut self) {
        self.action = None;
    }
    pub fn action(&self) -> Option<fn(&mut World)> {
        self.action
    }

    pub fn change_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn change_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }
    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn create_typewriter(mut self) -> Self {
        self.typewriter = TypeWriter::new();
        self
    }
    pub fn change_typewriter(mut self, typewriter: TypeWriter) -> Self {
        self.typewriter = typewriter;
        self
    }
    pub fn disable_typewriter(mut self) -> Self {
        self.typewriter.active = false;
        self
    }
    pub fn typewriter_characters(&self) -> Option<usize> {
        if self.typewriter.active {
            Some((self.typewriter.time * self.value.len() as f32) as usize)
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
                font: self.font.clone(),
                font_size: self.size,
                color: self.color,
            },
        }
    }
}
