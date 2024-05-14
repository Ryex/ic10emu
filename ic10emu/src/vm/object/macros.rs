macro_rules! object_trait {
    (@intf {$trt:ident}) => {
        paste::paste! {
        #[allow(missing_docs, unused)]
        pub type [<$trt Ref>]<'a> = &'a dyn $trt;
        #[allow(missing_docs, unused)]
        pub type [<$trt RefMut>]<'a> = &'a mut dyn $trt;
        }
    };
    (@body $trait_name:ident { $($trt:ident),* }; ) => {
        fn get_id(&self) -> crate::vm::object::ObjectID;
        fn set_id(&mut self, id: crate::vm::object::ObjectID);
        fn get_prefab(&self) -> &crate::vm::object::Name;
        fn get_mut_prefab(&mut self) -> &mut crate::vm::object::Name;
        fn get_name(&self) -> &crate::vm::object::Name;
        fn get_mut_name(&mut self) -> &mut crate::vm::object::Name;
        fn type_name(&self) -> &str;
        fn as_object(&self) -> &dyn $trait_name;
        fn as_mut_object(&mut self) -> &mut dyn $trait_name;

        paste::paste! {
            $(
                #[inline(always)]
                fn [<as_ $trt:snake>](&self) -> Option<[<$trt Ref>]> {
                    None
                }

                #[inline(always)]
                fn [<as_mut_ $trt:snake>](&mut self) -> Option<[<$trt RefMut>]> {
                    None
                }
            )*
        }
    };
    (@intf_struct $trait_name:ident { $($trt:ident),* };) => {
        paste::paste! {
            pub struct [<$trait_name Interfaces>]<'a> {
                $(
                    pub [<$trt:snake>]: Option<[<$trt Ref>]<'a>>,
                )*
            }

            impl<'a> [<$trait_name Interfaces>]<'a> {

                pub fn [<from_ $trait_name:snake>](obj: &'a dyn $trait_name) -> [<$trait_name Interfaces>]<'a> {
                    [<$trait_name Interfaces>] {
                        $(
                            [<$trt:snake>]: obj.[<as_ $trt:snake>](),
                        )*
                    }
                }
            }

        }
    };
    ( $trait_name:ident $(: $($bound:tt)* )? {$($trt:ident),*}) => {
        $(
            $crate::vm::object::macros::object_trait!{@intf {$trt}}
        )*


        #[doc = concat!("Generated with: ", stringify!($($trt),*))]
        pub trait $trait_name $(: $($bound)* )? {

            $crate::vm::object::macros::object_trait!{@body $trait_name {$($trt),*}; }
        }

        $crate::vm::object::macros::object_trait!{@intf_struct $trait_name {$($trt),*}; }
    };
}

pub(crate) use object_trait;

