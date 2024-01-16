use crate::errors::Error as AnalyticsError;
use crate::message::Message;
use crate::utils;
use std::time::Duration;
use tracing::debug;

// Rudderanalytics client
pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::Client,
}

impl RudderAnalytics {
    // Function to initialize the Rudderanalytics client with write-key and data-plane-url
    pub fn load(write_key: String, data_plane_url: String) -> RudderAnalytics {
        let mut client = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        {
            client = client.timeout(Duration::new(10, 0));
        }
        let client = client.build().unwrap();
        RudderAnalytics {
            write_key,
            data_plane_url,
            client,
        }
    }

    // Function that will receive user event data
    // and after validation
    // modify it to Ruddermessage format and send the event to data plane url
    pub async fn send(&self, msg: &Message) -> Result<(), AnalyticsError> {
        let id_err_msg = String::from("Either of user_id or anonymous_id is required");
        let reserve_key_err_msg = String::from("Reserve keyword present in context");
        let empty_msg = String::from("");
        let mut error_msg: String = String::from("");

        // match the type of event and fetch the proper API path
        let path = match msg {
            Message::Identify(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id.is_none() && b_.anonymous_id.is_none() {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context.is_some()
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/identify"
            }
            Message::Track(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id.is_none() && b_.anonymous_id.is_none() {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context.is_some()
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/track"
            }
            Message::Page(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id.is_none() && b_.anonymous_id.is_none() {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context.is_some()
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/page"
            }
            Message::Screen(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id.is_none() && b_.anonymous_id.is_none() {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context.is_some()
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/screen"
            }
            Message::Group(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id.is_none() && b_.anonymous_id.is_none() {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context.is_some()
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/group"
            }
            Message::Alias(b_) => {
                // Checking conflicts with reserved keywords
                if b_.context.is_some()
                    && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                {
                    error_msg = reserve_key_err_msg;
                }
                "/v1/alias"
            }
            Message::Batch(b_) => {
                // Checking conflicts with reserved keywords
                if b_.context.is_some()
                    && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                {
                    error_msg = reserve_key_err_msg;
                }
                "/v1/batch"
            }
        };

        if error_msg.is_empty() {
            // match the type of event and manipulate the payload to rudder format
            let rudder_message = match msg {
                Message::Identify(b_) => utils::parse_identify(b_),
                Message::Track(b_) => utils::parse_track(b_),
                Message::Page(b_) => utils::parse_page(b_),
                Message::Screen(b_) => utils::parse_screen(b_),
                Message::Group(b_) => utils::parse_group(b_),
                Message::Alias(b_) => utils::parse_alias(b_),
                Message::Batch(b_) => utils::parse_batch(b_),
            };

            // final payload
            debug!("rudder_message: {:#?}", rudder_message);
            // Send the payload to the data plane url
            let res = self
                .client
                .post(&format!("{}{}", self.data_plane_url, path))
                .basic_auth(self.write_key.to_string(), Some(""))
                .json(&rudder_message)
                .send()
                .await?;

            // handle error and send response
            if res.status() == 200 {
                Ok(())
            } else {
                Err(AnalyticsError::InvalidRequest(format!(
                    "status code: {}, message: Invalid request",
                    res.status()
                )))
            }
        } else {
            Err(AnalyticsError::InvalidRequest(error_msg))
        }
    }
}
