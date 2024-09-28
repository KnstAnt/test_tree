//!
#[derive(Clone, PartialEq)]
pub enum Path {
    Mass,
    Loads,
    LoadsMass,
}
///
impl ToString for Path {
    fn to_string(&self) -> String {
        match self {
            Path::Mass => "/mass",
            Path::Loads => "/loads",
            Path::LoadsMass => "/loads/mass",
        }.to_string()
    }
}