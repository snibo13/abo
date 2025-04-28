use serde::{Serialize, Deserialize};
use gui_macro::GUIForm;
use std::default::Default;
use chrono::{NaiveDate, Datelike};

pub trait GUIForm {
    fn show_form(&mut self, ui: &mut egui::Ui);
}



#[derive(Serialize, Deserialize, Debug, GUIForm, Clone, Default)]
pub struct PatientRecord {
    pub patient_name: String,
    pub patient_id: String,
    pub address: String,
    pub city: String,
    pub phone_number: String,
    pub dob: NaiveDate, // Date of Birth
    pub pregnant: bool, // 0: No, 1: Yes
    pub weight: f32,
    pub height: f32,
    // pub allergies: Vec<String>,
}
