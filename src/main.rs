use scraper::{element_ref, Selector};
struct Mansion {
    address: String,
    img: String,
}
impl Default for Mansion {
    fn default() -> Mansion {
        Mansion {
            address: "default".to_owned(),
            img: "default".to_owned(),
        }
    }
}

fn main() {
    //let mut mansions: Vec<Mansion> = Vec::new();
    let mut mansion: Mansion = Mansion::default();
    let response = reqwest::blocking::get("https://search.savills.com/property-detail/bvi71435");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    //print!("{}", document.html());
    let address_selector = Selector::parse(".sv-property-intro__address-line-1").unwrap();
    for element in document.select(&address_selector) {
        let text = dbg!(element.text().collect::<Vec<_>>().join(" "));
        mansion.address = text;
    }
    let image_selector = Selector::parse("div.Carouselstyled__CarouselTrack-sc-n9ijei-1").unwrap();
    let elements = document.select(&image_selector).collect::<Vec<_>>();
    println!("num of elems: {}", elements.len());
    for element in elements {
        let tag_name = element.value().name();
        let attrs = element
            .value()
            .attrs()
            .map(|(name, value)| format!("{}=\"{}\"", name, value))
            .collect::<Vec<_>>()
            .join(" ");
        let inner_html = element.inner_html();
        let outer_html = format!("<{} {}>{}</{}>", tag_name, attrs, inner_html, tag_name);
        println!("Element outer HTML: {}", outer_html);
    }
}

