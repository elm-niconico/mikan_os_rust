#[macro_export]
macro_rules! impl_debug_bit_filed {
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
macro_rules! impl_debug_only_fields {
    ($name:ident {
        $($field:ident),*$(,)?
    }) => {
        impl core::fmt::Debug for $name {
            fn fmt(&self, f:&mut core::fmt::Formatter<'_>) -> core::fmt::Result{
                f.debug_struct(core::stringify!($name))
                    $(.field(core::stringify!($field), &self.$field))*
                    .finish()
            }
        }
    };
}

// 以下のページ参照
// https://github.com/rust-lang/rust/issues/82523
#[macro_export]
macro_rules! impl_debug_packed_fields {
    ($name:ident {
        $($field:ident),*$(,)?
    }) => {
        impl core::fmt::Debug for $name {
            fn fmt(&self, f:&mut core::fmt::Formatter<'_>) -> core::fmt::Result{
                f.debug_struct(core::stringify!($name))
                    $(.field(core::stringify!($field), &{self.$field}))*
                    .finish()
            }
        }
    };
}
