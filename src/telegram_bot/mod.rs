
//candidade for sending: https://crates.io/crates/awesome-bot
//actix (old) based: https://github.com/jeizsm/actix-telegram/blob/master/examples/server.rs
// https://crates.io/crates/telegram-bot-raw


//plot generation using: https://github.com/38/plotters/tree/master/examples

use actix_web::web::{HttpResponse, Data, Form, Bytes};
use actix_web::http::StatusCode;

use reqwest;
use log::{warn, info ,error};
use serde::Deserialize;
use serde_json;

use telegram_bot::types::update::Update;
use telegram_bot::types::requests::send_message::SendMessage;
use telegram_bot::types::requests::_base::{HttpRequest, Request};
use telegram_bot::types::ToChatRef;
use telegram_bot::types::update::UpdateKind;
use telegram_bot::types::HttpResponse as telegramResponse;

use crate::httpserver::InnerState;

pub fn handle_bot_message<T: InnerState+'static>(state: Data<T>, raw_update: Bytes)
	 -> HttpResponse {
	
	//TODO make actix web deserialise bot messages to: 
	//"telegram_bot::types::update::Update", then we can handle upon that object

    dbg!("got telegrambot message");
	//FIXME TODO
	pub const TOKEN: &str = "109451485:AAE6Yghjq1qJsxu75uureFkvaMB_Zrt7YsY";

	let update: Update = serde_json::from_slice(&raw_update.to_vec()).unwrap();
	
	match &update.kind{
		UpdateKind::Message(message) => send_test_reply(&message.chat, TOKEN).unwrap(),
		_ => warn!("unhandled message type"),
	}
	HttpResponse::Ok().status(StatusCode::OK).finish()
}

#[derive(Debug)]
pub enum BotError{
	HttpClientError(reqwest::Error),
	CouldNotSetWebhook,
}

impl From<reqwest::Error> for BotError {
	fn from(error: reqwest::Error) -> Self {
		BotError::HttpClientError(error)
	}
}

fn send_test_reply<C: ToChatRef>(chat: C, token: &str) -> Result<(), BotError>{//add as arg generic ToChatRef (should get from Update)
	//TODO create a SendMessage, serialise it (use member function serialize) 
	//then use the HttpRequest fields, (url, method, and body) to send to telegram
	let text = String::from("hi");
	let request = SendMessage::new(chat, text).serialize().unwrap();
	//dbg!(&request);
	let HttpRequest {url: request_url, body, method: _} = request;

	match body {
		telegram_bot::Body::Empty => warn!("ERROR body empty"),
		telegram_bot::Body::Json(body) => {
			dbg!(&body);
			dbg!(&request_url.url(token));
			let client = reqwest::Client::new();
			client.post(request_url.url(token).as_str())
			      .body(body)
				  .send()?;
			info!("send message")
		},
		_ => warn!("Error unhandled body type"),
	}
	Ok(())
}

pub fn set_webhook(domain: &str, token: &str) -> Result<(), BotError> {
	let url = format!("https://api.telegram.org/bot{}/setWebhook", token);
	let webhook_url = format!("{}:8443/{}",domain, token);

	let params = [("url", &webhook_url)];
	let client = reqwest::Client::new();
	let res = client.post(url.as_str())
	      .form(&params)
		  .send()?;
	if res.status() != reqwest::StatusCode::OK {
		dbg!(res);
		Err(BotError::CouldNotSetWebhook)
	} else {
		info!("set webhook to: {}", webhook_url);
		Ok(())
	}
}

/*
fn send_plot(){
	//"sendChatAction" photo (shows taking photo)
	//The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status).
	//keep sending every 5 seconds

	//send inputMediaPhoto with media string "attach://<file_attach_name>"
	//Post the file using multipart/form-data to "<file_attach_name>"
	//When sending by URL the target file must have the correct MIME type (e.g., audio/mpeg for sendAudio, etc.).
}*/