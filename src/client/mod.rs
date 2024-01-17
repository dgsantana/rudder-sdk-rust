#[cfg(not(target_arch = "wasm32"))]
mod blocking;
mod non_blocking;

#[cfg(not(target_arch = "wasm32"))]
pub use blocking::RudderAnalytics;
#[cfg(not(target_arch = "wasm32"))]
pub use non_blocking::AsyncRudderAnalytics;

// For wasm we can only use non-blocking client
#[cfg(target_arch = "wasm32")]
pub use non_blocking::AsyncRudderAnalytics as RudderAnalytics;

use crate::message::Message;
use crate::utils;

fn prepare_message(msg: &Message) -> (String, &str) {
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
    (error_msg, path)
}
