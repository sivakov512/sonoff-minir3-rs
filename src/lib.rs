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
            "http://{host}:{port}{path}",
            host = self.host,
            port = self.port
        )
    }

    pub async fn info(&self) -> anyhow::Result<Info> {
        let res = self
            .inner
            .post(self.url("/zeroconf/info"))
            .body("{\"data\":{}}")
            .send()
            .await?;
        Ok(res.json::<InfoResponse>().await?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use rstest::*;

    #[fixture]
    fn server() -> MockServer {
        MockServer::start()
    }

    #[fixture]
    fn client(server: MockServer) -> Client {
        Client::new(server.host(), server.port())
    }

    fn load_fixture(fpath: &str) -> String {
        std::fs::read_to_string(format!("./testing_fixtures/{}", fpath)).unwrap()
    }

    #[rstest]
    #[tokio::test]
    async fn info_returns_expected_result(client: Client, server: MockServer) {
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/zeroconf/info")
                .body("{\"data\":{}}");
            then.status(200)
                .header("content-type", "application/json; charset=utf-8")
                .body(load_fixture("info_ok.json"));
        });

        let got = client.info().await;

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
}
