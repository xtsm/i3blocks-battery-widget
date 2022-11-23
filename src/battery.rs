use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum BatteryStatus {
    Unknown,
    Charging,
    Discharging,
    NotCharging,
    Full,
}

impl FromStr for BatteryStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unknown" => Ok(Self::Unknown),
            "Charging" => Ok(Self::Charging),
            "Discharging" => Ok(Self::Discharging),
            "Not charging" => Ok(Self::NotCharging),
            "Full" => Ok(Self::Full),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct BatteryInfo {
    pub name: String,
    pub status: BatteryStatus,
    pub power_now: u64,
    pub energy_now: u64,
    pub energy_full: u64,
}

impl BatteryInfo {
    pub fn hours_to_status_change(&self) -> Result<Option<f64>, Error> {
        if self.power_now == 0 {
            Ok(None)
        } else {
            match self.status {
                BatteryStatus::Charging => Ok(Some(
                    (self.energy_full - self.energy_now) as f64 / self.power_now as f64,
                )),
                BatteryStatus::Discharging => {
                    Ok(Some(self.energy_now as f64 / self.power_now as f64))
                }
                BatteryStatus::Full => Ok(None),
                s => Err(Error::InvalidStatusForTimeEstimation(s)),
            }
        }
    }

    pub fn capacity(&self) -> f64 {
        self.energy_now as f64 / self.energy_full as f64
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidBatteryName(OsString),
    ParseFromFileFailed(PathBuf, String),
    IOError(PathBuf, io::Error),
    InvalidStatusForTimeEstimation(BatteryStatus),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::InvalidBatteryName(name) => write!(f, "invalid battery name: {name:?}"),
            Error::ParseFromFileFailed(path, value) => {
                write!(f, "at {path:?}: invalid value: {value}")
            }
            Error::IOError(path, err) => write!(f, "at {path:?}: {err}"),
            Error::InvalidStatusForTimeEstimation(status) => {
                write!(f, "cannot estimate time with status {status:?}")
            }
        }
    }
}

fn parse_from_file<T: FromStr>(path: &Path) -> Result<T, Error> {
    let s = fs::read_to_string(path)
        .map_err(|e| Error::IOError(path.to_path_buf(), e))?
        .trim_end()
        .to_string();
    s.parse()
        .map_err(|_| Error::ParseFromFileFailed(path.to_path_buf(), s))
}

impl std::error::Error for Error {}

pub fn read_battery_infos() -> Result<Vec<BatteryInfo>, Error> {
    const POWER_SUPPLY_PATH: &str = "/sys/class/power_supply";

    fs::read_dir(&POWER_SUPPLY_PATH)
        .map_err(|e| Error::IOError(PathBuf::from(POWER_SUPPLY_PATH), e))?
        .map(|entry| -> Result<Option<BatteryInfo>, Error> {
            let entry = entry.map_err(|e| Error::IOError(PathBuf::from(POWER_SUPPLY_PATH), e))?;

            let type_path = entry.path().join("type");
            if fs::read_to_string(&type_path)
                .map_err(|e| Error::IOError(type_path, e))?
                .trim_end()
                != "Battery"
            {
                return Ok(None);
            }

            Ok(Some(BatteryInfo {
                name: entry
                    .file_name()
                    .into_string()
                    .map_err(Error::InvalidBatteryName)?,
                status: parse_from_file(&entry.path().join("status"))?,
                power_now: parse_from_file(&entry.path().join("power_now"))?,
                energy_now: parse_from_file(&entry.path().join("energy_now"))?,
                energy_full: parse_from_file(&entry.path().join("energy_full"))?,
            }))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|infos| infos.into_iter().filter_map(|x| x).collect())
}
