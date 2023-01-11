pub mod map;
pub mod api;
pub mod ws;

use serde::{Deserialize};

#[derive(Eq, Deserialize, Debug, PartialEq, Hash)]
pub enum CavasType {
	Earth = 0
}

#[derive(Debug, Eq, PartialEq)]
pub struct XY {
	x: i32,
	y: i32
}

#[derive(Debug, Eq, PartialEq)]
pub struct Pixel {
	position: XY,
	color: u8
}