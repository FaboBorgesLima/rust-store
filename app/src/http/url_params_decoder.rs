use std::collections::HashMap;

pub struct UrlParamsDecoder {}
impl UrlParamsDecoder {
    pub fn decode(params: &String) -> HashMap<String, String> {
        let mut decoded = HashMap::new();

        let params_clone = params.clone();

        let parts = params_clone.split("&");

        for part in parts {
            let mut key_value = part.split("=");

            let key = key_value.nth(0);

            let value = key_value.nth(0).unwrap_or("").to_string();

            let key = match key {
                Some(key) => key.to_string(),
                None => continue,
            };

            decoded.insert(key, value);
        }
        decoded
    }
}

#[cfg(test)]
mod test {
    use super::UrlParamsDecoder;

    #[test]
    fn it_decodes() {
        let decoded = UrlParamsDecoder::decode(&"key=value&key2=value2".to_string());

        assert_eq!(*decoded.get("key").unwrap(), "value".to_string());

        assert_eq!(*decoded.get("key2").unwrap(), "value2".to_string());

        assert_eq!(*decoded.get("key").unwrap(), "value".to_string());

        assert_eq!(*decoded.get("key2").unwrap(), "value2".to_string());
    }
}
