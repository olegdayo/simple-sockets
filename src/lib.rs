use serde::{Deserialize, Serialize};

pub const BUFFER_SIZE: usize = 1000;

#[derive(Debug, Serialize, Deserialize)]
pub struct Props {
    pub height: f64,
    pub weight: f64,
}

impl Props {
    pub fn new(height: f64, weight: f64) -> Props {
        Props { height, weight }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BMI {
    value: f64,
    class: String,
}

impl BMI {
    fn get_bmi_value(props: &Props) -> f64 {
        props.weight / (props.height * props.height)
    }

    pub fn get_bmi_class(bmi: f64) -> String {
        if bmi > 30f64 {
            "overweight".to_string()
        } else if bmi > 18.5 {
            "normal".to_string()
        } else {
            "underweight".to_string()
        }
    }

    pub fn get_bmi(props: &Props) -> BMI {
        let value = BMI::get_bmi_value(&props);
        let class = BMI::get_bmi_class(value);
        BMI { value, class }
    }
}
