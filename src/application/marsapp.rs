use eframe::{App, Frame};
use egui::{Context, Id, Label, LayerId, Ui, Widget};
use core::default::Default;
use eframe::epaint::Color32;
use crate::application::helper::StickyPopupExt;

pub(crate) struct MarsApp
{
	code_buffer : String,
}

impl MarsApp
{
	pub fn new() -> Self
	{
		Self {
			code_buffer: String::new(),
		}
	}

	fn create_toolbar(&mut self, ctx: &Context, _frame: &mut Frame) {
		egui::TopBottomPanel::top("top_panel")
			.show(ctx, |ui: &mut Ui| {
				egui::MenuBar::new().ui(ui, |ui: &mut Ui| {
					Self::create_file_section(ui);
					Self::create_edit_section(ui);
					Self::create_run_section(ui);
					Self::create_settings_section(ui);
					Self::create_tools_section(ui);
					Self::create_help_section(ui);
				})
			});
	} // create_toolbar

	fn create_file_section(ui : &mut Ui) {
		let file_button = ui.button("File");
		file_button.create_popup(ui, |ui : &mut Ui| {
			ui.label("New");
			ui.label("Open...");
			ui.label("Close");
			ui.label("Close All");
			// TODO: investigate putting a line here.
			ui.label("Save");
			ui.label("Save As...");
			ui.label("Save All");
			ui.label("Dump Memory...");
			// TODO: Another line here
			ui.label("Print...");
			// TODO: Investigate if this applies to web...
			ui.label("Exit");
		})
	} // create_file_section

	fn create_edit_section(ui : &mut Ui) {
		ui.button("Edit").create_popup(ui, |ui : &mut Ui| {
			ui.label("Undo");
			ui.label("Redo");
			// TODO: Put a line here
			ui.label("Cut");
			ui.label("Copy");
			ui.label("Paste");
			// TODO: Put a line here
			ui.label("Find/Replace");
			ui.label("Select All");
		});
	} // create_edit_section

	fn create_run_section(ui : &mut Ui) {
		ui.button("Run").create_popup(ui, |ui : &mut Ui| {
			ui.label("Assemble");
			ui.label("Go");
			ui.label("Step");
			ui.label("Backstep");
			ui.label("Pause");
			ui.label("Stop");
			ui.label("Reset");
			// TODO: Put a line break here
			ui.label("Clear all breakpoints");
			ui.label("Toggle all breakpoints");
		})
	} // create_run_section

	fn create_settings_section(ui : &mut Ui) {
		ui.button("Settings").create_popup(ui, |ui : &mut Ui| {
			let mut boolean : bool = false;
			
			ui.checkbox(&mut boolean, "Show Labels Window (symbol table)");
			ui.checkbox(&mut boolean, "Program arguments provided to wasm program");
			ui.checkbox(&mut boolean, "Popup dialogue for input syscalls"); // CLARIFY: Does wasm have syscalls?
			ui.checkbox(&mut boolean, "Addresses displayed in hexadecimal");
			ui.checkbox(&mut boolean, "Values displayed in hexadecimal");
			// TODO: Line
			ui.checkbox(&mut boolean, "Assemble file upon opening");
			ui.checkbox(&mut boolean, "Assemble all files in directory");
			ui.checkbox(&mut boolean, "Assembler warnings are considered errors");
			ui.checkbox(&mut boolean, "Initialize Program Counter to global 'main' if initialized");
			// Clarify: I don't think any of the last three options make sense here.
			ui.label("Editor...");
			ui.label("Highlighting...");
			ui.label("Exception Handler...");
			ui.label("Memory Configuration...");
		})
	} // create_settings_section

	fn create_tools_section(ui : &mut Ui) {
		let this_button = ui.button("Tools");
		this_button.create_popup(ui, |ui : &mut Ui| {
			ui.label("Nothing here yet!")
		})
	} // create_tools_section

	fn create_help_section(ui : &mut Ui) {
		ui.button("Help").create_popup(ui, |ui : &mut Ui| {
			ui.label("Help...");
			ui.label("About...");
		})
	} // create_help_section
	
} // impl MarsApp
impl App for MarsApp
{
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		self.create_toolbar(ctx, _frame);

		let frame = egui::containers::Frame::default().fill(Color32::WHITE);


		// Central content
		egui::CentralPanel::default().frame(frame).show(ctx, |ui : &mut Ui| {
			// Assembly editor
			egui::ScrollArea::vertical().show(ui, |ui : &mut Ui| {
				let available_size = ui.available_size();
				ui.add_sized(available_size, |ui : &mut Ui|{
					ui.text_edit_multiline(&mut self.code_buffer)
				});
			}); // Assembly editor
		}); // Central content

	} // fn update
} // impl App
