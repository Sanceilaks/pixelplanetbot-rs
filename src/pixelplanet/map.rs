use super::{XY, Pixel};

// Returns chunk and relative position
pub fn relative_to_absolute(map_size: i32, val: i32) -> (u8, u8) {
	let shifted = val + map_size / 2;
	((shifted / 256) as u8, (shifted % 256) as u8)
}

pub fn calc_chunks(map_size: i32, pixels: &Vec<Pixel>) -> Vec<XY> {
	let mut chunks: Vec<XY> = pixels.iter().map(move |p| {
		XY {
			x: relative_to_absolute(map_size, p.position.x).0 as i32,
			y: relative_to_absolute(map_size, p.position.y).0 as i32
		}
	}).collect();
	chunks.dedup_by(|a, b| a == b);
	chunks
}