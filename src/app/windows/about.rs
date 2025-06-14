use crate::app::ICON_SIZE;
use egui::{Context, Label, RichText, Sense, Widget, Window};
use egui_phosphor::regular::{COPYRIGHT, GITHUB_LOGO, GLOBE, INFO, WARNING};

const AFFILIATION: &str = "K. A. Timiryazev Institute of Plant Physiology, Russian Academy of Sciences, Botanicheskaya Street 35, 127276 Moscow, Russia";

/// About
#[derive(Debug, Default)]
pub(crate) struct About {
    pub(crate) open: bool,
}

impl About {
    pub(crate) fn window(&mut self, ctx: &Context) {
        Window::new(format!("{INFO} About"))
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let version = env!("CARGO_PKG_VERSION");
                    ui.label(RichText::new(format!("HMFA {version}")).heading());
                    ui.label("Human Milk Fatty Acids");
                    ui.label(format!("{COPYRIGHT} 2024"));
                    ui.separator();
                    ui.heading("Authors");
                    let authors = env!("CARGO_PKG_AUTHORS");
                    for author in authors.split(':') {
                        Label::new(author).sense(Sense::click()).ui(ui);
                    }
                    ui.separator();
                    ui.heading("Affiliation");
                    Label::new(AFFILIATION).wrap().ui(ui);
                    ui.separator();
                    ui.collapsing(RichText::new("Links").heading(), |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(GLOBE).size(ICON_SIZE))
                                .on_hover_text("web");
                            ui.hyperlink_to(
                                "https://ippras.github.io/hmfa",
                                "https://ippras.github.io/hmfa",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(GITHUB_LOGO).size(ICON_SIZE))
                                .on_hover_text("github.com");
                            ui.hyperlink_to(
                                "https://github.com/ippras/hmfa",
                                "https://github.com/ippras/hmfa",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(WARNING).size(ICON_SIZE))
                                .on_hover_text("report an issue");
                            ui.hyperlink_to(
                                "https://github.com/ippras/hmfa/issues",
                                "https://github.com/ippras/hmfa/issues",
                            );
                        });
                    });
                });
            });
    }
}
