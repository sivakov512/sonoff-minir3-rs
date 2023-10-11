use serde::{Deserialize, Serialize};
use std::fmt;

const OUTLET2USE: u8 = 0;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    WrongParameters,
}

impl Error {
    fn from_api_error_code(code: usize) -> Self {
        match code {
            400 => Self::WrongParameters,
            _ => panic!("Unexpected api error"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Error::WrongParameters => "API errored with code 400, wrong parameters",
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for Error {}

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

impl TryFrom<InfoResponse> for Info {
    type Error = Error;

    fn try_from(value: InfoResponse) -> Result<Self, Self::Error> {
        match value.error {
            0 => {
                let data = value.data.unwrap();
                Ok(Self {
                    switch: data
                        .switches
                        .into_iter()
                        .find(|s| s.outlet == OUTLET2USE)
                        .unwrap()
                        .switch,
                    startup: data
                        .configure
                        .into_iter()
                        .find(|s| s.outlet == OUTLET2USE)
                        .unwrap()
                        .startup,
                })
            }
            v => Err(Error::from_api_error_code(v)),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct InfoResponse {
    data: Option<InfoData>,
    error: usize,
}

#[derive(Deserialize)]
struct InfoData {
    switches: Vec<Switch>,
    configure: Vec<Startup>,
}

#[derive(Deserialize, Serialize)]
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
            outlet: OUTLET2USE,
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

impl From<SwitchPosition> for SwitchesRequest {
    fn from(value: SwitchPosition) -> Self {
        SwitchesRequest {
            data: SwitchesData {
                switches: vec![Switch {
                    switch: value,
                    outlet: OUTLET2USE,
                }],
            },
        }
    }
}

#[derive(Serialize)]
pub(crate) struct SwitchesRequest {
    data: SwitchesData,
}

#[derive(Serialize)]
struct SwitchesData {
    switches: Vec<Switch>,
}

impl TryFrom<EmptyResponse> for () {
    type Error = Error;

    fn try_from(value: EmptyResponse) -> Result<Self, Self::Error> {
        match value.error {
            0 => Ok(()),
            v => Err(Error::from_api_error_code(v)),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct EmptyResponse {
    error: usize,
}
