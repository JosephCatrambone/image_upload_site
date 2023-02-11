mod templates;
mod data_models;

use perseus::prelude::*;
use perseus::plugins::Plugins;
use serde::{Serialize, Deserialize};

use data_models::user;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
struct IndexState {
	name: String,
	//#[rx(nested)]  // Another thing.
}



#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
	PerseusApp::new()
		.plugins(Plugins::new().plugin(
			perseus_tailwind::get_tailwind_plugin,
			perseus_tailwind::TailwindOptions {
				in_file: "src/tailwind.css".into(),
				// Don't put this in /static, it will trigger build loops
				out_file: "generated/tailwind.css".into(),
			},
		))
		.template(crate::templates::index::get_template())
		//.templates()
}