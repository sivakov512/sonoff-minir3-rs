use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SwitchPosition {
    On,
    Off,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StartupPosition {
    On,
    Off,
    Stay,
}

#[derive(Debug, PartialEq)]
pub struct Info {
    pub switch: SwitchPosition,
    pub startup: StartupPosition,
}

impl From<InfoResponse> for Info {
    fn from(value: InfoResponse) -> Self {
        Self {
            switch: value
                .data
                .switches
                .into_iter()
                .find(|s| s.outlet == 0)
                .unwrap()
                .switch,
            startup: value
                .data
                .configure
                .into_iter()
                .find(|s| s.outlet == 0)
                .unwrap()
                .startup,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct InfoResponse {
    data: InfoData,
}

#[derive(Deserialize)]
struct InfoData {
    switches: Vec<Switch>,
    configure: Vec<Startup>,
}

#[derive(Deserialize)]
struct Switch {
    switch: SwitchPosition,
    outlet: u8,
}

#[derive(Deserialize, Serialize)]
struct Startup {
    startup: StartupPosition,
    outlet: u8,
}

impl From<StartupPosition> for StartupsRequest {
    fn from(value: StartupPosition) -> Self {
        let mut startups = vec![Startup {
            startup: value,
            outlet: 0,
        }];

        for i in 1..=3 {
            startups.push(Startup {
                startup: StartupPosition::Off,
                outlet: i,
            })
        }

        Self {
            data: StartupsData {
                configure: startups,
            },
        }
    }
}

#[derive(Serialize)]
pub(crate) struct StartupsRequest {
    data: StartupsData,
}

#[derive(Serialize)]
struct StartupsData {
    configure: Vec<Startup>,
}
