use psutil::sensors::temperatures;

pub struct SensorData {
    name: String,
    label: String,
    temperature: f64,
    temperature_max: f64,
}

pub fn get_sensors_data() {
    {
        println!("------ Sensors -----");
        let temperatures = temperatures();
        for t in temperatures {
            if let Some(temp) = t.ok() {
                let unit = temp.unit();
                let celsius = temp.current().celsius().round();

                println!("Unit: {}", unit);

                if let Some(max) = temp.high() {
                    let max = max.celsius().round();
                    println!("Temperature: {:.0} ºC (MAX: {:.0} ºC)", celsius, max);
                } else {
                    println!("Temperature: {:.0} ºC", celsius);
                }

                if let Some(label) = temp.label() {
                    println!("Label: {}", label);
                }
                println!();
            }
        }
    }
}