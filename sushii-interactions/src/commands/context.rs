use twilight_http::Client;

pub struct CommandContext<'a> {
    pub http: &'a Client,
}
