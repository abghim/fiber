use slint;
use tectonic;
use std::sync::Arc;
use hayro::{render, Pdf, RenderSettings};
use hayro_interpret::InterpreterSettings;

slint::include_modules!();

fn main() {
	let app_window = App::new().expect("App build error");
	let weak = app_window.as_weak();

	app_window.on_compile(move || {
		let app = weak.upgrade().unwrap();


		let latex = app.get_current();
		let pdf_data = tectonic::latex_to_pdf(latex).expect("LaTeX processing error");

		println!("LaTeX processing done, file {} bytes", pdf_data.len());

		let scale = 3.0;

		let pdf_data_cell = Arc::new(pdf_data);
		let pdf = Pdf::new(pdf_data_cell).expect("PDF data generation error");

		println!("PDF generation done.");

		/* v Mostly taken from `hayro` crate's public example https://github.com/LaurenzV/hayro/blob/master/hayro/examples/render.rs */

		let interpreter_settings = InterpreterSettings::default();

	    let render_settings = RenderSettings {
	        x_scale: scale,
	        y_scale: scale,
	        ..Default::default()
	    };
	    let mut slint_images: Vec<slint::Image> = Vec::new();

		println!("Starting slint image conversion");

	    for (idx, page) in pdf.pages().iter().enumerate() {
	        let png = render(page, &interpreter_settings, &render_settings).take_png();

	    /* ^ `hayro` crate's example */
		    slint_images.push({
			    let tmp = std::env::temp_dir().join(format!("tmp{idx}.png"));
				println!("Writing temp cache");
			    std::fs::write(&tmp, &png).expect("Temp png cache error");
			    slint::Image::load_from_path(&tmp).expect("Loading from cache error (slint::Image)")
		    });
			println!("Loaded slint image {idx}");
	    }

	    app.set_display(slint_images.get(app.get_current_page() as usize).expect("Accessing index beyond length of pdf; future resolve for no panics").clone());
		println!("Set display to current page");
	});


    app_window.run().unwrap();
}
