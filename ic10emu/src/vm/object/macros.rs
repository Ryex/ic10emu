macro_rules! object_trait {
    (@intf $trait_name:ident $trt:path) => {
        paste::paste! {
        #[allow(missing_docs)]
        pub type [<$trt Ref>]<'a, T> = &'a dyn $trt<ID = <T as $trait_name>::ID>;
        #[allow(missing_docs)]
        pub type [<$trt RefMut>]<'a, T> = &'a mut dyn $trt<ID = <T as $trait_name>::ID>;
        }
    };
    ( $trait_name:ident {$($trt:path),*}) => {
        $(
            object_trait!{@intf $trait_name $trt}
        )*

        pub trait $trait_name {
            type ID;
            fn id(&self) -> &Self::ID;

            fn as_object(&self) -> &dyn $trait_name<ID = Self::ID>;

            fn as_object_mut(&mut self) -> &mut dyn $trait_name<ID = Self::ID>;

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

pub(in crate) use object_trait;

macro_rules! ObjectInterface {
    {
        #[custom(implements($trait_name:ident {$($trt:path),*}))]
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $(
            $(#[#field1:meta])*
            $field1_viz:vis
            $field1_name:ident : $field1_ty:ty,
            ),*
            #[custom(object_id)]
            $(#[$id_attr:meta])*
            $id_viz:vis $field_id:ident: $id_typ:ty,
            $(
            $(#[#field2:meta])*
            $field2_viz:vis
            $field2_name:ident : $field2_ty:ty,
            ),*
        }
    } => {
        impl $trait_name for $struct {
            type ID = $id_typ;

            fn id(&self) -> &Self::ID {
                &self.$field_id
            }

            #[inline(always)]
            fn as_object(&self) -> &dyn $trait_name<ID = Self::ID> {
                self
            }

            #[inline(always)]
            fn as_object_mut(&mut self) -> &mut dyn $trait_name<ID = Self::ID> {
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

        }

    };
}
pub(in crate) use ObjectInterface;

macro_rules! ObjectTrait {
    {
        #[custom(object_trait = $trait_name:ident)]
        $(#[$attr:meta])*
        $viz:vis trait $trt:ident $(: $($bound:path)* )? {
            $($tbody:tt)*
        }
    } => {
        $(#[$attr])*
        $viz trait $trt: $($($bound)* +)? $trait_name {
            $($tbody)*
        }

    };
}

pub(in crate) use ObjectTrait;
