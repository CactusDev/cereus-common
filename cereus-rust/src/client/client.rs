
use ws;
use serde_json as json;

pub struct Client {
	client: Option<ws::Sender>
}

impl Client {

	pub fn new() -> Self{
		Client {
			client: None
		}
	}
}

impl ws::Handler for Client {

	fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
		println!("Connected to Cereus!");

		Ok(())
	}

	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		Ok(())
	}
}
