use super::CavasType;

pub enum Opcode {
	RegisterCanvas = 0xA0,
	RegisterChunk = 0xA1,
	UnregisterChunk = 0xA2,
	RegisterMultipleChunks = 0xA3,
	UnregisterMultipleChunks = 0xA4,
	ChangedMe = 0xA6,
	OnlineCounter = 0xA7,
	Ping = 0xB0,
	PixelUpdate = 0xC1,
	Cooldown = 0xC2,
	PixelReturn = 0xC3,
	CaptchaReturn = 0xC6
}

pub fn subscribe_to_canvas(canvas: CavasType) -> Vec<u8> {
	vec![Opcode::RegisterCanvas as u8, canvas as u8]
}