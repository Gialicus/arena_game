#[derive(Debug, Clone)]
pub enum Element {
    Water,
    Fire,
    Earth,
    Air,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Water => write!(f, "Water"),
            Element::Fire => write!(f, "Fire"),
            Element::Earth => write!(f, "Earth"),
            Element::Air => write!(f, "Air"),
        }
    }
}
