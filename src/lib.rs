
pub struct Portal {
    client_id: String,
    client_secret: String
}

impl Portal {

    pub fn new (client_id: String, client_secret: String) -> Self {
        Portal { client_id, client_secret }
    }

    pub fn generate_token(&self) -> String {
        let mut payload: String = "client_id=".to_owned();
        payload.push_str(&self.client_id);
        payload.push_str("&client_secret=");
        payload.push_str(&self.client_secret);
        payload.push_str("&grant_type=client_credentials");
        payload
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_token() {
        let portal = Portal { 
            client_id: "123".to_owned(), 
            client_secret: "xyz".to_owned()
        };

        let expected_payload: &str = "client_id=123&client_secret=xyz&grant_type=client_credentials";
        let payload = portal.generate_token();
        assert_eq!(expected_payload, payload);
    }
}