macro_rules! ObjectInterface {
    {
        @body_id_prefab_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        @prefab $prefab_field:ident: $prefab_typ:ty;
        @name $name_field:ident: $name_typ:ty;
    } => {
        impl $trait_name for $struct {

            fn get_id(&self) -> crate::vm::object::ObjectID {
                self.$id_field
            }

            fn set_id(&mut self, id: crate::vm::object::ObjectID) {
                self.$id_field = id;
            }

            fn get_prefab(&self) -> &$prefab_typ {
                &self.$prefab_field
            }

            fn get_mut_prefab(&mut self) -> &mut $prefab_typ {
                &mut self.$prefab_field
            }

            fn get_name(&self) -> &$name_typ {
                &self.$name_field
            }

            fn get_mut_name(&mut self) -> &mut $name_typ {
                &mut self.$name_field
            }

            fn type_name(&self) -> &str {
                std::any::type_name::<Self>()
            }

            #[inline(always)]
            fn as_object(&self) -> &dyn $trait_name {
                self
            }

            #[inline(always)]
            fn as_mut_object(&mut self) -> &mut dyn $trait_name {
                self
            }

            paste::paste!{$(
            #[inline(always)]
            fn [<as_ $trt:snake>](&self) -> Option<[<$trt Ref>]> {
                Some(self)
            }

            #[inline(always)]
            fn [<as_mut_ $trt:snake>](&mut self) -> Option<[<$trt RefMut>]> {
                Some(self)
            }
            )*}

        }
    };
    {
        @body_id_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        @name $name_field:ident: $name_typ:ty;
        #[custom(object_prefab)]
        $(#[$prefab_attr:meta])*
        $prefab_viz:vis $prefab_field:ident: $prefab_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id_prefab_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @prefab $prefab_field: $prefab_typ;
            @name $name_field: $name_typ;
        }
    };
    {
        @body_id_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        @name $name_field:ident: $name_typ:ty;
        $(#[#field:meta])*
        $field_viz:vis
        $field_name:ident : $field_ty:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @name $name_field: $name_typ;
            $( $rest )*
        }
    };
    {
        @body_id_prefab
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        @prefab $prefab_field:ident: $prefab_typ:ty;
        #[custom(object_name)]
        $(#[$name_attr:meta])*
        $name_viz:vis $name_field:ident: $name_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id_prefab_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @prefab $prefab_field: $prefab_typ;
            @name $name_field: $name_typ;
        }
    };
    {
        @body_id_prefab
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        @prefab $prefab_field:ident: $prefab_typ:ty;
        $(#[#field:meta])*
        $field_viz:vis
        $field_name:ident : $field_ty:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id_prefab
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @prefab $prefab_field: $prefab_typ;
            $( $rest )*
        }
    };
    {
        @body_prefab_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @prefab $prefab_field:ident: $prefab_typ:ty;
        @name $name_field:ident: $name_typ:ty;
        #[custom(object_id)]
        $(#[$id_attr:meta])*
        $id_viz:vis $id_field:ident: $id_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id_prefab_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @prefab $prefab_field: $prefab_typ;
            @name $name_field: $name_typ;
        }
    };
    {
        @body_prefab_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @prefab $prefab_field:ident: $prefab_typ:ty;
        @name $name_field:ident: $name_typ:ty;
        $(#[#field:meta])*
        $field_viz:vis
        $field_name:ident : $field_ty:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_prefab_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @prefab $prefab_field: $prefab_typ;
            @name $name_field: $name_typ;
            $( $rest )*
        }
    };
    {
        @body_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @name $name_field:ident: $name_typ:ty;
        #[custom(object_prefab)]
        $(#[$prefab_attr:meta])*
        $prefab_viz:vis $prefab_field:ident: $prefab_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_prefab_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @prefab $prefab_field: $prefab_typ;
            @name $name_field: $name_typ;
            $( $rest )*
        }
    };
    {
        @body_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @name $name_field:ident: $name_typ:ty;
        #[custom(object_id)]
        $(#[$id_attr:meta])*
        $id_viz:vis $id_field:ident: $id_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @name $name_field: $name_typ;
            $( $rest )*
        }
    };
    {
        @body_name
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @name $name_field:ident: $name_typ:ty;
        $(#[#field:meta])*
        $field_viz:vis
        $field_name:ident : $field_ty:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @name $name_field: $name_typ;
            $( $rest )*
        }
    };
    {
        @body_id
        @trt $trait_name:ident; $struct:ident;
        @impls $($trt:path),*;
        @id $id_field:ident: $id_typ:ty;
        #[custom(object_name)]
        $(#[$name_attr:meta])*
        $name_viz:vis $name_field:ident: $name_typ:ty,
        $( $rest:tt )*
    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_id_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @name $name_field: $name_typ;
            $( $rest )*
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
            @body_id_prefab
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @prefab $prefab_field: $prefab_typ;
            $( $rest )*
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
        #[custom(object_name)]
        $(#[$name_attr:meta])*
        $name_viz:vis $name_field:ident: $name_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_prefab_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @prefab $prefab_field: $prefab_typ;
            @name $name_field: $name_typ;
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
            @body_id_prefab
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @id $id_field: $id_typ;
            @prefab $prefab_field: $prefab_typ;
            $( $rest )*
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
        #[custom(object_name)]
        $(#[$name_attr:meta])*
        $name_viz:vis $name_field:ident: $name_typ:ty,
        $( $rest:tt )*

    } => {
        $crate::vm::object::macros::ObjectInterface!{
            @body_name
            @trt $trait_name; $struct;
            @impls $($trt),*;
            @name $name_field: $name_typ;
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
        acc={ $($tagged_trt:ident,)* };
        $(#[$attr:meta])*
        $viz:vis trait $trt:ident $(: $trt_bound_first:tt $(+ $trt_bound_others:tt)* )? {
            $($tbody:tt)*
        }
        $($used:tt)*
    } => {
        #[doc = concat!("Autotagged with ", stringify!($trt_name))]
        $(#[$attr])*
        $viz trait $trt : $( $trt_bound_first $(+ $trt_bound_others)* +)? $trt_name {
            $($tbody)*
        }

        $crate::vm::object::macros::tag_object_traits!{
            @tag
            tag=$trt_name $(: $($obj_bound)* )?;
            acc={ $($tagged_trt,)* $trt, };
            $($used)*
        }
    };
    {
        @tag
        tag=$trt_name:ident $(: $($obj_bound:tt)* )?;
        acc={ $($tagged_trt:ident,)* };
        impl $name:ident for $trt:path {
            $($body:tt)*
        }
        $($used:tt)*
    } => {
        /// Untouched by tag macro
        impl $name for $trt {
            $($body)*
        }
        $crate::vm::object::macros::tag_object_traits!{
            @tag
            tag=$trt_name $(: $($obj_bound)* )?;
            acc={ $($tagged_trt,)* };
            $($used)*
        }
    };
    {
        @tag
        tag=$trt_name:ident $(: $($obj_bound:tt)* )?;
        acc={ $($tagged_trt:ident,)* };
    } => {

        // end tagged traits {$trt_name}

        $crate::vm::object::macros::object_trait!($trt_name $(: $($obj_bound)* )? { $($tagged_trt),* });
    };
    { #![object_trait($trt_name:ident $(: $($bound:tt)* )? )] $($tree:tt)* } => {
        $crate::vm::object::macros::tag_object_traits!{ @tag tag=$trt_name; acc={}; $($tree)* }
    };
}

pub(crate) use tag_object_traits;
