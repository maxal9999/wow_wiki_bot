use api::Api;
use api::types::{UrlParams, Update};
use error::Result;
use hyper::Client;
use url::Url;


/// A listening handler returns this type to signal the listening-method either
/// to stop or to continue.
#[derive(Debug)]
pub enum ListeningAction {
    Continue,
    Stop
}

/// Struct for offers methods to easily receive new updates via the specified method
pub struct Listener {
    pub confirmed: i64,
    pub url: Url,
    pub client: Client,
    pub timeout: Option<i64>
}


impl Listener {
    /// Corresponds to the "getUpdates" method of the API
    fn get_updates(&self, offset: i64, 
                   timeout: Option<i64>, 
                   limit: Option<i64>) -> Result<Vec<Update>> {
        let mut params = UrlParams::new();
        params.add_value("offset", offset);
        params.add_opt_value("timeout", timeout);
        params.add_opt_value("limit", limit);
        Api::request(&self.client, &self.url, "getUpdates", params)
    }

    /// Receive and handle updates with the given closure
    pub fn listen<H>(&mut self, mut handler: H) -> Result<()>
        where H: FnMut(Update) -> Result<ListeningAction>
    {
        // `handled_until` will hold the id of the last handled update
        let mut handled_until = self.confirmed;

        // Calculate final timeout: Given or default (30s)
        let timeout = self.timeout.or(Some(30));

        loop {
            // Receive updates with correct offset
            let updates = match self.get_updates(handled_until, timeout, None) {
                Ok(val) => val,
                Err(err) => {
                    println!("{:?}", err);
                    continue
                }
            };

            self.confirmed = handled_until;

            // For every update: Increase the offset and call the handler.
            for new_data in updates {
                let update_id = new_data.update_id;

                // Execute the handler and save it's result.
                let res = handler(new_data);

                if let Err(err) = res {
                    // Send a last request to confirm already handled updates
                    let _ = try!(self.get_updates(handled_until, None, Some(0)));
                    self.confirmed = handled_until;

                    return Err(err);
                }

                if update_id >= handled_until {
                    handled_until = update_id + 1;
                }

                if let Ok(ListeningAction::Stop) = res {
                    // Send a last request to confirm already handled updates
                    let _ = try!(self.get_updates(handled_until, None, Some(0)));

                    self.confirmed = handled_until;

                    return Ok(());
                }
            }
        }
    }
}
