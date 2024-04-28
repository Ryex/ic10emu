
macro_rules! __define_object_interface_for {
    ($trt:ident) => {
        paste::paste! {
        #[allow(missing_docs)]
        pub type [<$trt Ref>]<'a, T> = &'a dyn $trt<ID = <T as Object>::ID>;
        #[allow(missing_docs)]
        pub type [<$trt RefMut>]<'a, T> = &'a mut dyn $trt<ID = <T as Object>::ID>;
        }
    };
}

macro_rules! object_trait_for {
    ( $($trt:ident),*) => {
        $(
            __define_object_interface_for!{$trt}
        )*
        pub trait Object {
            type ID;
            fn id(&self) -> &Self::ID;

            fn as_object(&self) -> &dyn Object<ID = Self::ID>;

            fn as_object_mut(&mut self) -> &mut dyn Object<ID = Self::ID>;

            paste::paste!{$(
            #[inline(always)]
            fn [<as_ $trt:lower>](&self) -> Option<[<$trt Ref>]<Self>> {
                None
            }

            #[inline(always)]
            fn [<as_ $trt:lower _mut>](&mut self) -> Option<[<$trt RefMut>]<Self>> {
                None
            }
            )*}
        }
    };
}

macro_rules! __emit_dollar__ {
    ($($rules:tt)*) => {
    macro_rules! __emit__ { $($rules)* }
    __emit__! { $ }
    };
}

pub(in crate) use __emit_dollar__;

macro_rules! alias {
    ($($name:ident -> $(#[$($stuff:tt)*])+;)* ) => {
        $(
            $crate::vm::object::macros::__emit_dollar__! { ($_:tt) => (
                #[allow(nonstandard_style)]
                macro_rules! $name {($_($item:tt)*) => (
                $( #[$($stuff)*] )+
                    $_($item)*
                )}
                #[allow(unused_imports)]
                pub(in crate) use $name;
            )}
        )*

    };
}
pub(in crate) use alias;



macro_rules! impl_trait_interfaces {
    ( $($trt:ident),*) => {
        #[inline(always)]
        fn as_object(&self) -> &dyn Object<ID = Self::ID> {
            self
        }

        #[inline(always)]
        fn as_object_mut(&mut self) -> &mut dyn Object<ID = Self::ID> {
            self
        }

        paste::paste!{$(
        #[inline(always)]
        fn [<as_ $trt:lower>](&self) -> Option<[<$trt Ref>]<Self>> {
            Some(self)
        }

        #[inline(always)]
        fn [<as_ $trt:lower _mut>](&mut self) -> Option<[<$trt RefMut>]<Self>> {
            Some(self)
        }
        )*}
    };
}
