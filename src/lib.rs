#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate failure;
extern crate image;
extern crate webrender;

mod images;

use std::collections::HashMap;
use std::default::Default;
use std::sync::{Mutex, MutexGuard};

use self::images::ImageLoader;
use webrender::api::RenderApiSender;

lazy_static! {
  static ref RES: Mutex<Resources> = Mutex::new(Resources::new());
}

pub fn init_resources(render_api: RenderApiSender) {
  RES.try_lock().unwrap().set_render_api(render_api);
}

// Allow global access to Resources
pub fn resources() -> MutexGuard<'static, Resources> {
  RES.try_lock().unwrap()
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
