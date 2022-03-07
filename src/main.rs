#![allow(dead_code)]

mod rayt;
mod scene;

use crate::rayt::*;
use crate::scene::*;

fn main() {
    render_aa(SimpleScene::new());
}
