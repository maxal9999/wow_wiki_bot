#[macro_use]
pub mod listener;
pub mod types;

use hyper::Client;
use hyper::header::{Connection, ContentType, ContentLength};
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;
use rustc_serialize::{json, Decodable};
use std::io::Read;
use std::time::Duration;
use self::types::{UrlParams, Response};
use super::error::{Error, Result};
use url::Url;


/// Base struct for work with web application
pub struct Api {
    pub client: Client,
    pub url: Url
}

impl Clone for Api {
    fn clone(&self) -> Api {
        Api {
            client: Api::create_default_client(),
            url: self.url.clone()
        }
    }
}

impl Api {
    /// Method for create connection with default settings.
    /// hyper_rustls::TlsClient::new() - create atomic pointer to rustls::ClientConfig.
    /// rustls::ClientConfig - common configuration for all connections made by a program.
    /// HttpsConnector::new(tls_client) - generate connect with the underlying OpenSSL context. 
    pub fn create_default_client() -> Client {
        let tls_client = TlsClient::new();
        let https_connector = HttpsConnector::new(tls_client);
        let mut conn = Client::with_connector(https_connector);
        conn.set_read_timeout(Some(Duration::new(5, 0)));
        conn.set_write_timeout(Some(Duration::new(5, 0)));
        conn
    }

    /// Method for create a new object api by url endpoint
    pub fn new(new_url: Url) -> Api {
        Api {
            client: Api::create_default_client(),
            url: new_url
        }
    }

    /// Method for post request by url
    pub fn send_request<T: Decodable>(&self, method: &str, params: UrlParams) -> Result<T> {
        Self::request(&self.client, &self.url, method, params)
    }

    /// Method for post request by url
    pub fn request<T: Decodable>(client: &Client, enter_url: &Url, 
                                 method: &str, params: UrlParams) -> Result<T> {
        let mut url = enter_url.clone();

        // Delete last call method and add new
        if let Ok(mut segments_mut) = url.path_segments_mut() {
            segments_mut.pop().push(method.into());
        }

        // Formed full url with params
        let url_params = params.get_url_string();

        // Create request with the body and headers
        let request = client
            .post(url)
            .body(&*url_params)
            .header(Connection::close())
            .header(ContentType::form_url_encoded())
            .header(ContentLength(url_params.len() as u64));

        // Send request
        let mut resp = try!(request.send());

        // Read response into string and return error if it failed
        let mut resp_body = String::new();
        try!(resp.read_to_string(&mut resp_body));

        // Try to decode response as JSON representing a Response
        match json::decode(&resp_body).unwrap() {
            // If ok == false, return Api error
            Response { ok: false, description: Some(desc), ..} => {
                Err(Error::Api(desc))
            },
            Response { ok: true, result: Some(res), ..} => {
                Ok(res)
            },
            // If some other state, return InvalidState error
            _ => Err(Error::InvalidState("Invalid server response".into())),
        }
    }
}