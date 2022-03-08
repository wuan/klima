use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

use configparser::ini::Ini;

struct Config {
    client_name: &'static str,
    sensors: Vec<Sensor>,
}

struct Sensor {
    sensor_type: SensorType,
}

enum SensorType {
    SHT1x
}

fn parse() -> Result<Config, Error> {
    let path = Path::new("/etc/klimalogger.conf");
    let mut file = File::open(&path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let config = parse_string(&s);

    return Result::Ok(config);
}

fn parse_string(source: &str) -> Config {
    let mut config = Ini::new();
    let configString = String::from(source);
    config.read(configString);

    let value = config.getint("somesection", "someintvalue").unwrap().unwrap();

    let config = Config {
        client_name: "adf",
        sensors: Vec::new(),
    };

    return config;
}

#[cfg(test)]
mod tests {
    use crate::config::parse;

    #[test]
    fn test_parse() -> Result<(), String> {
        let result = parse();

        assert!(result.is_err());
        Ok(())
    }
}