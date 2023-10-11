mod models;

pub use models::*;

pub struct Client {
    host: String,
    port: u16,
    inner: reqwest::Client,
}

impl Client {
    pub fn new<H: Into<String>, P: Into<u16>>(host: H, port: P) -> Self {
        Client {
            host: host.into(),
            port: port.into(),
            inner: reqwest::Client::default(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!(
            "http://{host}:{port}/zeroconf/{path}",
            host = self.host,
            port = self.port
        )
    }

    pub async fn fetch_info(&self) -> anyhow::Result<Info> {
        let res = self
            .inner
            .post(self.url("info"))
            .body("{\"data\":{}}")
            .send()
            .await?;
        Ok(res.json::<InfoResponse>().await?.into())
    }

    pub async fn set_startup_position(&self, position: StartupPosition) -> anyhow::Result<()> {
        self.inner
            .post(self.url("startups"))
            .json(&StartupsRequest::from(position))
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;

    fn load_fixture(fpath: &str) -> String {
        let read = std::fs::read_to_string(format!("./testing_fixtures/{}", fpath)).unwrap();
        jsonxf::minimize(&read).unwrap()
    }

    fn make_server_and_client() -> (MockServer, Client) {
        let server = MockServer::start();
        let client = Client::new(server.host(), server.port());
        (server, client)
    }

    #[tokio::test]
    async fn info_returns_expected_result() {
        let (server, client) = make_server_and_client();
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/zeroconf/info")
                .body("{\"data\":{}}");
            then.status(200)
                .header("content-type", "application/json; charset=utf-8")
                .body(load_fixture("response_info_ok.json"));
        });

        let got = client.fetch_info().await;

        mock.assert();

        assert!(got.is_ok());
        assert_eq!(
            got.unwrap(),
            Info {
                switch: SwitchPosition::Off,
                startup: StartupPosition::Off
            }
        )
    }

    #[tokio::test]
    async fn set_startup_sent_expected_request() {
        let (server, client) = make_server_and_client();
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/zeroconf/startups")
                .body(load_fixture("request_startups_ok.json"));
            then.status(200)
                .header("content-type", "application/json; charset=utf-8")
                .body(load_fixture("response_ok.json"));
        });

        let got = client.set_startup_position(StartupPosition::Stay).await;

        mock.assert();

        assert!(got.is_ok());
    }
}
