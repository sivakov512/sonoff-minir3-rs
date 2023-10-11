use crate::models::*;

pub struct Client {
    host: String,
    port: u16,
    inner: reqwest::Client,
}

/// An aynchronous client for Sonoff mini R3 API
///
/// For more details look at the official docs:
/// https://sonoff.tech/sonoff-diy-developer-documentation-minir3-http-api/
impl Client {
    /// Constructs a new `Client` with given host and port
    ///
    /// # Example
    ///
    /// ```
    /// # use sonoff_minir3::Client;
    ///
    /// let client = Client::new("192.168.1.75", 8081);
    /// ```
    pub fn new<H: Into<String>>(host: H, port: u16) -> Self {
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

    /// Fetch device info.
    ///
    /// In current implementation it always uses `/zeroconf/info` API and returns limited info. For
    /// more details take a look at `Info` struct.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let got = client.fetch_info().await;
    ///
    /// assert!(got.is_ok());
    /// assert_eq!(
    ///     got.unwrap(),
    ///     Info {
    ///         switch: SwitchPosition::Off,
    ///         startup: StartupPosition::Off
    ///     }
    /// )
    /// ```
    pub async fn fetch_info(&self) -> anyhow::Result<Info> {
        Ok(self
            .inner
            .post(self.url("info"))
            .body("{\"data\":{}}")
            .send()
            .await?
            .json::<InfoResponse>()
            .await?
            .try_into()?)
    }

    /// Set startup position for device.
    ///
    /// It uses `/zeroconf/startups` API and always sets given position only for outlet 0.
    /// Other outlets will be set to `off` on every call, because API doesn't allow to specify
    /// state for one outlet only.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let got = client.set_startup_position(StartupPosition::Stay).await;
    ///
    /// assert!(got.is_ok());
    /// ```
    pub async fn set_startup_position(&self, position: StartupPosition) -> anyhow::Result<()> {
        Ok(self
            .inner
            .post(self.url("startups"))
            .json(&StartupsRequest::from(position))
            .send()
            .await?
            .json::<EmptyResponse>()
            .await?
            .try_into()?)
    }

    /// Set switch position.
    ///
    /// Is uses `/zeroconf/switches` API and always sets given position for outlet 0 only. This API
    /// allows to ignore state of another outlets, so they will be ignored.
    ///
    /// # Example
    /// ```ignore
    /// let got = client.set_switch_position(SwitchPosition::On).await;
    ///
    /// assert!(got.is_err());
    /// assert_eq!(
    ///     got.unwrap_err().downcast::<Error>().unwrap(),
    ///     Error::WrongParameters
    /// )
    /// ```
    pub async fn set_switch_position(&self, position: SwitchPosition) -> anyhow::Result<()> {
        Ok(self
            .inner
            .post(self.url("switches"))
            .json(&SwitchesRequest::from(position))
            .send()
            .await?
            .json::<EmptyResponse>()
            .await?
            .try_into()?)
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

    mod info {
        use super::*;

        #[tokio::test]
        async fn returns_expected_result() {
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
        async fn errored_in_expected_way() {
            let (server, client) = make_server_and_client();
            let mock = server.mock(|when, then| {
                when.method("POST")
                    .path("/zeroconf/info")
                    .body("{\"data\":{}}");
                then.status(400)
                    .header("content-type", "application/json; charset=utf-8")
                    .body(load_fixture("response_error.json"));
            });

            let got = client.fetch_info().await;

            mock.assert();

            assert!(got.is_err());
            assert_eq!(
                got.unwrap_err().downcast::<Error>().unwrap(),
                Error::WrongParameters
            )
        }
    }

    mod set_startup_position {
        use super::*;

        #[tokio::test]
        async fn sent_expected_request() {
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

        #[tokio::test]
        async fn errored_in_expected_way() {
            let (server, client) = make_server_and_client();
            let mock = server.mock(|when, then| {
                when.method("POST")
                    .path("/zeroconf/startups")
                    .body(load_fixture("request_startups_ok.json"));
                then.status(400)
                    .header("content-type", "application/json; charset=utf-8")
                    .body(load_fixture("response_error.json"));
            });

            let got = client.set_startup_position(StartupPosition::Stay).await;

            mock.assert();

            assert!(got.is_err());
            assert_eq!(
                got.unwrap_err().downcast::<Error>().unwrap(),
                Error::WrongParameters
            )
        }
    }

    mod set_switch_position {
        use super::*;

        #[tokio::test]
        async fn sent_expected_request() {
            let (server, client) = make_server_and_client();
            let mock = server.mock(|when, then| {
                when.method("POST")
                    .path("/zeroconf/switches")
                    .body(load_fixture("request_switches_ok.json"));
                then.status(400)
                    .header("content-type", "application/json; charset=utf-8")
                    .body(load_fixture("response_error.json"));
            });

            let got = client.set_switch_position(SwitchPosition::On).await;

            mock.assert();

            assert!(got.is_err());
            assert_eq!(
                got.unwrap_err().downcast::<Error>().unwrap(),
                Error::WrongParameters
            )
        }
    }
}
