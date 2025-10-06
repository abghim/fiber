use slint;
use tectonic;
use std::io::Write;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use std::fs::File;
use hayro::{render, Pdf, RenderSettings};
use hayro_interpret::InterpreterSettings;


slint::include_modules!();

#[derive(Default)]
struct SharedState {
	compiled: Vec<slint::Image>,
	writeable: Vec<u8>
}

fn main() {
	let app_window = App::new().expect("App build error");
	let app_weak = app_window.as_weak();

	let app_state = Rc::new(RefCell::new(SharedState::default()));

	{
		let weak = app_weak.clone();
		let app_state_clone = Rc::clone(&app_state);

		app_window.on_compile(move || {
			let mut st = app_state_clone.borrow_mut();
			let app = weak.upgrade().unwrap();

			let latex = app.get_current();

			let pdf_data = match tectonic::latex_to_pdf(latex) {
				Ok(val) => val,
				Err(_e) => tectonic::latex_to_pdf("\\documentclass{article}\n\\title{LaTeX processing error}\n\\begin{document}\n\n\\maketitle\\end{document}").expect("Huh?")
			};

			let pdf_data_write = pdf_data.clone();

			let scale = 2.5;

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


			app.set_current_page((slint_images.len().min(app.get_current_page() as usize + 1)-1) as i32);


		    app.set_display(slint_images.get(app.get_current_page() as usize).expect("Accessing index beyond length of pdf; future resolve for no panics").clone());
			st.compiled = slint_images.clone();
			st.writeable = pdf_data_write.clone();
		});
	}

	{
		let weak = app_weak.clone();
		let app_state_clone = Rc::clone(&app_state);

		app_window.on_next(move || {
			let st = app_state_clone.borrow();
			let app = weak.upgrade().unwrap();

			if (app.get_current_page() as usize + 1) < st.compiled.len() {
				app.set_current_page(app.get_current_page()+1);
			}

		    app.set_display(st.compiled.get(app.get_current_page() as usize).expect("Accessing index beyond length of pdf; future resolve for no panics").clone());

		});
	}
	{
		let weak = app_weak.clone();
		let app_state_clone = Rc::clone(&app_state);

		app_window.on_prev(move || {
			let st = app_state_clone.borrow();
			let app = weak.upgrade().unwrap();

			if (app.get_current_page() as usize) > 0 {
				app.set_current_page(app.get_current_page()-1);
			}

		    app.set_display(st.compiled.get(app.get_current_page() as usize).expect("Accessing index beyond length of pdf; future resolve for no panics").clone());

		});
	}
	{
		let weak = app_weak.clone();
		let app_state_clone = Rc::clone(&app_state);

		app_window.on_download(move || {
			let st = app_state_clone.borrow();
			let app = weak.upgrade().unwrap();

			if (app.get_current_page() as usize) > 0 {
				app.set_current_page(app.get_current_page()-1);
			}

			let home = dirs::home_dir().expect("Could not find home directory");
			let downloads = home.join("Downloads");
		    let filepath = downloads.join("fiber_download.pdf");

		    let mut file = File::create(&filepath).expect("File creation error");
		    file.write(&st.writeable).expect("Write error");
		});
	}

    app_window.run().unwrap();
}
