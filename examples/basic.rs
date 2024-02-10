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
        .add_systems(Update, exit)
        .run();
}

fn enable_second_dialogue(world: &mut World) {
    let mut dialogue_query = world.query_filtered::<&mut Dialogue, With<SecondaryDialogue>>();

    dialogue_query.single_mut(world).push_back(
        Paragrah::new(vec![Sentance::new("multiple running at once!")])
            .change_width(Val::Percent(50.)),
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
        .insert(Dialogue::new(
            vec![
                Paragrah::new(vec![Sentance::new(
                    "Press Enter, Space or A to advance",
                )]),
                Paragrah::new(vec![
                    Sentance::new("This is a very basic story,\n"),
                    Sentance::new("that uses colors,\n")
                        .change_color(Color::RED),
                    Sentance::new("different text sizes,\n").change_font_size(64.),
                    Sentance::new(
                        "different fonts for accessibilitys sake,\n",
                    ).change_font(dyslexic_font),
                    Sentance::new("and typewriter text... "),
                    Sentance::new(
                        "that you can skip if you're really impatient.",
                    )
                    .create_typewriter(),
                ]),
                Paragrah::new(vec![Sentance::new(
                    "It can move itself around.",
                )])
                .change_position(UiRect::left(Val::Px(300.))),
                Paragrah::new(vec![Sentance::new(
                    "And can squish itself at will.",
                ).change_action(enable_second_dialogue)])
                .change_position(UiRect::default())
                .change_width(Val::Percent(25.)),
                Paragrah::new(vec![Sentance::new(
                    "You can even have ",
                )])
                .change_width(Val::Percent(50.)),
                Paragrah::new(vec![
                    Sentance::new(
                        "Oh and you can affect the world directly...",
                    )
                    .change_action(change_background),
                    Sentance::new("like so!"),
                ]),
                Paragrah::new(vec![Sentance::new(
                    "And it was only after I made this that I realised the yarnspinner plugin was released not ten days ago...",
                ).change_typewriter(TypeWriter::new().change_speed(0.7))])
                .change_width(Val::Percent(100.)),
            ],
        ).change_keys(KEYS.to_vec()))
        .insert(MainDialogue);

    commands
        .spawn(TextBundle {
            style: Style {
                justify_self: JustifySelf::End,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Dialogue::default().change_keys(KEYS.to_vec()))
        .insert(SecondaryDialogue);
}

fn exit(mut app_exit: EventWriter<AppExit>, dialogue: Query<&Dialogue>) {
    if dialogue.iter().all(|dialogue| dialogue.is_empty()) {
        app_exit.send_default();
    }
}
