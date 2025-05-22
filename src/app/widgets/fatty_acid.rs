use crate::presets::_10_1021_jf903048p::MATURE_MILK;
use egui::{
    ComboBox, DragValue, Grid, Id, InnerResponse, PopupCloseBehavior, Response, Sense, Ui, UiKind,
    Vec2, Widget, collapsing_header, style::Widgets,
};
use egui_l20n::ResponseExt as _;
use lipid::prelude::*;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{hash::Hash, num::NonZeroI8};

/// Fatty acid widget
pub(crate) struct FattyAcidWidget {
    fatty_acid: Option<FattyAcidChunked>,
    id_salt: Id,
    editable: bool,
    hover: bool,
}

impl FattyAcidWidget {
    pub(crate) fn new(fatty_acid: Option<FattyAcidChunked>) -> Self {
        Self {
            fatty_acid,
            id_salt: Id::new("FattyAcid"),
            editable: false,
            hover: false,
        }
    }

    pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
        self.id_salt = Id::new(id_salt);
        self
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

    pub(crate) fn show(self, ui: &mut Ui) -> InnerResponse<Option<FattyAcidChunked>> {
        let mut inner = None;
        // None
        let Some(fatty_acid) = self.fatty_acid else {
            let mut response = ui.label("None");
            if self.editable {
                let mut changed = false;
                response.context_menu(|ui| {
                    if ui.button("Some").clicked() {
                        inner = Some(Default::default());
                        changed = true;
                        ui.close_menu();
                    }
                });
                if changed {
                    response.mark_changed();
                };
            }
            return InnerResponse::new(inner, response);
        };
        // Some
        let text = &format!("{:#}", fatty_acid.display(Default::default()));
        let mut response = if self.editable {
            let mut changed = false;
            let mut response = ui.add_sized(
                [ui.available_width(), ui.spacing().interact_size.y],
                |ui: &mut Ui| {
                    let response = ui
                        .menu_button(text, |ui| {
                            let response =
                                FattyAcidContent::new(self.id_salt, &fatty_acid).show(ui);
                            inner = Some(fatty_acid.clone());
                            changed |= response.changed();
                        })
                        .response;
                    // Popup::menu(&response).close_behavior(PopupCloseBehavior::IgnoreClicks);
                    response
                },
            );
            response.context_menu(|ui| {
                let response = ui.button("None");
                if response.clicked() {
                    inner = None;
                    changed = true;
                    ui.close_menu();
                }
            });
            if changed {
                response.mark_changed();
            };
            response
        } else {
            ui.label(text)
        };
        if self.hover {
            response = response.on_hover_text(text);
        }
        InnerResponse::new(inner, response)

        // let fatty_acid = (self.value)()?;
        // let text = match &fatty_acid {
        //     Some(fatty_acid) => &format!("{:#}", fatty_acid.display(Default::default())),
        //     None => "",
        // };
        // let mut inner = None;
        // let mut response = if self.editable {
        //     let current_value = fatty_acid.unwrap_or_default();
        //     let response = ComboBox::from_id_salt(ui.next_auto_id())
        //         .width(ui.available_width())
        //         .selected_text(text)
        //         .show_ui(ui, |ui| -> PolarsResult<()> {
        //             let mature_milk = MATURE_MILK.data.try_fatty_acid_list()?;
        //             for selected_value in mature_milk.iter() {
        //                 let text = format!("{:#}", selected_value.display(Default::default()));
        //                 // if let Some(selected_value) = mature_milk.get(index) {
        //                 //     let text = format!("{:#}", selected_value.display(Default::default()));
        //                 //     // if ui
        //                 //     //     .selectable_value(current_value, *selected_value, text)
        //                 //     //     .changed()
        //                 //     // {
        //                 //     //     inner = Some(current_value.clone())
        //                 //     // }
        //                 // }
        //             }
        //             Ok(())
        //         });
        //     response.response
        // } else {
        //     ui.label(text)
        // };
        // if self.hover {
        //     response = response.on_hover_text(text);
        // }
        // Ok(InnerResponse::new(inner, response))
        // Ok(InnerResponse::new(None, ui.response()))
    }
}

