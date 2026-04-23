use bevy::{
    asset::RenderAssetUsages,
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use nokhwa::{pixel_format::RgbAFormat, utils::RequestedFormat, Camera, NokhwaError};

pub use nokhwa::utils::{CameraIndex, RequestedFormatType};

pub struct BevommsPlugin;

impl Plugin for BevommsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_non_send_resource(Webcams::default())
            .add_systems(Update, (update_webcams, cleanup_webcams));
    }
}

#[derive(Default)]
pub struct Webcams {
    instances: HashMap<CameraIndex, (Camera, Handle<Image>)>,
}

impl Webcams {
    pub fn open(
        &mut self,
        mut images: ResMut<Assets<Image>>,
        index: CameraIndex,
        format_type: RequestedFormatType,
    ) -> Result<Handle<Image>, NokhwaError> {
        let mut camera = Camera::new(
            index.clone(),
            RequestedFormat::new::<RgbAFormat>(format_type),
        )?;
        camera.open_stream()?;

        let (width, height) = (camera.resolution().width(), camera.resolution().height());
        let handle = images.add(Image::new(
            Extent3d {
                width,
                height,
                ..default()
            },
            TextureDimension::D2,
            vec![255; (width * height * 4) as usize],
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        ));
        let weak_handle = handle.clone();

        self.instances.insert(index, (camera, handle));
        Ok(weak_handle)
    }

    pub fn close(&mut self, index: CameraIndex) -> Result<(), NokhwaError> {
        if let Some((camera, _target)) = self.instances.get_mut(&index) {
            camera.stop_stream()?;
            self.instances.remove(&index);
            // TODO: Delete target images
        }
        Ok(())
    }

    fn update_all(&mut self, mut images: ResMut<Assets<Image>>) {
        // TODO: Test random image updates to see if Bevy renders image updates as expected
        for (_index, (camera, target)) in &mut self.instances {
            if let Ok(frame) = camera.frame() {
                if let Some(data) = images.get_mut(target).and_then(|image| image.data.as_mut()) {
                    println!("old: {}, new: {}", data.len(), frame.buffer().len());
                    if data.len() == frame.buffer().len() {
                        data.copy_from_slice(frame.buffer());
                    }
                }
            } else {
                // TODO: Close the camera and print a warning if it failed
            }
        }
    }

    pub fn close_all(&mut self) -> Result<(), NokhwaError> {
        for (_index, (camera, _target)) in &mut self.instances {
            camera.stop_stream()?;
            // TODO: Delete target images
        }
        self.instances.clear();
        Ok(())
    }
}

fn update_webcams(mut webcams: NonSendMut<Webcams>, images: ResMut<Assets<Image>>) {
    webcams.update_all(images);
}

fn cleanup_webcams(app_exit: MessageReader<AppExit>, mut webcams: NonSendMut<Webcams>) {
    if !app_exit.is_empty() && let Err(error) = webcams.close_all() {
        warn!("Error when closing webcams: {:?}", error);
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
