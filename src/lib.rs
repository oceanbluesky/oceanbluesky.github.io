use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    // Select the container div
    let container = document.query_selector(".container").unwrap().unwrap();

    // Create "About" header
    let about_header: Element = document.create_element("h2")?;
    about_header.set_inner_html("About");
    about_header.set_class_name("about-header");
    container.append_child(&about_header)?;

    let linkedin_link = document.create_element("a")?;
    linkedin_link.set_attribute("href", "https://www.linkedin.com/in/yourprofile")?;
    linkedin_link.set_attribute("target", "_blank")?;
    linkedin_link.set_inner_html("<i class='fab fa-linkedin'></i>");
    container.append_child(&linkedin_link)?;

    // Create "About" paragraphs
    let about_paragraph1 = document.create_element("p")?;
    about_paragraph1.set_inner_html("Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
    sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, 
    quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute 
    irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
    Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit 
    anim id est laborum.");
    container.append_child(&about_paragraph1)?;

    let about_paragraph2 = document.create_element("p")?;
    about_paragraph2.set_inner_html("Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.");
    container.append_child(&about_paragraph2)?;

    // Create main content section with paragraphs and thumbnails
    for i in 1..=7 {
        // Create a container for each paragraph and thumbnail
        let paragraph_container = document.create_element("div")?;
        paragraph_container.set_class_name("paragraph-container");

        // Create thumbnail image
        let thumbnail = document.create_element("img")?;
        thumbnail.set_attribute("src", &format!("assets/image{}.png", i))?;
        thumbnail.set_attribute("alt", &format!("Thumbnail {}", i))?;
        thumbnail.set_class_name("thumbnail");
        paragraph_container.append_child(&thumbnail)?;

        // Create paragraph
        let paragraph = document.create_element("p")?;
        let text = format!(
            "{}. More text follows in this paragraph to make it sufficiently long.",
            get_bold_first_sentence(i)
        );
        paragraph.set_inner_html(&text);
        paragraph_container.append_child(&paragraph)?;

        // Append the container to the main container
        container.append_child(&paragraph_container)?;
    }

    Ok(())
}

// Helper function to generate the first sentence in all caps and bold
fn get_bold_first_sentence(index: usize) -> String {
    match index {
        1 => "THIS IS THE FIRST SENTENCE IN PARAGRAPH ONE",
        2 => "THIS IS THE FIRST SENTENCE IN PARAGRAPH TWO",
        3 => "THIS IS THE FIRST SENTENCE IN PARAGRAPH THREE",
        4 => "THIS IS THE FIRST SENTENCE IN PARAGRAPH FOUR",
        5 => "THIS IS THE FIRST SENTENCE IN PARAGRAPH FIVE",
        6 => "THIS IS THE FIRST SENTENCE IN PARAGRAPH SIX",
        7 => "THIS IS THE FIRST SENTENCE IN PARAGRAPH SEVEN",
        _ => "THIS IS A DEFAULT FIRST SENTENCE",
    }
    .to_string()
}


