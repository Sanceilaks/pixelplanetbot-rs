use std::collections::HashMap;

use super::CavasType;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Canvas {
	size: i32,
	v: bool, // Is 3D
	colors: Vec<u8>,
	bcm: i32, // Place Cooldown
	pcd: i32, // Replace Cooldown
	cds: i32, // Cumulative Cooldown
}

#[derive(Deserialize, Debug)]
pub struct User {
	name: String,
	canvases: HashMap<CavasType, Canvas>
}

pub async fn download_chunk(x: i32, y: i32, canvas: CavasType) -> Vec<u8> {
	let url = std::format!("https://pixelplanet.fun/chunks/{}/{}/{}.bmp", canvas as u8, x, y);
	reqwest::get(url).await.expect("Cannot get chunk")
		.bytes().await
		.unwrap().to_vec()
}

pub async fn get_me() -> User {
	let url = "https://pixelplanet.fun/api/me";
	reqwest::get(url).await.expect("Cannot get session information")
		.json().await.expect("Cannot parse response from /api/me")
}