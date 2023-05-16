use psutil::sensors::temperatures;

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
}