use super::client::DnseClient;
use reqwest::Method;

impl DnseClient {
    pub async fn get_list_care_by(&self, dry_run: bool) -> Result<(u16, Vec<u8>), Box<dyn std::error::Error>> {
        self.request(Method::GET, "/brokers/accounts/care-by", None, None, None, dry_run).await
    }
}
