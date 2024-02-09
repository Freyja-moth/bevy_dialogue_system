use crate::dialogue::Dialogue;
use bevy::{
    ecs::system::RunSystemOnce,
    prelude::{
        App, Input, IntoSystemConfigs, KeyCode, Plugin, Query, Res, ResMut, Resource, Update,
        Visibility, World,
    },
    text::Text,
    time::Time,
    ui::Style,
};

#[derive(Resource, Default)]
pub(crate) struct CurrentAction(Option<fn(&mut World)>);

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentAction>().add_systems(
            Update,
            (
                update_dialogue.before(run_action),
                run_action,
                show_dialogue,
                update_typewriter,
                move_dialogue,
                change_width,
            ),
        );
    }
}

fn update_dialogue(
    mut dialogue: Query<&mut Dialogue>,
    mut current_action: ResMut<CurrentAction>,
    input: Res<Input<KeyCode>>,
) {
    dialogue
        .iter_mut()
        .filter(|dialogue| input.any_just_pressed(dialogue.skip_keys().iter().cloned()))
        .filter(|dialogue| dialogue.front().is_some())
        .for_each(|mut dialogue| {
            let front = dialogue.front_mut().unwrap();
            let all_sections = front.all_sections_visible();
            let all_characters = front.all_characters_displayed();

            current_action.0 = front
                .get_current_section()
                .and_then(|section| section.action());

            if all_sections && all_characters {
                dialogue.pop_front();
            } else if all_characters {
                front.advance_section();
            } else if let Some(section) = front.get_current_section_mut() {
                section.typewriter.finish();
            }
        });
}

fn run_action(world: &mut World) {
    if let Some(action) = world.resource_mut::<CurrentAction>().0 {
        println!("HERE");
        world.run_system_once(action);
    }
    world.resource_mut::<CurrentAction>().0 = None;
}

fn update_typewriter(mut dialogue: Query<&mut Dialogue>, time: Res<Time>) {
    dialogue.iter_mut().for_each(|mut dialogue| {
        if let Some(typewriter) = dialogue.front_mut() {
            typewriter.update_typwriter(time.delta_seconds());
        }
    });
}

fn show_dialogue(mut dialogue_area: Query<(&mut Text, &mut Visibility, &Dialogue)>) {
    dialogue_area
        .iter_mut()
        .for_each(|(mut text, mut visibility, dialogue)| {
            if let Some(section) = dialogue.front() {
                *visibility = Visibility::Inherited;

                text.sections = section.as_text_sections().collect();
            } else {
                *visibility = Visibility::Hidden;
            }
        });
}

fn move_dialogue(mut dialogue_area: Query<(&mut Style, &Dialogue)>) {
    dialogue_area.iter_mut().for_each(|(mut style, dialogue)| {
        if let Some(position) = dialogue.front().and_then(|section| section.position()) {
            style.top = position.top;
            style.bottom = position.bottom;
            style.left = position.left;
            style.right = position.right;
        }
    });
}

fn change_width(mut dialogue_area: Query<(&mut Style, &Dialogue)>) {
    dialogue_area.iter_mut().for_each(|(mut style, dialogue)| {
        if let Some(width) = dialogue.front().and_then(|section| section.width()) {
            style.width = width;
        }
    });
}
