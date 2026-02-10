use eframe::{App, Frame};
use egui::{Context, Id, LayerId, Ui};
use core::default::Default;
use eframe::epaint::Color32;

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
					ui.button("File").context_menu(|ui: &mut Ui| {});
					ui.button("Edit");
					ui.button("Run");
					ui.button("Settings");
					Self::create_tools_section(ui)
				})
			});
	} // create_toolbar
	
	fn create_tools_section(ui : &mut Ui) {
		let this_button = ui.button("Tools");
		let popup_id = ui.next_auto_id();
		if this_button.contains_pointer() || ui.memory(|mem| mem.is_popup_open(popup_id)) {
			let anchor = egui::PopupAnchor::ParentRect(this_button.rect);
			egui::Popup::new(popup_id, ui.ctx().clone(), anchor, this_button.layer_id)
				.open_memory(egui::SetOpenCommand::Bool(true))
				.show(|ui : &mut Ui| {
					ui.button("Item 1");
					ui.button("Item 2");
					ui.button("Item 3");
				}) ;
		}
	}
	
	
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
