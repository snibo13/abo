use sled::Db;
use eframe::egui;
use eframe::egui::style;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::collections::BTreeMap;

use epaint::text::FontInsert;

mod patient;
use patient::{PatientRecord, GUIForm};

mod custom_components;
use custom_components::large_button;

mod medicine;
use medicine::{Medication, GUIForm as MedicationForm};

#[derive(PartialEq)]
enum Screens  {
    Main,
    PatientForm,
    Patients,
    MedicationForm,
}


struct App {
    patient: PatientRecord,
    medicine: Medication,
    patient_provided: bool, // Indicates if a patient record is provided for editing
    screen: Screens,
    db: Db,
}

fn render_medication_form(ui: &mut egui::Ui, app: &mut App) {
    ui.heading(egui::RichText::new("Medication Form").size(64.0).color(egui::Color32::WHITE));
    app.medicine.show_form(ui);
    if ui.button("Save").clicked() {
        // Create a medication id
        let medication_id = format!("medication_{}", chrono::Utc::now().timestamp_millis());
        let medication = Medication {
            name: "Sample Medication".to_string(),
            medication_string: "Sample String".to_string(),
            supplier: "Sample Supplier".to_string(),
            cost_per_pill: 0.0,
        };
        let serialized = serde_json::to_string(&medication).expect("Failed to serialize medication");
        app.db.insert(medication_id, serialized.as_bytes())
            .expect("Failed to save medication record");
        app.db.flush().expect("Failed to flush database");
        println!("Saved medication: {:?}", medication);
        app.medicine = Medication::default(); // Reset the form after saving
    }
    if ui.button("Save and Return").clicked() {
        // Create a medication id
        let medication_id = format!("medication_{}", chrono::Utc::now().timestamp_millis());
        let medication = Medication {
            name: "Sample Medication".to_string(),
            medication_string: "Sample String".to_string(),
            supplier: "Sample Supplier".to_string(),
            cost_per_pill: 0.0,
        };
        let serialized = serde_json::to_string(&medication).expect("Failed to serialize medication");
        app.db.insert(medication_id, serialized.as_bytes())
            .expect("Failed to save medication record");
        app.db.flush().expect("Failed to flush database");
        println!("Saved medication: {:?}", medication);
        app.medicine = Medication::default(); // Reset the form after saving
        app.screen = Screens::Main; // Go back to main screen
    }
    if ui.button("Back to Main").clicked() {
        app.screen = Screens::Main;
        app.medicine = Medication::default(); // Reset the form when going back to main
    }
}


fn render_patient_form(ui: &mut egui::Ui, app: &mut App) {
    ui.heading(egui::RichText::new("Patient Record Form").size(64.0).color(egui::Color32::WHITE));
    app.patient.show_form(ui);
    
    if ui.button("Delete").clicked() {
        // Delete the patient record if it exists
        if !app.patient.patient_id.is_empty() {
            app.db.remove(app.patient.patient_id.clone())
                .expect("Failed to delete patient record");
            app.db.flush().expect("Failed to flush database");
            println!("Deleted patient: {:?}", app.patient);
            app.patient = PatientRecord::default(); // Reset the form after deletion
            app.patient_provided = false; // Reset the flag after deletion
            app.screen = Screens::Main;
            app.patient_provided = false; // Reset the flag when going back to main
        }
    }

    if ui.button("Save").clicked() {
        // Create a patient id
        if app.patient.patient_id.is_empty() {
            // Generate a unique patient ID, e.g., using a timestamp or UUID
            app.patient.patient_id = format!("patient_{}", chrono::Utc::now().timestamp_millis());
        }
        let serialized = serde_json::to_string(&app.patient).expect("Failed to serialize patient record");
        app.db.insert(app.patient.patient_id.clone(), serialized.as_bytes())
            .expect("Failed to save patient record");
        app.db.flush().expect("Failed to flush database");
        // Log the saved patient record
        eprintln!("Saved patient: {:?}", app.patient);
       

        println!("Saved patient: {:?}", app.patient);
        app.patient = PatientRecord::default(); // Reset the form after saving
        app.patient_provided = false; // Reset the flag after saving
    }
    if ui.button("Save and Return").clicked() {
        // Create a patient id
        if app.patient.patient_id.is_empty() {
            // Generate a unique patient ID, e.g., using a timestamp or UUID
            app.patient.patient_id = format!("patient_{}", chrono::Utc::now().timestamp_millis());
        }
        let serialized = serde_json::to_string(&app.patient).expect("Failed to serialize patient record");
        app.db.insert(app.patient.patient_id.clone(), serialized.as_bytes())
            .expect("Failed to save patient record");
        app.db.flush().expect("Failed to flush database");
        // Log the saved patient record
        eprintln!("Saved patient: {:?}", app.patient);
       

        println!("Saved patient: {:?}", app.patient);
        app.patient = PatientRecord::default(); // Reset the form after saving
        app.patient_provided = false; // Reset the flag after saving
        app.screen = Screens::Main;
        app.patient_provided = false; // Reset the flag when going back to main
    }
    if ui.button("Back to Main").clicked() {
        app.screen = Screens::Main;
        app.patient = PatientRecord::default(); // Reset the form when going back to main
        app.patient_provided = false; // Reset the flag when going back to main
    }
}

