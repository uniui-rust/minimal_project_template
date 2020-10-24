fn main () {
	let mut build = uniui_build::WasmBuilder::for_framework(uniui_build::Framework::Tide);

	build.add_page(sitemap::MAIN_PAGE, "main_page");
	// ? build.default_css_theme(...);
	build.execute();
}
