use std::f64::consts::PI;
use std::ops::{Deref, DerefMut};
use bevy::{color::palettes::basic::PURPLE, prelude::*, sprite::MaterialMesh2dBundle};
use bevy::color::palettes::basic::BLACK;
use bevy::color::palettes::css::DARK_CYAN;
use bevy::ecs::system::{RunSystemOnce, SystemState};
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, PowerPreference, RenderCreation, WgpuSettings};
use bevy::utils::info;
use bevy::window::{PrimaryWindow, WindowCloseRequested};
use crate::app2d::active_info::ActiveInfo;
use crate::app2d::schema::{App2DOptions};
use crate::content::component::ExampleShape;
use crate::content::control::pancam::{PanCam, PanCamPlugin};
use crate::content::shape::prelude::*;
use crate::content::sync::config::AppConfig;
use crate::content::sync::plugin::SyncPlugin;

pub struct Application2D {
    pub app: App,
    // pub window: Entity,
}

impl Application2D {
    // Creating app
    pub fn new(options: App2DOptions) -> Application2D {
        let canvas_id = options.canvas_id.clone();
        let mut app = App::new();
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // provide the ID selector string here
                        canvas: Some(canvas_id),
                        // ... any other window properties ...
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        // backends: Some(Backends::BROWSER_WEBGPU),
                        power_preference: PowerPreference::HighPerformance,
                        ..default()
                    }),
                    ..default()
                }),
            PanCamPlugin::default(),
            SyncPlugin::default(),
            ShapePlugin,
        ));
        app.insert_resource(ActiveInfo::new());
        app.add_systems(Startup, setup);
        app.add_systems(Update, setup2);
        // app.add_systems(Update, ime_toggle);
        Self {
            app,
            // window: Entity::PLACEHOLDER,
        }
    }
}

impl Deref for Application2D {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for Application2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle::default(), PanCam::default()));
    // commands.spawn(Camera2dBundle::default());
    // let count = 100;
    // for x in 0..count {
    //     for y in 0..count {
    //         commands.spawn(MaterialMesh2dBundle {
    //             mesh: meshes.add(Rectangle::default()).into(),
    //             transform: Transform::default().with_scale(Vec3::splat(1.)).with_translation(Vec3::new(x as f32, y as f32, 0.)),
    //             material: materials.add(Color::from(PURPLE)),
    //             ..default()
    //         });
    //     }
    // }
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::from(PURPLE)),
        ..default()
    });

    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(DARK_CYAN),
        Stroke::new(BLACK, 10.0),
        ExampleShape(1),
    ));

    let svg_doc_size = Vec2::new(0., 0.);
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                svg_path_string: BLACKSMITH_OUTLINE.to_owned(),
                svg_doc_size_in_px: svg_doc_size.to_owned(),
            }),
            ..default()
        },
        Stroke::new(Color::BLACK, 4.0),
        Fill::color(Color::WHITE),
    ));
    // root(c_root, &assets, &mut commands, |p| {
    //     node((c_half, c_green), p, |p| {
    //         // text("This is the left pane!", c_text, c_pixel, p);
    //         // text("Do you like it?", c_text, c_pixel, p);
    //         // text_buttoni("Hiya", c_button_left, c_pixel, UiId::HiyaButton, p);      // Inline variant of text_button
    //         // grid(6, 6, c_grid, p, |p, _row, _col| {
    //         //     image(c_inv_slot, p);
    //         // });
    //         // text("Le grid", c_text, c_pixel, p);
    //     });
    //     node((c_half, c_blue), p, |p| {
    //         // text("This is the right pane!", c_text, c_pixel, p);
    //         // text("Indeed, I do!", c_text, c_pixel, p);
    //         // text_buttoni("Howdy", c_button_right, c_pixel, UiId::HowdyButton, p);   // Inline variant of text_button
    //     });
    // });
}

fn setup2(world: &mut World) {
    world.run_system_once(move |mut query: Query<&mut Path, With<ExampleShape>>, time: Res<Time>| {
        for mut path in query.iter_mut() {
            let sides = ((time.elapsed_seconds_f64() - PI * 2.5).sin() * 2.5 + 5.5).round() as usize;
            let polygon = shapes::RegularPolygon {
                sides,
                feature: RegularPolygonFeature::Radius(200.0),
                ..shapes::RegularPolygon::default()
            };
            *path = ShapePath::build_as(&polygon);
        }
    });
}

fn ime_toggle(
    mousebtn: Res<ButtonInput<MouseButton>>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mousebtn.just_pressed(MouseButton::Right) {
        let mut window = q_window.single_mut();

        // toggle "IME mode"
        window.ime_enabled = !window.ime_enabled;

        // We need to tell the OS the on-screen coordinates where the text will
        // be displayed; for this simple example, let's just use the mouse cursor.
        // In a real app, this might be the position of a UI text field, etc.
        window.ime_position = window.cursor_position().unwrap();
    }
}

fn ime_input(
    mut evr_ime: EventReader<Ime>,
) {
    for ev in evr_ime.read() {
        match ev {
            Ime::Commit { value, .. } => {
                println!("IME confirmed text: {}", value);
            }
            Ime::Preedit { value, cursor, .. } => {
                println!("IME buffer: {:?}, cursor: {:?}", value, cursor);
            }
            Ime::Enabled { .. } => {
                println!("IME mode enabled!");
            }
            Ime::Disabled { .. } => {
                println!("IME mode disabled!");
            }
        }
    }
}

const BLACKSMITH_OUTLINE: &str = "m
210.49052,219.61666
c
-54.97575,-3.12045
-153.83891,-43.5046
-181.900067,-79.34483
41.944976,3.29834
143.100787,1.42313
185.138697,1.61897
l
6e-5,-0.003
c
41.78023,-0.87477
200.563,-0.4537
261.24529,0
0.085,7.05106
0.79737,22.71244
1.07386,32.86306
-42.04814,8.31883
-101.90702,24.33338
-128.45794,63.97855
-10.53308,31.59203
39.6912,45.827
74.62215,55.19132
1.14898,12.80889
2.62233,32.62936
2.46309,44.71853
-75.4682,-0.86499
-141.64601,-1.07063
-209.86695,-1.35786
-10.81491,-1.77566
-6.66734,-23.1495
-4.31819,-32.38456
5.44628,-16.65332
38.03788,-18.20507
28.06768,-83.12367
-7.29786,-2.58188
-23.92259,-1.83114
-28.06768,-2.15756";
