use std::{
    fs::read_to_string,
    path::PathBuf,
    process::{Command, Stdio},
};

use eframe::egui::{panel::TopBottomSide, Id, Label, TopBottomPanel, Vec2};
use log::error;
use ron::from_str;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Cfg {
    pub uses: Option<Vec<Cfg>>,
    pub panel_name: String,
    pub items: Vec<Items>,
    pub animate: bool,
    pub side: String,
}
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Items {
    pub name: String,
    pub script: String,
    pub typ: String,
}
impl Cfg {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        Self::default()
    }
    pub(crate) fn load(f: PathBuf) {
        let f = read_to_string(f).unwrap();
        let cfg: Self = from_str(&f).unwrap();

        Self {
            uses: cfg.uses,
            panel_name: cfg.panel_name,
            items: cfg.items,
            animate: cfg.animate,
            side: cfg.side,
        };
    }
}
impl eframe::App for Cfg {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.animate_bool(self.panel_name.clone().into(), self.animate);
        let side: TopBottomSide = match self.side.as_str() {
            "top" => TopBottomSide::Top,
            "bottom" => TopBottomSide::Bottom,
            _ => TopBottomSide::Top,
        };
        TopBottomPanel::new(side, Id::new(self.panel_name.clone())).show(ctx, |ui| {
            // ui.allocate_space();
            for ty in self.items.iter() {
                match ty.typ.as_str() {
                    "button" => {
                        if ui.button(&ty.name).clicked() {
                            Command::new(&ty.script).output().unwrap();
                        }
                    }
                    "text" => {
                        ui.add(
                            Label::new(format!(
                                "{:#?}",
                                Command::new(&ty.script)
                                    .stdout(Stdio::inherit())
                                    .output()
                                    .unwrap()
                            ))
                            .wrap(),
                        );
                    }
                    _ => error!("Unknown component type"),
                }
            }
        });
    }
}
