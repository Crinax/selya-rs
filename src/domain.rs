#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XmlElement {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<XmlElement>,
}
