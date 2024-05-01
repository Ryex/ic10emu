macro_rules! object_trait {
    (@intf {$trait_name:ident $trt:path}) => {
        paste::paste! {
        #[allow(missing_docs, unused)]
        pub type [<$trt Ref>]<'a, T> = &'a dyn $trt<ID = <T as $trait_name>::ID>;
        #[allow(missing_docs, unused)]
        pub type [<$trt RefMut>]<'a, T> = &'a mut dyn $trt<ID = <T as $trait_name>::ID>;
        }
    };
    (@body $trait_name:ident $($trt:path),*) => {
        type ID;
        fn id(&self) -> &Self::ID;
        fn prefab(&self) -> &crate::vm::object::Name;

        fn type_name(&self) -> &str;

        fn as_object(&self) -> &dyn $trait_name<ID = Self::ID>;

        fn as_object_mut(&mut self) -> &mut dyn $trait_name<ID = Self::ID>;


        paste::paste!{$(
        #[inline(always)]
        fn [<as_ $trt:snake>](&self) -> Option<[<$trt Ref>]<Self>> {
            None
        }

        #[inline(always)]
        fn [<as_ $trt:snake _mut>](&mut self) -> Option<[<$trt RefMut>]<Self>> {
            None
        }
        )*}
    };
    ( $trait_name:ident $(: $($bound:tt)* )? {$($trt:path),*}) => {
        $(
            $crate::vm::object::macros::object_trait!{@intf {$trait_name $trt}}
        )*

        pub trait $trait_name $(: $($bound)* )? {

            $crate::vm::object::macros::object_trait!{@body $trait_name $($trt),*}
        }
    };
}

pub(crate) use object_trait;

macro_rules! ObjectInterface {
    {
        @final
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        @prefab $prefab_field:ident: $prefab_typ:ty;
    } => {
        impl $trait_name for $struct {
            type ID = $id_typ;

            fn id(&self) -> &Self::ID {
                &self.$id_field
            }

            fn prefab(&self) -> &crate::vm::object::Name {
                &self.$prefab_field
            }

            fn type_name(&self) -> &str {
                std::any::type_name::<Self>()
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
            fn [<as_ $trt:snake>](&self) -> Option<[<$trt Ref>]<Self>> {
                Some(self)
            }

            #[inline(always)]
            fn [<as_ $trt:snake _mut>](&mut self) -> Option<[<$trt RefMut>]<Self>> {
                Some(self)
            }
            )*}

        }
    };
    {
        @body_id
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        #[custom(object_prefab)]
        $(#[$prefab_attr:meta])*
        $prefab_viz:vis $prefab_field:ident: $prefab_typ:ty,
        $( $rest:tt )*
    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @final
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @prefab $prefab_field: $prefab_typ;
        }
    };
    {
        @body_id
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        $(#[#field:meta])*
        $field_viz:vis
        $field_name:ident : $field_ty:ty,
        $( $rest:tt )*
    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            $( $rest )*
        }
    };
    {
        @body_prefab
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @prefab $prefab_field:ident: $prefab_typ:ty;
        #[custom(object_id)]
        $(#[$id_attr:meta])*
        $id_viz:vis $id_field:ident: $id_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @final
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @prefab $prefab_field: $prefab_typ;
        }
    };
    {
        @body_prefab
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @prefab $prefab_field:ident: $prefab_typ:ty;
        $(#[#field:meta])*
        $field_viz:vis
        $field_name:ident : $field_ty:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_prefab
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @prefab $prefab_field: $prefab_typ;
            $( $rest )*
        }
    };
    {
        @body
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        #[custom(object_prefab)]
        $(#[$prefab_attr:meta])*
        $prefab_viz:vis $prefab_field:ident: $prefab_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_prefab
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @prefab $prefab_field: $prefab_typ;
            $( $rest )*
        }
    };
    {
        @body
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        #[custom(object_id)]
        $(#[$id_attr:meta])*
        $id_viz:vis $id_field:ident: $id_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            $( $rest )*
        }
    };
    {
        @body
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        $(#[#field:meta])*
        $field_viz:vis
        $field_name:ident : $field_ty:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body
            @trt $trait_name; $struct;
            @impls $($trt),*;
            $( $rest )*
        }
    };
    {
        #[custom(implements($trait_name:ident {$($trt:path),*}))]
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $( $body:tt )*
        }
    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body
            @trt $trait_name; $struct;
            @impls $($trt),*;
            $( $body )*
        }
    };
}
pub(crate) use ObjectInterface;

#[allow(unused_macros)]
macro_rules! ObjectTrait {
    {
        #[custom(object_trait = $trait_name:ident)]
        $(#[$attr:meta])*
        $viz:vis trait $trt:ident $(: $($bound:tt)* )? {
            $($tbody:tt)*
        }
    } => {
        $(#[$attr])*
        $viz trait $trt: $($($bound)* +)? $trait_name {
            $($tbody)*
        }

    };
}
#[allow(unused_imports)]
pub(crate) use ObjectTrait;

macro_rules! tag_object_traits {
    {
        @tag
        tag=$trt_name:ident $(: $($obj_bound:tt)* )?;
        acc={ $($tagged_trt:ident,)* }
        $(#[$attr:meta])*
        $viz:vis trait $trt:ident $(: $($trt_bound:path)* )? {
            $($tbody:tt)*
        }
        $($used:tt)*
    } => {
        #[doc = concat!("Autotagged with ", stringify!($trt_name))]
        $(#[$attr])*
        $viz trait $trt : $($($trt_bound)* +)? $trt_name {
            $($tbody)*
        }

        $crate::vm::object::macros::tag_object_traits!{ @tag tag=$trt_name $(: $($obj_bound)* )?; acc={ $trt, $($tagged_trt,)* } $($used)* }
    };
    {
        @tag
        tag=$trt_name:ident $(: $($obj_bound:tt)* )?;
        acc={ $($tagged_trt:ident,)* }
        impl $name:ident for $trt:path {
            $($body:tt)*
        }
        $($used:tt)*
    } => {
        /// Untouched by tag macro
        impl $name for $trt {
            $($body)*
        }
        $crate::vm::object::macros::tag_object_traits!{ @tag tag=$trt_name $(: $($obj_bound)* )?; acc={ $($tagged_trt,)* } $($used)* }
    };
    {
        @tag
        tag=$trt_name:ident $(: $($obj_bound:tt)* )?;
        acc={ $($tagged_trt:ident,)* }
    } => {

        // end tagged traits {$trt_name}

        $crate::vm::object::macros::object_trait!($trt_name $(: $($obj_bound)* )? { $($tagged_trt),* });
    };
    { #![object_trait($trt_name:ident $(: $($bound:tt)* )? )] $($tree:tt)* } => {
        $crate::vm::object::macros::tag_object_traits!{ @tag tag=$trt_name; acc={} $($tree)* }
    };
}

pub(crate) use tag_object_traits;
