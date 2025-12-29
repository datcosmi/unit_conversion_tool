pub trait Conversion {
    type Unit;
    type Error;

    fn to_canonical(self) -> Result<Self, Self::Error> where Self: Sized;
    fn from_canonical(canon: Self, target: Self::Unit) -> Result<Self, Self::Error> where Self: Sized;
}
