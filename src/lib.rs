use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera, NokhwaError,
};

pub struct BevommsPlugin;

impl Plugin for BevommsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_video_streams.pipe(print_error));
    }
}

pub enum VideoSource {
    Webcam { index: u32 },
    Network,
}

#[derive(Component)]
pub struct Video {
    source: VideoSource,
    target: Handle<Image>,
}

impl Video {
    pub fn new(
        source: VideoSource,
        mut images: ResMut<Assets<Image>>,
    ) -> Result<Self, NokhwaError> {
        match source {
            VideoSource::Webcam { index } => {
                let camera = Camera::new(
                    CameraIndex::Index(index),
                    RequestedFormat::new::<RgbAFormat>(
                        RequestedFormatType::AbsoluteHighestFrameRate,
                    ),
                )?;
                let (width, height) = (camera.resolution().width(), camera.resolution().height());
                Ok(Self {
                    source,
                    target: images.add(Image::new(
                        Extent3d {
                            width,
                            height,
                            ..default()
                        },
                        TextureDimension::D2,
                        vec![255; (width * height * 4) as usize],
                        TextureFormat::Rgba8UnormSrgb,
                        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
                    )),
                })
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn image_handle(&self) -> Handle<Image> {
        self.target.clone()
    }
}

fn update_video_streams(videos: Query<&Video>, mut images: ResMut<Assets<Image>>) -> Result<()> {
    for video in videos {
        if let Some(image) = images.get_mut(&video.image_handle()) {
            match video.source {
                VideoSource::Webcam { index } => {
                    let mut camera = Camera::new(
                        CameraIndex::Index(index),
                        RequestedFormat::new::<RgbAFormat>(
                            RequestedFormatType::AbsoluteHighestFrameRate,
                        ),
                    )?;

                    camera.open_stream()?;

                    let frame = camera.frame()?;

                    camera.stop_stream()?;

                    if let Some(data) = &mut image.data {
                        println!("old: {}, new: {}", data.len(), frame.buffer().len());
                        if data.len() == frame.buffer().len() {
                            data.copy_from_slice(frame.buffer());
                        }
                    }
                }
                _ => {
                    todo!()
                }
            }
        }
    }
    Ok(())
}

fn print_error(In(result): In<Result<()>>) {
    if let Err(error) = result {
        warn!("Error from piped system: {:?}", error);
    }
}

pub struct Voice;
pub struct Presence;

#[cfg(test)]
mod test {
    #[test]
    fn todo() {
        todo!();
    }
}
