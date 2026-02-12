use eframe::{App, Frame};
use egui::{Context, Ui};
use core::default::Default;
use std::io::{Read};
use eframe::epaint::Color32;
use serde::{Deserialize, Serialize};
use crate::application::helper::StickyPopupExt;

#[repr(u64)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum Id {
	File,
	Edit,
	Run,
	Settings,
	Tools,
	Help,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MarsApp
{
	#[serde(skip)]
	code_buffer : String,
	show_symbol_table : bool,
	allow_program_args : bool,
	allow_syscall_interrupts : bool,
	hexadecimal_addresses : bool,
	hexadecimal_values : bool,
	assemble_on_open : bool,
	assemble_full_dir : bool,
	warn_as_err : bool,
	init_pc : bool,
}

impl MarsApp
{
	#[allow(dead_code)]
	pub fn new() -> Self
	{
		Self {
			code_buffer: String::new(),
			show_symbol_table: false,
			allow_program_args: false,
			allow_syscall_interrupts: false,
			hexadecimal_addresses: false,
			hexadecimal_values: false,
			assemble_on_open: false,
			assemble_full_dir: false,
			warn_as_err: false,
			init_pc: false,
		}
	}

	fn create_toolbar(&mut self, ctx: &Context, _frame: &mut Frame) {
		egui::TopBottomPanel::top("top_panel")
			.show(ctx, |ui: &mut Ui| {
				egui::MenuBar::new().ui(ui, |ui: &mut Ui| {
					Self::create_file_section(ui);
					Self::create_edit_section(ui);
					Self::create_run_section(ui);
					self.create_settings_section(ui);
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
			ui.separator();
			ui.label("Save");
			ui.label("Save As...");
			ui.label("Save All");
			ui.label("Dump Memory...");
			ui.separator();
			ui.label("Print...");
			ui.separator();
			ui.label("Exit");
		},
		Id::File,
		)
	} // create_file_section

	fn create_edit_section(ui : &mut Ui) {
		ui.button("Edit").create_popup(ui, |ui : &mut Ui| {
			ui.label("Undo");
			ui.label("Redo");
			ui.separator();
			ui.label("Cut");
			ui.label("Copy");
			ui.label("Paste");
			ui.separator();
			ui.label("Find/Replace");
			ui.label("Select All");
		},
		Id::Edit,
		);
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
		ui.separator();
		ui.label("Clear all breakpoints");
		ui.label("Toggle all breakpoints");
		},
		Id::Run,
		)
	} // create_run_section

	fn create_settings_section(&mut self, ui : &mut Ui) {
		ui.button("Settings").create_popup(ui, |ui : &mut Ui| {
			ui.checkbox(&mut self.show_symbol_table, "Show Labels Window (symbol table)");
			ui.checkbox(&mut self.allow_program_args, "Program arguments provided to wasm program");
			ui.checkbox(&mut self.allow_syscall_interrupts, "Popup dialogue for input syscalls"); // CLARIFY: Does wasm have syscalls?
			ui.checkbox(&mut self.hexadecimal_addresses, "Addresses displayed in hexadecimal");
			ui.checkbox(&mut self.hexadecimal_values, "Values displayed in hexadecimal");
			ui.separator();
			ui.checkbox(&mut self.assemble_on_open, "Assemble file upon opening");
			ui.checkbox(&mut self.assemble_full_dir, "Assemble all files in directory");
			ui.checkbox(&mut self.warn_as_err, "Assembler warnings are considered errors");
			ui.checkbox(&mut self.init_pc, "Initialize Program Counter to global 'main' if initialized");
			ui.separator();
			// Clarify: I don't think any of the last three options make sense here.
			ui.label("Editor...");
			ui.label("Highlighting...");
			ui.label("Exception Handler...");
			ui.label("Memory Configuration...");
		},
		Id::Settings,
		)
	} // create_settings_section

	fn create_tools_section(ui : &mut Ui) {
		let this_button = ui.button("Tools");
		this_button.create_popup(ui, |ui : &mut Ui| {
			ui.label("Nothing here yet!")
		},
		Id::Tools,
		)
	} // create_tools_section

	fn create_help_section(ui : &mut Ui) {
		ui.button("Help").create_popup(ui, |ui : &mut Ui| {
			ui.label("Help...");
			ui.label("About...");
		},
		Id::Help
		)
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

impl Read for MarsApp {
	/// Unexpected behavior: This does not report the number of bytes written.
	/// The value defaults to 0.
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		serde_json::to_writer(buf, self)?;
		Ok(0)
	}
}
