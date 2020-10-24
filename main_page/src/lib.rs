use uniui_gui::prelude::*;
use uniui_widget_button::prelude::*;

#[uniui_gui::u_main]
pub fn app_main(app: &mut Application) {
	uniui_gui::utils::init_logger(app);

	let button = u_button!{
		text: "Hello {{authors}}!".to_owned(),
	};
	
	app.set_title("{{project-name}}");
	app.set_main_widget(button);
}
