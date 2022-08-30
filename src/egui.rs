use eframe::egui::{self}; //Imports the rendering engine
use eframe::egui::Visuals; //Imports dark mode

mod api_handler; //Imports the API handler
use api_handler::*;

pub struct MyApp {
    //Enter global values to be used with your app here
    insult: String,
}

impl Default for MyApp {
    //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            insult: "".to_string(),
        }
    }
}

//A function that makes a get request from https://insult.mattbas.org/api/insult and returns it as a string

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
            ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark
            egui::warn_if_debug_build(ui);

            self.insult = self.insult.replace("\n", ""); // Removes newlines from the string

            ui.label("Hello and welcome to version 4 of the bully luna program!");
            ui.label("You can randomly generate an insult or write your own below to be sent to luna!");

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Enter an insult: ");
                ui.text_edit_multiline(&mut self.insult);
            });

            let generate_button = ui.button("Generate an insult");
            if generate_button.clicked() {
                self.insult = get_insult();
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Your insult:");
                ui.label(&self.insult);
            });

            let send_button = ui.button("Send insult to luna");
            //if send_button.hovered() {
            //    ui.label("This is still a work in progress!");
            //}

            let popup_id = ui.make_persistent_id("send_button_popup");
            if send_button.clicked() {
                ui.memory().toggle_popup(popup_id);
            }

            egui::popup::popup_below_widget(ui, popup_id, &send_button, |ui| {
                //The contents of the popup go here
                ui.set_min_width(350.0); // if you want to control the size
                ui.label(format!("{}", self.insult));
                ui.separator();
                ui.label("Has been successfully sent to luna!\nThank you for choosing bully luna v.4!");
            }); 
        });
    }
}
