use crate::presets::_10_1021_jf903048p::MATURE_MILK;
use egui::{ComboBox, InnerResponse, Ui};
use lipid::prelude::*;
use polars::prelude::*;

/// Fatty acid widget
pub(crate) struct FattyAcidWidget {
    pub(crate) value: Option<FattyAcidChunked>,
    pub(crate) editable: bool,
    pub(crate) hover: bool,
}

impl FattyAcidWidget {
    pub(crate) fn new(value: Option<FattyAcidChunked>) -> Self {
        Self {
            value,
            editable: false,
            hover: false,
        }
    }

    pub(crate) fn editable(self, editable: bool) -> Self {
        Self { editable, ..self }
    }

    pub(crate) fn hover(self) -> Self {
        Self {
            hover: true,
            ..self
        }
    }

    #[instrument(skip(self, ui), err)]
    pub(crate) fn show(self, ui: &mut Ui) -> PolarsResult<InnerResponse<Option<FattyAcidChunked>>> {
        let fatty_acid = (self.value)()?;
        let text = match &fatty_acid {
            Some(fatty_acid) => &format!("{:#}", fatty_acid.display(Default::default())),
            None => "",
        };
        let mut inner = None;
        let mut response = if self.editable {
            let current_value = fatty_acid.unwrap_or_default();
            let response = ComboBox::from_id_salt(ui.next_auto_id())
                .width(ui.available_width())
                .selected_text(text)
                .show_ui(ui, |ui| -> PolarsResult<()> {
                    let mature_milk = MATURE_MILK.data.try_fatty_acid_list()?;
                    for selected_value in mature_milk.iter() {
                        let text = format!("{:#}", selected_value.display(Default::default()));
                        // if let Some(selected_value) = mature_milk.get(index) {
                        //     let text = format!("{:#}", selected_value.display(Default::default()));
                        //     // if ui
                        //     //     .selectable_value(current_value, *selected_value, text)
                        //     //     .changed()
                        //     // {
                        //     //     inner = Some(current_value.clone())
                        //     // }
                        // }
                    }
                    Ok(())
                });
            response.response
        } else {
            ui.label(text)
        };
        if self.hover {
            response = response.on_hover_text(text);
        }
        Ok(InnerResponse::new(inner, response))
        // Ok(InnerResponse::new(None, ui.response()))
    }
}
