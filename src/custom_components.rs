pub fn large_button(label: &str) -> egui::Button {
    egui::Button::new(
        egui::RichText::new(label).size(36.0).color(egui::Color32::WHITE)
    ).min_size(egui::vec2(750.0, 125.0))
    .wrap(true)
}