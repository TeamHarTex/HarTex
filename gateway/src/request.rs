use base::discord::model::gateway::event::Event;
use base::error::{Error, ErrorKind, Result};
use env::EnvVarValue;
use hyper::client::Client;
use hyper::{Body, Method, Request};

use crate::ENV;

pub async fn request_event(event: Event) -> Result<()> {
    match event {
        Event::Ready(ready) => {
            let client = Client::new();

            log::trace!("retrieving the port of the event server");
            let port = match &ENV.as_ref().unwrap()["EVENT_SERVER_PORT"] {
                EnvVarValue::U16(port) => port,
                _ => unreachable!(),
            };

            log::trace!("serializing ready payload");
            let result = serde_json::to_string(&ready);
            if let Err(src) = result {
                log::error!("request error: could not serialize body: {src}");
                return Err(Error::from(ErrorKind::JsonError { src }));
            }

            log::trace!("building request");
            let result = Request::builder()
                .method(Method::POST)
                .uri(format!("http://127.0.0.1:{port}/ready"))
                .body(Body::from(result.unwrap()));
            if let Err(src) = result {
                log::error!("request error: could not build request: {src}");
                return Err(Error::from(ErrorKind::HttpError { src }));
            }

            log::trace!("sending request to http://127.0.0.1:{port}/ready");
            if let Err(src) = client.request(result.unwrap()).await {
                log::error!("request error: could not send request: {src}");
                return Err(Error::from(ErrorKind::HyperError { src }));
            }
        }
        _ => (),
    }

    Ok(())
}
