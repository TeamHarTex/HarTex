use base::discord::model::gateway::event::Event;
use base::error::{Error, ErrorKind, Result};
use env::EnvVarValue;
use hyper::client::Client;
use hyper::{Body, Error as HyperError, Method, Request};

use crate::ENV;

pub async fn request_event(event: Event) -> Result<()> {
    match event {
        Event::ShardIdentifying(identifying) => {
            let client = Client::new();

            log::trace!("retrieving the port of the event server");
            let port = match &ENV.as_ref().unwrap()["EVENT_SERVER_PORT"] {
                EnvVarValue::U16(port) => port,
                _ => unreachable!(),
            };

            let result = serde_json::to_string(&event);
            if let Err(src) = result {
                log::trace!("request error: could not serialize body: {src}");
                return Err(Error::from(ErrorKind::JsonError { src }));
            }

            let result = Request::builder()
                .method(Method::POST)
                .uri(format!("http://localhost:{port}/identify"))
                .body(Body::from(result.unwrap()));

            if let Err(src) = result {
                log::trace!("request error: could not build request: {src}");
                return Err(Error::from(ErrorKind::HttpError { src }));
            }

            if let Err(src) = client.request(result.unwrap()).await {
                log::trace!("request error: could not send request: {error}");
                return Err(Error::from(ErrorKind::HyperError { src }));
            }
        }
        _ => unimplemented!(),
    }

    Ok(())
}
