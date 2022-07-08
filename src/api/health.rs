use poem_openapi::payload::PlainText;
use poem_openapi::OpenApi;

pub struct Health;

#[OpenApi]
impl Health {
    #[oai(path = "/health", method = "get")]
    async fn health_check(&self) -> PlainText<String> {
        PlainText(format!("Healthy, running as process: {}", std::process::id()))
    }
}
