#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate failure;
extern crate image;
extern crate webrender;

pub mod images;

use std::sync::{Mutex, MutexGuard};
use std::default::Default;

use webrender::api::RenderApiSender;
use self::images::ImageLoader;

lazy_static! {
  static ref RESOURCES: Mutex<Resources> = Mutex::new(Resources::new());
}

pub fn init_resources(render_api: RenderApiSender) {
  RESOURCES.try_lock().unwrap().set_render_api(render_api);
}

// Allow global access to Resources
pub fn resources() -> MutexGuard<'static, Resources> {
  RESOURCES.try_lock().unwrap()
}

pub struct Resources {
  pub image_loader: ImageLoader,
}

impl Default for Resources {
  fn default() -> Self {
    Resources {
      image_loader: ImageLoader::new(),
    }
  }
}

impl Resources {
  pub fn new() -> Self {
    Self::default()
  }

  fn set_render_api(&mut self, render: RenderApiSender) {
    self.image_loader.render = Some(render.create_api());
  }
}
