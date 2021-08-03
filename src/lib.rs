pub mod base {
    use reqwest::blocking::Client;
    use reqwest::blocking::Response;
    use reqwest::header::HeaderMap;
    use reqwest::Method;
    pub struct Base {
        name: String,
        project_key: String,
        project_id: String,
        host: String,
    }
    impl Base {
        pub fn new(name: String, project_key: String, project_id: String) -> Self {
            Base {
                name: name,
                project_key: project_key,
                project_id: project_id,
                host: String::from("https://database.deta.sh"),
            }
        }
        pub fn get(&self, key: String) -> reqwest::Result<Response> {
            self.request(
                format!("/v1/{}/{}/items/{}", self.project_id, self.name, key),
                Method::GET,
            )
        }
        fn request(&self, path: String, method: Method) -> reqwest::Result<Response> {
            let client = Client::new();
            let url = format!("{}{}", self.host, path);
            let mut headers = HeaderMap::new();
            headers.insert("X-Api-Key", self.project_key.parse().unwrap());
            let res = client.request(method, url).headers(headers).send();
            res
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
