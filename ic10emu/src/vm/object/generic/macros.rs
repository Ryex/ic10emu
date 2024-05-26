macro_rules! GWThermal {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWThermal for $struct {
            fn is_thermal(&self) -> bool {
                self.thermal_info.is_some()
            }
            fn thermal_info(&self) -> &ThermalInfo {
                self.thermal_info
                    .as_ref()
                    .expect("GWTherml::thermal_info called on non thermal")
            }
        }
    };
}
pub(crate) use GWThermal;

macro_rules! GWInternalAtmo {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWInternalAtmo for $struct {
            fn is_internal_atmo(&self) -> bool {
                self.internal_atmo_info.is_some()
            }
            fn internal_atmo_info(&self) -> &InternalAtmoInfo {
                self.internal_atmo_info
                    .as_ref()
                    .expect("GWInternalAtmo::internal_atmo_info called on non internal atmo")
            }
        }
    };
}
pub(crate) use GWInternalAtmo;

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
            fn known_modes(&self) -> Option<&BTreeMap<u32, String>> {
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
            fn connections(&self) -> &[Connection] {
                self.connections.as_slice()
            }
            fn connections_mut(&mut self) -> &mut [Connection] {
                self.connections.as_mut_slice()
            }
            fn pins(&self) -> Option<&[Option<ObjectID>]> {
                self.pins.as_ref().map(|pins| pins.as_slice())
            }
            fn pins_mut(&mut self) -> Option<&mut [Option<ObjectID>]> {
                self.pins.as_mut().map(|pins| pins.as_mut_slice())
            }
            fn reagents(&self) -> Option<&BTreeMap<i32, f64>> {
                self.reagents.as_ref()
            }
            fn reagents_mut(&mut self) -> &mut Option<BTreeMap<i32, f64>> {
                &mut self.reagents
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
            fn damage(&self) -> &Option<f32> {
                &self.damage
            }
            fn damage_mut(&mut self) -> &mut Option<f32> {
                &mut self.damage
            }
        }
    };
}
pub(crate) use GWItem;

macro_rules! GWSuit {
    (
        $( #[$attr:meta] )*
        $viz:vis struct $struct:ident {
            $($body:tt)*
        }
    ) => {
        impl GWSuit for $struct {
            fn suit_info(&self) -> &SuitInfo {
                &self.suit_info
            }
        }
    };
}
pub(crate) use GWSuit;
