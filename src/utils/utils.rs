use gloo::utils::window;

#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(PartialEq, Clone)]
pub enum Align {
    Left,
    Center,
}
impl Align {
    pub fn to_class(&self) -> String {
        match self {
            Align::Left => "",
            Align::Center => "my-0 mx-auto",
        }
        .into()
    }
}

pub fn scroll_into_view(page: &i8) {
    let win = window();
    let doc = win.document().unwrap();
    let anchor = format!("a[href=\"#{}\"]", &page);
    let elem = doc.query_selector(&anchor);

    if elem.is_err() {
        return;
    }
    let unwrap = elem.unwrap_or(None);
    if let Some(anchor) = unwrap {
        anchor.scroll_into_view();
    }
}
