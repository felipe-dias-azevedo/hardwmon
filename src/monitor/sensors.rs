use psutil::sensors::temperatures;
use crate::monitor::MonitorRow;

pub struct SensorData {
    name: String,
    label: Option<String>,
    temperature: f64,
    temperature_max: Option<f64>,
}

impl SensorData {
    pub fn new() -> Vec<SensorData> {
        let temperatures = temperatures();

        temperatures
            .into_iter()
            .filter_map(|t| t.ok())
            .map(|temp| {
                let unit = temp.unit();
                let temperature = temp.current().celsius().round();
                
                let temperature_max = match temp.high() {
                    Some(x) => Some(x.celsius().round()),
                    _ => None,
                };
                let label = match temp.label() {
                    Some(x) => Some(String::from(x)),
                    _ => None,
                };

                SensorData {
                    name: String::from(unit),
                    label,
                    temperature,
                    temperature_max,
                }
            })
            .collect()
    }

    pub fn format(sensors: Vec<SensorData>) -> MonitorRow {
        MonitorRow {
            title: String::from("SENSORS"),
            value: None,
            child: sensors.iter().map(|sensor| {
                MonitorRow {
                    title: sensor.name.to_owned(),
                    value: None,
                    child: vec![
                        MonitorRow {
                            title: format!("{} Temperature", sensor.label.as_ref().unwrap_or(&String::from(""))),
                            value: Some(format!("{:.0} ºC", sensor.temperature)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: format!("{} Temperature Max", sensor.label.as_ref().unwrap_or(&String::from(""))),
                            value: match sensor.temperature_max {
                                Some(x) => Some(format!("{:.0} ºC", x)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        // MonitorRow {
                        //     title: String::from("Label"),
                        //     value: match &sensor.label {
                        //         Some(x) => Some(x.to_owned()),
                        //         _ => Some(String::from("-"))
                        //     },
                        //     child: vec![]
                        // }
                    ]
                }
            }).collect()
        }
    }
}