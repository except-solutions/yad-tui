pub struct DiskClient {
    pub api_url: String,
    pub oauth_url: String,
    pub client_id: String,
    pub token: Option<String>,
}

impl DiskClient {
    fn auth(&self, code: String) {
        // TODO impl
        let url = &format!(
            "{}&client_id={}",
            self.oauth_url.clone(),
            self.client_id.clone()
        );
        let get = ureq::get(url);
    }
}
