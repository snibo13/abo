use serde::{Serialize, Deserialize};
use gui_macro::GUIForm;
use std::default::Default;
use chrono::{NaiveDate, Datelike};

pub trait GUIForm {
    fn show_form(&mut self, ui: &mut egui::Ui);
}



#[derive(Serialize, Deserialize, Debug, GUIForm, Clone, Default)]
pub struct Medication {
    pub name: String,
    pub medication_string: String,
    pub supplier: String,
    pub cost_per_pill: f32,
}
