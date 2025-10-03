use slint;
use tectonic;
use std::sync::Arc;
use hayro::{render, Pdf, Pixmap, RenderSettings};
use hayro_interpret::InterpreterSettings;

slint::include_modules!();

fn main() {
	let app = App::new().expect("App build error");

	let mut latex = app.get_current();
	let pdf_data = tectonic::latex_to_pdf(latex).expect("Latex processing error");

	let scale = 3.0;

	let pdf_data_cell = Arc::new(pdf_data);
	let pdf = Pdf::new(pdf_data_cell).expect("PDF data generation error");

	/* v Mostly taken from `hayro` crate's public example https://github.com/LaurenzV/hayro/blob/master/hayro/examples/render.rs */

	let interpreter_settings = InterpreterSettings::default();

    let render_settings = RenderSettings {
        x_scale: scale,
        y_scale: scale,
        ..Default::default()
    };
    let mut slint_images: Vec<slint::Image> = Vec::new();

    for (idx, page) in pdf.pages().iter().enumerate() {
        let png = render(page, &interpreter_settings, &render_settings).take_png();

    /* ^ `hayro` crate's example */
	    slint_images.push({
		    let tmp = std::env::temp_dir().join(format!("tmp{idx}.png"));
		    std::fs::write(&tmp, &png).expect("Temp png cache error");
		    slint::Image::load_from_path(&tmp).expect("Loading from cache error (slint::Image)")
	    });
    }

    app.set_display(slint_images.get(app.get_current_page() as usize).expect("Accessing index beyond length of pdf; future resolve for no panics").clone());

    app.run().unwrap();
}