impl Widget for FattyAcidWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

/// Fatty acid content
struct FattyAcidContent<'a> {
    id_salt: Id,
    fatty_acid: &'a FattyAcidChunked,
}

impl<'a> FattyAcidContent<'a> {
    fn new(id_salt: Id, fatty_acid: &'a FattyAcidChunked) -> Self {
        Self {
            id_salt,
            fatty_acid,
        }
    }

    fn show(&mut self, ui: &mut Ui) -> Response {
        let widgets = if ui.visuals().dark_mode {
            Widgets::dark()
        } else {
            Widgets::light()
        };
        ui.visuals_mut().widgets.inactive.weak_bg_fill = widgets.hovered.weak_bg_fill;
        ui.visuals_mut().widgets.hovered.bg_stroke = widgets.hovered.bg_stroke;

        let mut state: State = ui.data_mut(|data| data.get_temp(self.id_salt).unwrap_or_default());

        let mut outer_response = ui.allocate_response(Default::default(), Sense::hover());
        let openness = ui.ctx().animate_bool(self.id_salt, state.is_opened);
        Grid::new(ui.auto_id_with(self.id_salt)).show(ui, |ui| {
            for (index, (mut offset, mut bound)) in self.fatty_acid.iter().enumerate() {
                let text = match offset {
                    Some(Some(index)) => &index.to_string(),
                    Some(None) => "?",
                    None => "*",
                };
                let delta = (index + 1) as i8;
                ComboBox::from_id_salt(ui.auto_id_with(self.id_salt))
                    .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                    .width(ui.spacing().combo_width / 2.0)
                    .selected_text(text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut offset,
                            Some(NonZeroI8::new(delta)),
                            delta.to_string(),
                        );
                        ui.selectable_value(&mut offset, Some(None), "?");
                        ui.selectable_value(&mut offset, None, "*");
                    });
                let text = Bound::display(bound).to_string();
                ComboBox::from_id_salt(ui.auto_id_with(self.id_salt))
                    .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                    .width(ui.spacing().combo_width / 2.0)
                    .selected_text(text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut bound, Some(Bound::S), S);
                        ui.selectable_value(&mut bound, Some(Bound::D), D);
                        ui.selectable_value(&mut bound, Some(Bound::DC), DC);
                        ui.selectable_value(&mut bound, Some(Bound::DT), DT);
                        ui.selectable_value(&mut bound, Some(Bound::T), T);
                        ui.selectable_value(&mut bound, Some(Bound::TC), TC);
                        ui.selectable_value(&mut bound, Some(Bound::TT), TT);
                        ui.selectable_value(&mut bound, Some(Bound::U), U);
                        ui.selectable_value(&mut bound, Some(Bound::UC), UC);
                        ui.selectable_value(&mut bound, Some(Bound::UT), UT);
                        ui.selectable_value(&mut bound, None, B);
                    });
                ui.end_row();
            }
        });
        // for (index, (mut offset, mut bound)) in self.fatty_acid.iter().enumerate() {
        //     ui.horizontal(|ui| {
        //         let text = match offset {
        //             Some(Some(index)) => &index.to_string(),
        //             Some(None) => "?",
        //             None => "*",
        //         };
        //         let delta = (index + 1) as i8;
        //         ComboBox::from_id_salt(ui.auto_id_with(self.id_salt))
        //             .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        //             .width(ui.spacing().combo_width / 2.0)
        //             .selected_text(text)
        //             .show_ui(ui, |ui| {
        //                 ui.selectable_value(
        //                     &mut offset,
        //                     Some(NonZeroI8::new(delta)),
        //                     delta.to_string(),
        //                 );
        //                 ui.selectable_value(&mut offset, Some(None), "?");
        //                 ui.selectable_value(&mut offset, None, "*");
        //             });
        //         let text = Bound::display(bound).to_string();
        //         ComboBox::from_id_salt(ui.auto_id_with(self.id_salt))
        //             .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        //             .width(ui.spacing().combo_width / 2.0)
        //             .selected_text(text)
        //             .show_ui(ui, |ui| {
        //                 ui.selectable_value(&mut bound, Some(Bound::S), S);
        //                 ui.selectable_value(&mut bound, Some(Bound::D), D);
        //                 ui.selectable_value(&mut bound, Some(Bound::DC), DC);
        //                 ui.selectable_value(&mut bound, Some(Bound::DT), DT);
        //                 ui.selectable_value(&mut bound, Some(Bound::T), T);
        //                 ui.selectable_value(&mut bound, Some(Bound::TC), TC);
        //                 ui.selectable_value(&mut bound, Some(Bound::TT), TT);
        //                 ui.selectable_value(&mut bound, Some(Bound::U), U);
        //                 ui.selectable_value(&mut bound, Some(Bound::UC), UC);
        //                 ui.selectable_value(&mut bound, Some(Bound::UT), UT);
        //                 ui.selectable_value(&mut bound, None, B);
        //             });
        //     });
        // }

        // ui.horizontal(|ui| {
        //     // Carbons
        //     let mut carbons = self.fatty_acid.carbons();
        //     let response = ui
        //         .add(DragValue::new(&mut carbons))
        //         .on_hover_localized("carbons.hover");
        //     if response.changed() {
        //         loop {
        //             match carbons.cmp(&self.fatty_acid.carbons()) {
        //                 Ordering::Less => {
        //                     self.fatty_acid.slice().unsaturated.pop();
        //                 }
        //                 Ordering::Equal => break,
        //                 Ordering::Greater => {
        //                     self.fatty_acid.unsaturated.push(Unsaturated {
        //                         index: Some(0),
        //                         isomerism: Some(Isomerism::Cis),
        //                         unsaturation: Some(Unsaturation::One),
        //                     });
        //                 }
        //             }
        //         }
        //     }
        //     outer_response |= response;
        //     // Unsaturated
        //     let mut unsaturated = self.fatty_acid.unsaturated_count();
        //     let response = ui
        //         .add(
        //             DragValue::new(&mut unsaturated)
        //                 .range(0..=carbons)
        //                 .clamp_existing_to_range(true),
        //         )
        //         .on_hover_localized("unsaturated.hover");
        //     if response.changed() {
        //         loop {
        //             match unsaturated.cmp(&self.fatty_acid.unsaturated.len()) {
        //                 Ordering::Less => {
        //                     self.fatty_acid.unsaturated.pop();
        //                 }
        //                 Ordering::Equal => break,
        //                 Ordering::Greater => {
        //                     self.fatty_acid.unsaturated.push(Unsaturated {
        //                         index: Some(0),
        //                         isomerism: Some(Isomerism::Cis),
        //                         unsaturation: Some(Unsaturation::One),
        //                     });
        //                 }
        //             }
        //         }
        //     }
        //     // outer_response |= response;
        //     // if unsaturated == 0 {
        //     //     ui.disable();
        //     // }
        //     let (_, response) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::click());
        //     collapsing_header::paint_default_icon(ui, openness, &response);
        //     if response.clicked() {
        //         state.is_opened ^= true;
        //     }
        //     outer_response |= response;
        // });
        // if 0.0 < openness {
        //     ui.separator();
        //     if !self.fatty_acid.unsaturated.is_empty() {
        //         let response = UnsaturatedContent::new(self.id_salt, &mut self.fatty_acid).show(ui);
        //         outer_response |= response;
        //     }
        // }
        ui.data_mut(|data| data.insert_temp(self.id_salt, state));
        outer_response
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
struct State {
    is_opened: bool,
}
