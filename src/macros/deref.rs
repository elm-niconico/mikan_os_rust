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

#[macro_export]
macro_rules! impl_debug {
    ($name:ident {
        $($method:ident),*$(,)?
    }) => {
        impl core::fmt::Debug for $name {
            fn fmt(&self, f:&mut core::fmt::Formatter<'_>) -> core::fmt::Result{
                f.debug_struct(core::stringify!($name))
                    $(.field(core::stringify!($method), &self.$method()))*
                    .finish()
            }
        }
    };
}
#[macro_export]
macro_rules! impl_debug_with_generic {
    ($name:ident {
        $($method:ident),*$(,)?
    }) => {
        impl<T: Debug> core::fmt::Debug for $name<T> {
            fn fmt(&self, f:&mut core::fmt::Formatter<'_>) -> core::fmt::Result{
                f.debug_struct(core::stringify!($name))
                    $(.field(core::stringify!($method), &self.$method))*
                    .finish()
            }
        }
    };
}