fn render_main_screen(ui: &mut egui::Ui, app: &mut App) {
    use egui::*;

    // Fill the entire screen
    CentralPanel::default().show(ui.ctx(), |ui| {
        // Center everything vertically and horizontally
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Welcome to the Patient Management System");
                ui.add_space(20.0);

                Frame::none()
                    .inner_margin(Margin::same(20.0)) // Optional: add padding around
                    .show(ui, |ui| {
                        Grid::new("main_menu")
                            .num_columns(2)
                            .spacing([40.0, 20.0]) // Wider spacing between buttons
                            .show(ui, |ui| {
                                if ui.add(large_button("Add Patient Form")).clicked() {
                                    app.screen = Screens::PatientForm;
                                }
                                if ui.add(large_button("View Patients")).clicked() {
                                    app.screen = Screens::Patients;
                                }
                                ui.end_row();
                                if ui.add(large_button("Add Medication Form")).clicked() {
                                    app.screen = Screens::MedicationForm;
                                }
                                if ui.add(large_button("Exit")).clicked() {
                                    std::process::exit(0);
                                }
                            });
                    });
            });
        });
    });
}


fn render_patient_list(ui: &mut egui::Ui, app: &mut App) {
    let mut patient_records: Vec<PatientRecord>;
    let mut search_term: String = String::new();
    
    ui.heading("Patient List");
    ui.add(egui::Label::new("Click on a patient ID to view details."));
    ui.add(egui::text_edit::TextEdit::singleline(&mut search_term).hint_text("Search by Patient ID or Name"));
    
    patient_records = app.db
        .iter()
        .values()
        .filter_map(|value| value.ok())
        .filter_map(|value| serde_json::from_slice::<PatientRecord>(&value).ok())
        .filter(|patient| {
            patient.patient_id.to_lowercase().contains(&search_term.to_lowercase()) ||
            patient.patient_name.to_lowercase().contains(&search_term.to_lowercase())
        })
        .collect();
    

    patient_records.sort_by(|a, b| a.patient_name.cmp(&b.patient_name));
    for patient in patient_records {
        let patient_id = patient.patient_id.clone();
        let patient_name = patient.patient_name.clone();
        let dob = patient.dob.format("%Y-%m-%d").to_string();

        if ui.button(format!("{} - {} (DOB: {})", patient_id, patient_name, dob)).clicked() {
            // When a patient is clicked, you can show their details or perform an action
            app.patient = patient; // Load the selected patient's data into the form
            app.patient_provided = true;
            app.screen = Screens::PatientForm; // Switch to the Patient Form screen
        }
    }
    if ui.button("Back to Main").clicked() {
        app.screen = Screens::Main;
    }
}

impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.screen == Screens::Main {
                render_main_screen(ui, self);
            } else if self.screen == Screens::PatientForm {
                render_patient_form(ui, self);
            } else if self.screen == Screens::Patients {
                render_patient_list(ui, self);
            } else if self.screen == Screens::MedicationForm {
                render_medication_form(ui, self);
            }

            ctx.debug_on_hover(); 
        });
    }
}

fn main() {
    // Initialize the sled database
    // let db: Db = sled::open("patient_db").expect("Failed to open database");
    // Ensure the database is empty at startup
    // db.clear().expect("Failed to clear database");
    // Set up the eframe application

    let mut native_options = eframe::NativeOptions::default();
    // Start full screen
    native_options.viewport = egui::ViewportBuilder::default().with_maximized(true);
    eframe::run_native(
        "Patient Form",
        native_options,
        Box::new(|cc| {
            // Customize fonts
            customize_fonts(&cc.egui_ctx);
            // Set the style
            customize_style(&cc.egui_ctx);

            

            
            Box::new(App {
                patient: PatientRecord::default(),
                patient_provided: false,
                screen: Screens::Main,
                medicine: Medication::default(),
                db: sled::open("patient_db").expect("Failed to open database"),
            })
        }),
    ).unwrap();
}

fn customize_fonts(ctx: &egui::Context) {
    use egui::{FontDefinitions, FontFamily};

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Merriweather".to_owned(),
        egui::FontData::from_static(include_bytes!("./fonts/MerriweatherSans-VariableFont_wght.ttf")),
    );
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
    .insert(0, "Merriweather".to_owned());
    ctx.set_fonts(fonts);
}

fn customize_style(ctx: &egui::Context) {
   // Redefine text_styles
let text_styles: BTreeMap<_, _> = [
    (Heading, FontId::new(30.0, Proportional)),
    (Name("Heading2".into()), FontId::new(25.0, Proportional)),
    (Name("Context".into()), FontId::new(23.0, Proportional)),
    (Body, FontId::new(36.0, Proportional)),
    (Monospace, FontId::new(24.0, Proportional)),
    (Button, FontId::new(24.0, Proportional)),
    (Small, FontId::new(18.0, Proportional)),
  ].into();
  
  // Mutate global styles with new text styles
  ctx.style_mut(move |style| style.text_styles = text_styles.clone());
}
