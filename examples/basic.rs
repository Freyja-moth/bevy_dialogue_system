use bevy::{app::AppExit, prelude::*};
use bevy_dialogue_system::prelude::*;

const KEYS: [KeyCode; 3] = [KeyCode::Space, KeyCode::Enter, KeyCode::KeyA];

#[derive(Component)]
pub struct MainDialogue;

#[derive(Component)]
pub struct SecondaryDialogue;

fn main() {
    App::new()
        .add_plugins((DialoguePlugin, DefaultPlugins))
        .init_resource::<ClearColor>()
        .add_systems(Startup, setup)
        .add_systems(Update, exit)
        .run();
}

fn enable_second_dialogue(world: &mut World) {
    let mut dialogue_query = world.query_filtered::<&mut Dialogue, With<SecondaryDialogue>>();

    dialogue_query.single_mut(world).add_paragraph(
        Paragraph::new()
            .with_sentences(vec![Sentence::new().with_text("multiple running at once!")])
            .with_width(Val::Percent(50.)),
    );
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
        .insert(Dialogue::new().with_paragraphs(
            vec![
                Paragraph::new().with_sentences(vec![Sentence::new().with_text(
                    "Press Enter, Space or A to advance",
                )]),
                Paragraph::new().with_sentences(vec![
                    Sentence::new().with_text("This is a very basic story,\n"),
                    Sentence::new().with_text("that uses colors,\n")
                        .with_color(Color::RED),
                    Sentence::new().with_text("different text sizes,\n").with_font_size(64.),
                    Sentence::new().with_text(
                        "different fonts for accessibilitys sake,\n",
                    ).with_font(dyslexic_font),
                    Sentence::new().with_text("and typewriter text... "),
                    Sentence::new().with_text(
                        "that you can skip if you're really impatient.",
                    )
                    .create_typewriter(),
                ]),
                Paragraph::new().with_sentences(vec![Sentence::new().with_text(
                    "It can move itself around.",
                )])
                .with_position(UiRect::left(Val::Px(300.))),
                Paragraph::new().with_sentences(vec![Sentence::new().with_text(
                    "And can squish itself at will.",
                ).with_action(enable_second_dialogue)])
                .with_position(UiRect::default())
                .with_width(Val::Percent(25.)),
                Paragraph::new().with_sentences(vec![Sentence::new().with_text(
                    "You can even have ",
                )])
                .with_width(Val::Percent(50.)),
                Paragraph::new().with_sentences(vec![
                    Sentence::new().with_text(
                        "Oh and you can affect the world directly...",
                    )
                    .with_action(change_background),
                    Sentence::new().with_text("like so!"),
                ]),
                Paragraph::new().with_sentences(vec![Sentence::new().with_text(
                    "And it was only after I made this that I realised the yarnspinner plugin was released not ten days ago...",
                ).with_typewriter(TypeWriter::new().with_speed(0.7))])
                .with_width(Val::Percent(100.)),
            ],
        ).with_keys(KEYS.to_vec()))
        .insert(MainDialogue);

    commands
        .spawn(TextBundle {
            style: Style {
                justify_self: JustifySelf::End,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Dialogue::default().with_keys(KEYS.to_vec()))
        .insert(SecondaryDialogue);
}

fn exit(mut app_exit: EventWriter<AppExit>, dialogue: Query<&Dialogue>) {
    if dialogue.iter().all(|dialogue| dialogue.is_empty()) {
        app_exit.send_default();
    }
}
