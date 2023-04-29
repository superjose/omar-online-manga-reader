#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(PartialEq, Clone)]
pub enum Align {
    Left,
    Right,
    Center,
}
impl Align {
    pub fn to_class(&self) -> String {
        match self {
            Align::Left => "",
            Align::Right => "right",
            Align::Center => "my-0 mx-auto",
        }
        .into()
    }
}
