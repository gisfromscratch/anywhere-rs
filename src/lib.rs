use std::collections::HashMap;

use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use oauth2::url;

use reqwest:: {
    Client,
    header::HeaderMap,
    StatusCode
};

use serde::Deserialize;



pub struct Portal {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
pub struct AccessToken {
    access_token: String,
    expires_in: i64,
}

impl Portal {

    pub fn new (client_id: String, client_secret: String) -> Self {
        Portal { client_id, client_secret }
    }

    fn create_oauth2_client(&self) -> Result<BasicClient, url::ParseError> {
        let client = BasicClient::new(
            ClientId::new(self.client_id.clone()), 
            Some(ClientSecret::new(self.client_secret.clone())), 
            AuthUrl::new("https://www.arcgis.com/sharing/rest/oauth2/token".to_string())?,
            Some(TokenUrl::new("https://www.arcgis.com/sharing/rest/oauth2/token".to_string())?,
        ));

        Ok(client)
    }

    fn create_client(&self) -> reqwest::Result<Client> {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert("accept", "application/json".parse().unwrap());
        headers.insert("cache-control", "no-cache".parse().unwrap());
        
        Client::builder()
            .default_headers(headers)
            .build()
    }

    fn generate_payload(&self) -> String {
        let mut payload: String = "client_id=".to_owned();
        payload.push_str(&self.client_id);
        payload.push_str("&client_secret=");
        payload.push_str(&self.client_secret);
        payload.push_str("&grant_type=client_credentials");
        payload
    }

    fn generate_payload_params(&self) -> HashMap<&str, String> {
        let mut payload_params = HashMap::new();
        payload_params.insert("client_id", self.client_id.clone());
        payload_params.insert("client_secret", self.client_secret.clone());
        payload_params.insert("grant_type", String::from("client_credentials"));
        payload_params
    }

    pub fn generate_token(&self) -> String {
        let payload_params = self.generate_payload_params();
        let client_result = self.create_client();
        match client_result {
            Ok(client) => {
                let response_result = client.post("https://www.arcgis.com/sharing/rest/oauth2/token")
                    .form(&payload_params)
                    .send();
                match response_result {
                    Ok(mut response) => {
                        match response.status() {
                            StatusCode::OK => {
                                // TODO: Still can fail
                                // usually only some error messages are created!
                                println!("SUCCESS...");
                                let json_result: reqwest::Result<AccessToken> = response.json();
                                match json_result {
                                    Ok(access_token) => {
                                        println!("TOKEN: {}", access_token.to_token());
                                        return access_token.to_token();
                                    },

                                    Err(err) => {
                                        println!("ERROR: {}", err);
                                        match response.text() {
                                            Ok(response_text) => {
                                                println!("RESPONSE: {}", response_text);
                                                return response_text;
                                            },

                                            Err(err) => {
                                                println!("ERROR: {}", err);
                                                return String::from("");
                                            }
                                        }

                                        //return String::from("1");
                                    }
                                };
                            }

                            status_code => println!("Received response status: {:?}", status_code),
                        };

                        return String::from("")
                    }

                    Err(err) => {
                        println!("ERROR: {}", err);
                        return String::from("");
                    }
                }
            }

            Err(err) => {
                println!("ERROR: {}", err);
                return String::from("");
            }
        };       
    }
}


impl AccessToken {

    pub fn to_token(&self) -> String {
        return self.access_token.clone();
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    use std::env;

    #[test]
    fn generate_token() {
        let portal = Portal { 
            client_id: "123".to_owned(), 
            client_secret: "xyz".to_owned()
        };

        let expected_payload: &str = "client_id=123&client_secret=xyz&grant_type=client_credentials";
        let payload = portal.generate_payload();
        assert_eq!(expected_payload, payload);
    }

    #[test]
    fn generate_token_using_environment() {
        let client_id_key = "portal.appid";
        let client_id = env::var(client_id_key);
        assert_eq!(true, client_id.is_ok());
        
        let client_secret_key = "portal.clientid";
        let client_secret = env::var(client_secret_key);
        assert_eq!(true, client_secret.is_ok());

        let portal = Portal::new(client_id.unwrap(), client_secret.unwrap());
        let payload = portal.generate_payload();
        assert_ne!(true, payload.is_empty());

        let token = portal.generate_token();
        assert_ne!(true, token.is_empty());
    }
}
