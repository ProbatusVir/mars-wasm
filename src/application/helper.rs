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
		if is_pointing_at_button || is_popup_open {
				popup.open_memory(egui::SetOpenCommand::Bool(true))
				.close_behavior(PopupCloseBehavior::CloseOnClickOutside)
				.show(content)
		} else {
			None
		};
	} // fn create_popup
}
