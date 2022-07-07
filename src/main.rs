use bevy::app::{App, Plugin};
use bevy::core::{Time, Timer};
use bevy::core_pipeline::ClearColor;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::ecs::query::With;
use bevy::hierarchy::Children;
use bevy::math::Size;
use bevy::prelude::{
    AssetServer,
    BuildChildren,
    Button,
    ButtonBundle,
    Changed,
    Rect,
    TextBundle,
    UiCameraBundle,
};
use bevy::render::color::Color;
use bevy::text::{Text, TextStyle};
use bevy::ui::{AlignItems, Interaction, JustifyContent, Style, UiColor, Val};
use bevy::utils::default;
use bevy::window::exit_on_window_close_system;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use rand::thread_rng;
use rand::prelude::SliceRandom;

// plugins
/*
use bevy::animation::AnimationPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy::asset::AssetPlugin;
use bevy::core::CorePlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin,
use bevy::gltf::GltfPlugin;
use bevy::gilrs::GilrsPlugin;
use bevy::hierarchy::HierarchyPlugin;
use bevy::log::LogPlugin;
use bevy::input::InputPlugin;
use bevy::pbr::PbrPlugin;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::transform::TransformPlugin;
use bevy::ui::UiPlugin;
use bevy::window::WindowPlugin;
use bevy::winit::WinitPlugin;
*/
use bevy::prelude::DefaultPlugins;
// disabled
use bevy::animation::AnimationPlugin;
use bevy::audio::AudioPlugin;
use bevy::pbr::PbrPlugin;
use bevy::gltf::GltfPlugin;
use bevy::gilrs::GilrsPlugin;
// enabled
use bevy::diagnostic::{
    FrameTimeDiagnosticsPlugin,
    LogDiagnosticsPlugin
};

const BUTTON_NORMAL: Color = Color::rgb(0.15, 0.15, 0.15);
const BUTTON_HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);
const BUTTON_PRESSED: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Inspectable)]
struct Dog;

#[derive(Component, Inspectable)]
struct Voice(String);

#[derive(Inspectable, Default)]
struct Data {
  should_render: bool,
  text: String,
  #[inspectable(min = 42.0, max = 100.0)]
  size: f32,
}

struct GameTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn().insert(Dog).insert(Voice("Bark".to_string()));
    commands.spawn().insert(Dog).insert(Voice("Woof".to_string()));
    commands.spawn().insert(Dog).insert(Voice("arf".to_string()));

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: BUTTON_NORMAL.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Dog",
                    TextStyle {
                        font: asset_server.load("font/FiraMono-Medium.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}

fn timer_system(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    voice_query: Query<&Voice, With<Dog>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        println!("{}!", get_random(&voice_query));
    }
}

fn get_random(query: &Query<&Voice, With<Dog>>) -> String {
    let mut rng = thread_rng();
    let mut values: Vec<&Voice> = query.iter().collect();
    values.shuffle(&mut rng);

    format!("{}!", values[0].0)
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    voice_query: Query<&Voice, With<Dog>>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        let current = &text.sections[0].value;
        let mut bark = get_random(&voice_query);
        while current == &bark {
            bark = get_random(&voice_query);
        }

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = format!("{}!!", bark);
                *color = BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = bark;
                *color = BUTTON_HOVERED.into();
            }
            Interaction::None => {
                text.sections[0].value = bark;
                *color = BUTTON_NORMAL.into();
            }
        }
    }
}

pub struct BarrkPlugin;

impl Plugin for BarrkPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WinitSettings::desktop_app())
            .insert_resource(ClearColor(Color::rgb(0.11, 0.11, 0.11)))
            .insert_resource(GameTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup)
            .add_system(timer_system)
            .add_system(button_system)
            .add_system(exit_on_window_close_system);
    }
}

fn main() {
    App::new()
        /*
        .add_plugin(LogPlugin::default())
        .add_plugin(CorePlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(HierarchyPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_plugin(AssetPlugin::default())
        .add_plugin(ScenePlugin::default())
        .add_plugin(WinitPlugin::default())
        .add_plugin(RenderPlugin::default())
        .add_plugin(CorePipelinePlugin::default())
        .add_plugin(SpritePlugin::default())
        .add_plugin(TextPlugin::default())
        .add_plugin(UiPlugin::default())
        .add_plugin(PbrPlugin::default())
        .add_plugin(GltfPlugin::default())
        .add_plugin(GilrsPlugin::default())
        .add_plugin(AnimationPlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        */
        .add_plugins_with(DefaultPlugins, |g| {
            // NOTE: disabled plugin will still be bundled into the final build
            g.disable::<AudioPlugin>();
            g.disable::<PbrPlugin>();
            g.disable::<GltfPlugin>();
            g.disable::<GilrsPlugin>();
            g.disable::<AnimationPlugin>();
            g
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Dog>()
        .register_inspectable::<Voice>()
        .register_inspectable::<Data>()
        .add_plugin(BarrkPlugin)
        .run();
}
