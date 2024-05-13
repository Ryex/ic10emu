macro_rules! GWStructure {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWStructure for $struct {
            fn small_grid(&self) -> bool {
                self.small_grid
            }
        }
    };
}
pub(crate) use GWStructure;

macro_rules! GWStorage {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWStorage for $struct {
            fn slots(&self) -> &Vec<Slot> {
                &self.slots
            }
            fn slots_mut(&mut self) -> &mut Vec<Slot> {
                &mut self.slots
            }
        }
    };
}
pub(crate) use GWStorage;

macro_rules! GWLogicable {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWLogicable for $struct {
            fn fields(&self) -> &BTreeMap<LogicType, LogicField> {
                &self.fields
            }
            fn fields_mut(&mut self) -> &mut BTreeMap<LogicType, LogicField> {
                &mut self.fields
            }
            fn modes(&self) -> Option<&BTreeMap<u32, String>> {
                self.modes.as_ref()
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
        impl GWDevice for $struct {
            fn device_info(&self) -> &DeviceInfo {
                &self.device_info
            }
            fn device_connections(&self) -> &[Connection] {
                self.connections.as_slice()
            }
            fn device_connections_mut(&mut self) -> &mut [Connection] {
                self.connections.as_mut_slice()
            }
            fn device_pins(&self) -> Option<&[Option<ObjectID>]> {
                self.pins.as_ref().map(|pins| pins.as_slice())
            }
            fn device_pins_mut(&mut self) -> Option<&mut [Option<ObjectID>]> {
                self.pins.as_mut().map(|pins| pins.as_mut_slice())
            }
        }
    };
}
pub(crate) use GWDevice;

macro_rules! GWItem {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWItem for $struct {
            fn item_info(&self) -> &ItemInfo {
                &self.item_info
            }
            fn parent_slot(&self) -> Option<ParentSlotInfo> {
                self.parent_slot
            }
            fn set_parent_slot(&mut self, info: Option<ParentSlotInfo>) {
                self.parent_slot = info;
            }
        }
    };
}
pub(crate) use GWItem;
