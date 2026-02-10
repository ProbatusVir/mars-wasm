use egui::{Ui, Popup};

pub(super) trait StickyPopupExt {
	fn create_popup<R>(&self, ui : &mut Ui, content: impl FnOnce(&mut Ui) -> R);
}

impl StickyPopupExt for egui::Response {
	fn create_popup<R>(&self, ui : &mut Ui, content: impl FnOnce(&mut Ui) -> R) {
		let popup_id = ui.next_auto_id();
		if self.contains_pointer() || ui.memory(|mem| mem.is_popup_open(popup_id)) {
			let anchor = egui::PopupAnchor::ParentRect(self.rect);
			 Popup::new(popup_id, ui.ctx().clone(), anchor, self.layer_id)
				.open_memory(egui::SetOpenCommand::Bool(true))
				.show(content);
		}
	} // fn create_popup
}
