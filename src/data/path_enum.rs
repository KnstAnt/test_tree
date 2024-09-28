//!
#[derive(Clone, PartialEq)]
///
pub enum Loads {
    Mass,
    Volume,
    Level,
}
///
impl ToString for Loads {
    fn to_string(&self) -> String {
        match self {
            Loads::Mass => "/mass",
            Loads::Volume => "/volume",
            Loads::Level => "/level",
        }
        .to_string()
    }
}
///
#[derive(Clone, PartialEq)]
pub enum Path {
    Mass,
    Loads(Loads),
    }

impl ToString for Path {
    fn to_string(&self) -> String {
        match self {
            Path::Mass => "/mass".to_string(),
            Path::Loads(loads) => "/loads".to_owned() + &loads.to_string(),
        }        
    }
}
