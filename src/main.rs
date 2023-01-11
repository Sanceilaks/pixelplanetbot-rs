use websocket::header::{Headers, Cookie, UserAgent};

pub mod pixelplanet;

#[tokio::main]
async fn main() {
	let mut my_headers = Headers::new();
	my_headers.set(UserAgent(
		"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_owned()
	));

	let mut client = websocket::ClientBuilder::new("wss://pixelplanet.fun/ws")
		.unwrap()
		.custom_headers(&my_headers)
		.origin("https://pixelplanet.fun".into())
		.connect(None).expect("Cannot connect with websocket");

	client.writer_mut().write(&pixelplanet::ws::subscribe_to_canvas(pixelplanet::CavasType::Erath)).unwrap();
	
	loop {
		let mut buff = Vec::new();
		if let Ok(num) = client.reader_mut().read_to_end(&mut buff) {
			if num > 0 {
				println!("{:#?}", buff);
			}
		}
	}

}
