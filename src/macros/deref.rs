#[macro_export]
macro_rules! impl_deref_from_type {

    ($name: ident, $type: ident) => {
        impl core::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                let a = &self.0;
                a as &Self::Target
            }
        }
    };
}

