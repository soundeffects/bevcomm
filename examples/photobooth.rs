use bevomms::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevommsPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut webcams: NonSendMut<Webcams>, images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    if let Ok(image_handle) = webcams.open(
        images,
        CameraIndex::Index(0),
        RequestedFormatType::AbsoluteHighestFrameRate,
    ) {
        commands.spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                ImageNode::new(image_handle),
                Node {
                    border: px(5.).all(),
                    padding: px(10.).all(),
                    width: px(256.),
                    height: px(256.),
                    ..default()
                },
                BorderColor::all(Color::WHITE)
            )],
        ));
    } else {
        println!("Could not open webcam");
    }
}
