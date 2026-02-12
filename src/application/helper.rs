use egui::{Ui, Popup, InputState, PopupCloseBehavior};
use log::{info};
use super::marsapp::Id;

pub(super) trait StickyPopupExt {
	fn create_popup<R>(&self, ui : &mut Ui, content: impl FnOnce(&mut Ui) -> R, id: Id);
}

impl StickyPopupExt for egui::Response {
	// FIXME: The popup goes away no matter where the click is. 
	fn create_popup<R>(&self, ui : &mut Ui, content: impl FnOnce(&mut Ui) -> R, id: Id) {
		let popup_id = self.id.with(id);
		let anchor = egui::PopupAnchor::ParentRect(self.rect);
		let popup = Popup::new(popup_id, ui.ctx().clone(), anchor, self.layer_id);

		let is_popup_open = Popup::<'_>::is_id_open(ui.ctx(), popup_id);
		let is_pointing_at_button = self.contains_pointer();

		// Open (or keep open) the popup.
		let popup_response = if is_pointing_at_button || is_popup_open {
				popup.open_memory(egui::SetOpenCommand::Bool(true))
				.close_behavior(PopupCloseBehavior::CloseOnClickOutside)
				.show(content)
		} else {
			None
		};

		// Close if we're not interacting with the button or child
		if let Some(response) = popup_response {
			let is_pointing_at_popup = response.response.contains_pointer();
			let click_registered = ui.input(|input_reader : &InputState| input_reader.pointer.any_click());

			if click_registered && is_pointing_at_popup {
				info!("Clicked le popup!");

			}
			else if !is_pointing_at_button && !is_pointing_at_popup {
				info!("{:?}", (is_pointing_at_button, !is_pointing_at_popup, !click_registered));
				//Popup::<'_>::close_id(ui.ctx(), popup_id);
			}
		}
	} // fn create_popup
}
