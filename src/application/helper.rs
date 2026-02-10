use egui::{Ui, Popup};
use super::marsapp::Id;

pub(super) trait StickyPopupExt {
	fn create_popup<R>(&self, ui : &mut Ui, content: impl FnOnce(&mut Ui) -> R, id: Id);
}

impl StickyPopupExt for egui::Response {
	fn create_popup<R>(&self, ui : &mut Ui, content: impl FnOnce(&mut Ui) -> R, id: Id) {
		let popup_id = self.id.with(id);
		let anchor = egui::PopupAnchor::ParentRect(self.rect);
		let popup = Popup::new(popup_id, ui.ctx().clone(), anchor, self.layer_id);
		if self.contains_pointer() || Popup::<'_>::is_id_open(ui.ctx(), popup_id) {
				popup.open_memory(egui::SetOpenCommand::Bool(true))
				.show(content);
		}
	} // fn create_popup
}
