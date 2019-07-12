use reqwest;
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

    fn is_authentication_ok(&self) -> bool {
        let auth = match self.auth {
            Authentication::KeyToken(key, token) => (key, token)
        };

        let url = format!("https://api.trello.com/1/{0}",
                          format!("members/me?key={}&token={}",
                                  auth.0, auth.1));
        println!("Value {}", url.trim());
        let response: HashMap<String, String> = match reqwest::get(url.trim()) {
            Err(_) => return false,
            Ok(mut res) => match res.json() {
                Err(_) => return false,
                Ok(v) => v
            }
        };

        println!("{:#?}", response);
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
}