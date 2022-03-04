#![allow(dead_code)]

mod rayt;
mod simple_scene;

use crate::rayt::*;
use crate::simple_scene::SimpleScene;
fn main() {
    render_aa(SimpleScene::new());
}
