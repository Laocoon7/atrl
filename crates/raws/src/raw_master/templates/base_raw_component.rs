pub trait BaseRawComponent: std::fmt::Debug + Clone {
    fn name(&self) -> String;
    fn as_any(&self) -> &dyn std::any::Any;
}

#[macro_export]
macro_rules! impl_raw {
    ($to:ty) => {
        impl BaseRawComponent for $to {
            fn name(&self) -> String { self.name.clone() }

            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    };
}
