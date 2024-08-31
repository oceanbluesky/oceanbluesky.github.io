use wasm_bindgen::prelude::*;
use web_sys::{Document, Window};


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("Couldn't get the window object")?
        .document()
        .ok_or("Couldn't get the document object")?;
    
    let body = document.body().ok_or("Couldn't get the body element")?;

    // Create a header for the "About" section
    let about_header = document.create_element("h1")?;
    about_header.set_inner_html("About");
    about_header.set_class_name("about-header");
    body.append_child(&about_header)?;

    // Create "About" paragraphs
    let about_paragraph1 = document.create_element("p")?;
    about_paragraph1.set_inner_html("Lorem ipsum dolor sit amet...");
    body.append_child(&about_paragraph1)?;

    let about_paragraph2 = document.create_element("p")?;
    about_paragraph2.set_inner_html("Consectetur adipiscing elit...");
    body.append_child(&about_paragraph2)?;

    // Create main content section
    for i in 1..=7 {
        let paragraph = document.create_element("p")?;
        let thumbnail = document.create_element("img")?;
        thumbnail.set_attribute("src", &format!("assets/image{}.png", i))?;
        thumbnail.set_class_name("thumbnail");

        let text = format!("This is the FIRST sentence in paragraph {}. More text follows...", i);
        paragraph.set_inner_html(&text);
        body.append_child(&thumbnail)?;
        body.append_child(&paragraph)?;
    }

    Ok(())
}


