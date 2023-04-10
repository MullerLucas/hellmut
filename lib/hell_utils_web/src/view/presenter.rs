use super::Element;

pub trait ElementTree {
    fn root(&self) -> &Element;
}

pub trait Presenter: ElementTree { }
