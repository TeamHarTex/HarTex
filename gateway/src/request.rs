use std::env as stdenv;

use base::discord::model::gateway::event::Event;
use base::error::{Error, ErrorKind, Result};
use hyper::client::Client;
use hyper::header::AUTHORIZATION;
use hyper::{Body, Method, Request};

pub async fn request_event(event: Event) -> Result<()> {
    match event {
        Event::Ready(ready) => {
            let client = Client::new();

            log::trace!("retrieving the port of the event server");
            let result = stdenv::var("EVENT_SERVER_PORT");
            let port = if let Ok(port) = result {
                let result = port.parse::<u16>();
                if let Ok(port) = result {
                    port
                } else {
                    log::error!("processing error: port is not an integer: {}", result.unwrap_err());
                    return Ok(());
                }
            } else {
                let error = result.unwrap_err();
                log::error!("env error: {error}");
                return Err(Error::from(error));
            };

            log::trace!("serializing ready payload");
            let result = serde_json::to_string(&ready);
            if let Err(src) = result {
                log::error!("request error: could not serialize body: {src}");
                return Err(Error::from(ErrorKind::JsonError { src }));
            }

            let result = stdenv::var("EVENT_SERVER_AUTH");
            let auth = if let Ok(auth) = result {
                auth
            } else {
                let error = result.unwrap_err();
                log::error!("env error: {error}");
                return Err(Error::from(error));
            };

            log::trace!("building request");
            let result = Request::builder()
                .header(AUTHORIZATION, auth)
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
