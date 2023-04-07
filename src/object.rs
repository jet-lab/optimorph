use crate::category::Key;

pub trait HasId<Id: Key> {
    fn id(&self) -> Id;
}

#[macro_export]
macro_rules! self_identify {
    ($Type:ty) => {
        impl HasId<$Type> for $Type {
            fn id(&self) -> $Type {
                self.clone()
            }
        }
    };
}
