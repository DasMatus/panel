#![allow(special_module_name)]
use eframe::run_native;
use lib::{Cfg, Items};
use log::info;
use std::{error::Error, fs::DirBuilder, path::Path};
mod lib;
type MyEguiApp = lib::Cfg;
fn main() -> Result<(), Box<dyn Error>> {
    colog::init();
    let path = Path::new(env!("HOME")).join(".config/mtp");
    let cfg = Cfg {
        uses: Some(vec![Cfg::default()]),
        panel_name: "".to_string(),
        items: vec![Items {
            name: "".to_string(),
            script: "".to_string(),
            typ: "".to_string()
        }],
        side: "".to_string(),
        animate: true,
    };
    if !path.exists() {
        DirBuilder::new().create(&path)?;
        std::fs::write(
            path.join("config"),
            ron::ser::to_string_pretty(&cfg, ron::ser::PrettyConfig::default())?,
        )?;
        info!("Generated config file at ~/.config/mtp/config");
    } else {
        lib::Cfg::load(path.join("config"));

        run_native(
            "panel",
            eframe::NativeOptions::default(),
            Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
        )
        .unwrap()
    }
    Ok(())
}
