use bevy::{app::AppExit, prelude::*};
use bevy_dialogue_system::prelude::*;

const KEYS: [KeyCode; 3] = [KeyCode::Space, KeyCode::Return, KeyCode::A];

#[derive(Component)]
pub struct MainDialogue;

#[derive(Component)]
pub struct SecondaryDialogue;

fn main() {
    App::new()
        .add_plugins((DialoguePlugin, DefaultPlugins))
        .init_resource::<ClearColor>()
        .add_systems(Startup, setup)
        .add_systems(Update, (exit, show_multiple.run_if(should_show_multiple)))
        .run();
}

fn change_background(world: &mut World) {
    let mut clear_color = world.resource_mut::<ClearColor>();

    clear_color.0 = Color::rgb(87. / 255., 34. / 255., 104. / 255.);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let dyslexic_font = asset_server.load("open_dyslexia_regular.otf");

    commands
        .spawn(TextBundle::default())
        .insert(Dialogue::new(
            vec![
                DialogueSections::new(vec![DialogueSection::new_without_font(
                    "Press Enter, Space or A to advance",
                )]),
                DialogueSections::new(vec![
                    DialogueSection::new_without_font("This is a very basic story,\n"),
                    DialogueSection::new_without_font("that uses colors,\n")
                        .change_color(Color::RED),
                    DialogueSection::new_without_font("different text sizes,\n").change_size(64.),
                    DialogueSection::new(
                        "different fonts for accessibilitys sake,\n",
                        dyslexic_font,
                    ),
                    DialogueSection::new_without_font("and typewriter text... "),
                    DialogueSection::new_without_font(
                        "that you can skip if you're really impatient.",
                    )
                    .create_typewriter(),
                ]),
                DialogueSections::new(vec![DialogueSection::new_without_font(
                    "It can move itself around.",
                )])
                .change_position(UiRect::left(Val::Px(300.))),
                DialogueSections::new(vec![DialogueSection::new_without_font(
                    "And can squish itself at will.",
                )])
                .change_position(UiRect::default())
                .change_width(Val::Percent(25.)),
                DialogueSections::new(vec![DialogueSection::new_without_font(
                    "You can even have ",
                )])
                .change_width(Val::Percent(50.)),
                DialogueSections::new(vec![
                    DialogueSection::new_without_font(
                        "Oh and you can affect the world directly...",
                    )
                    .change_action(change_background),
                    DialogueSection::new_without_font("like so!"),
                ]),
                DialogueSections::new(vec![DialogueSection::new_without_font(
                    "And it was only after I made this that I realised the yarnspinner plugin was released not ten days ago...",
                ).change_typewriter(TypeWriter::new().change_speed(0.7))])
                .change_width(Val::Percent(100.)),
            ],
            Box::new(KEYS),
        ))
        .insert(MainDialogue);

    commands
        .spawn(TextBundle {
            style: Style {
                justify_self: JustifySelf::End,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Dialogue::empty(Box::new(KEYS)))
        .insert(SecondaryDialogue);
}

fn should_show_multiple(dialogue: Query<&Dialogue, With<MainDialogue>>) -> bool {
    dialogue
        .single()
        .front()
        .is_some_and(|front| front.width().is_some_and(|val| val == Val::Percent(50.)))
}

fn show_multiple(
    mut secondary: Query<&mut Dialogue, With<SecondaryDialogue>>,
    mut shown: Local<bool>,
) {
    if !*shown {
        let mut secondary = secondary.single_mut();

        secondary.push_back(
            DialogueSections::new(vec![DialogueSection::new_without_font(
                "multiple running at once!",
            )])
            .change_width(Val::Percent(50.)),
        );

        *shown = true;
    }
}

fn exit(mut app_exit: EventWriter<AppExit>, dialogue: Query<&Dialogue>) {
    if dialogue.iter().all(|dialogue| dialogue.is_empty()) {
        app_exit.send_default();
    }
}
