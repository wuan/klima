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

fn parse() -> Result<Config, &'static str> {
    if !Path::new("/etc/klimalogger.conf").exists() {
        return Result::Err("config file not found");
    }

    let config = Config {
        client_name: "adf",
        sensors: Vec::new(),
    };

    return Result::Ok(config);
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