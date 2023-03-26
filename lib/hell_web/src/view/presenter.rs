use super::Element;

pub trait Presenter {
    fn root(&self) -> &Element;
}
