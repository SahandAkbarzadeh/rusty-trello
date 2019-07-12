use reqwest;
use url::Url;

use std::collections::HashMap;

enum Authentication {
    KeyToken(&'static str, &'static str)
}

struct TrelloClient {
    auth: Authentication
}

impl TrelloClient {
    fn new(auth: Authentication) -> Self {
        TrelloClient { auth }
    }

    fn create_url(&self, method: &str, parameters: Option<Vec<(&str, &str)>>) -> String {
        let params: Vec<(&str, &str)> = match parameters {
            None => vec![],
            Some(value) => value
        };

        let auth = match self.auth {
            Authentication::KeyToken(key, token) => (key, token)
        };

        let params_with_keys: Vec<(&str, &str)> = params.into_iter()
            .chain(vec![("key", auth.0), ("token", auth.1)].into_iter())
            .collect();

        let url = format!("https://api.trello.com/1/{0}", method);

        Url::parse_with_params(url.trim(), params_with_keys.into_iter()).unwrap().into_string()
    }

    fn is_authentication_ok(&self) -> bool {
        let response: HashMap<String, String> = match reqwest::get(self.create_url("members/me", None).trim()) {
            Err(_) => return false,
            Ok(mut res) => match res.json() {
                Err(_) => return false,
                Ok(v) => v
            }
        };

        match response.get("key") {
            None => false,
            Some(value) => value == "value"
        }
    }
}


#[cfg(test)]
mod test_basic_validations {
    use super::*;

    #[test]
    fn test_authenticate_fail() {
        let client = TrelloClient::new(Authentication::KeyToken("wrong", "fail"));
        assert!(!client.is_authentication_ok());
    }

    #[test]
    fn test_make_request_url() {
        let client = TrelloClient::new(Authentication::KeyToken("wrong", "fail"));
        assert_eq!(client.create_url("members/me", None), "https://api.trello.com/1/members/me?key=wrong&token=fail");
        assert_eq!(client.create_url("members/me", Option::Some(vec![("test_key", "value")])), "https://api.trello.com/1/members/me?test_key=value&key=wrong&token=fail");
    }
}