use serde_json::{json, Value};

fn read_field_f64(data: &Value, field: &str) -> Option<f64> {
    let wo = data["<".to_string() + field].as_f64();
    let rw = data["<>".to_string() + field].as_f64();
    wo.or(rw)
}

fn read_field_u64(data: &Value, field: &str) -> Option<u64> {
    let wo = data["<".to_string() + field].as_u64();
    let rw = data["<>".to_string() + field].as_u64();
    wo.or(rw)
}

pub fn parse_sim_to_robot(data: String) -> Option<String> {
    
    let json_data: Value = serde_json::from_str(&data).unwrap();
    if let Some("SimDevice") = json_data["type"].as_str() {
        let (device, mut num) = json_data["device"]
            .as_str()
            .unwrap()
            .split_once("[")
            .unwrap();
        num = num.trim_end_matches("]");

        let message = match device {
            "NoUMotor" => {
                if let Some(speed) = read_field_f64(&json_data["data"], "speed") {
                    format!("m{num}{speed}")
                } else {
                    return None;
                }
            }
            "NoUServo" => {
                if let Some(angle) = read_field_f64(&json_data["data"], "angle") {
                    format!("s{num}{angle}")
                } else {
                    return None;
                }
            }
            "NoUGPIO" => {
                if let Some(value) = read_field_u64(&json_data["data"], "value") {
                    format!("g{value}{num}")
                } else {
                    return None;
                }
            }
            "GPIOPrep" => {
                if let Some(mode) = read_field_u64(&json_data["data"], "mode") {
                    format!("p{mode}{num}")
                } else {
                    return None;
                }
            }
            _ => unreachable!(),
        };
        Some(message)
    } else {
        None
    }
}

pub fn parse_robot_to_sim(data: Vec<u8>) -> Option<String> {
    let mut result = json!({
        "data": {},
        "device": "",
        "type": "SimDevice"
    });

    let device_num = String::from_utf8_lossy(&data[3..]);

    match data[0] as char {
        'm' => unimplemented!(),
        's' => unimplemented!(),
        'g' => {
            *result.get_mut("device").unwrap() = json!(format!("NoUGPIO[{device_num}]"));
            let key = match data[1] as char {
                '0' => ">value".to_string(),
                '2' => "<>value".to_string(),
                _ => unreachable!(),
            };
            result["data"]
                .as_object_mut()
                .unwrap()
                .insert(key, json!(data[2] - '0' as u8));
        }
        'p' => unimplemented!(),
        _ => (),
    }
    serde_json::to_string(&result).ok()
}
