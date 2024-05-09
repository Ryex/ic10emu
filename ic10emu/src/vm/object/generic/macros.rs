macro_rules! GWLogicable {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWLogicable for $struct {
            fn name(&self) -> &Option<Name> {
                &self.name
            }
            fn fields(&self) -> &BTreeMap<LogicType, LogicField> {
                &self.fields
            }
            fn fields_mut(&mut self) -> &mut BTreeMap<LogicType, LogicField> {
                &mut self.fields
            }
            fn slots(&self) -> &Vec<Slot> {
                &self.slots
            }
            fn slots_mut(&mut self) -> &mut Vec<Slot> {
                &mut self.slots
            }
        }
    };
}
pub(crate) use GWLogicable;

macro_rules! GWMemoryReadable {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWMemoryReadable for $struct {
            fn memory_size(&self) -> usize {
                self.memory.len()
            }
            fn memory(&self) -> &Vec<f64> {
                &self.memory
            }
        }
    };
}
pub(crate) use GWMemoryReadable;

macro_rules! GWMemoryWritable {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWMemoryWritable for $struct {
            fn memory_mut(&mut self) -> &mut Vec<f64> {
                &mut self.memory
            }
        }
    };
}

pub(crate) use GWMemoryWritable;

macro_rules! GWDevice {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWDevice for $struct {}
    };
}
pub(crate) use GWDevice;
