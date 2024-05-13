use crate::{
    errors::ICError,
    interpreter::ICState,
    network::{CableConnectionType, Connection},
    vm::enums::{
        basic_enums::{Class as SlotClass, SortingClass},
        script_enums::{LogicReagentMode as ReagentMode, LogicSlotType, LogicType},
    },
    vm::VM,
};
use std::{collections::BTreeMap, ops::Deref};

use itertools::Itertools;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryAccess {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicField {
    pub field_type: MemoryAccess,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotOccupant {
    pub id: u32,
    pub prefab_hash: i32,
    pub quantity: u32,
    pub max_quantity: u32,
    pub sorting_class: SortingClass,
    pub damage: f64,
    fields: BTreeMap<LogicSlotType, LogicField>,
}

impl SlotOccupant {
    pub fn from_template<F>(template: SlotOccupantTemplate, id_fn: F) -> Self
    where
        F: FnOnce() -> u32,
    {
        let mut fields = template.fields;
        SlotOccupant {
            id: template.id.unwrap_or_else(id_fn),
            prefab_hash: fields
                .remove(&LogicSlotType::PrefabHash)
                .map(|field| field.value as i32)
                .unwrap_or(0),
            quantity: fields
                .remove(&LogicSlotType::Quantity)
                .map(|field| field.value as u32)
                .unwrap_or(1),
            max_quantity: fields
                .remove(&LogicSlotType::MaxQuantity)
                .map(|field| field.value as u32)
                .unwrap_or(1),
            damage: fields
                .remove(&LogicSlotType::Damage)
                .map(|field| field.value)
                .unwrap_or(0.0),
            sorting_class: fields
                .remove(&LogicSlotType::SortingClass)
                .map(|field| SortingClass::from_repr(field.value as u8))
                .flatten()
                .unwrap_or(SortingClass::Default),
            fields,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotOccupantTemplate {
    pub id: Option<u32>,
    pub fields: BTreeMap<LogicSlotType, LogicField>,
}

impl SlotOccupant {
    pub fn new(id: u32, prefab_hash: i32) -> Self {
        SlotOccupant {
            id,
            prefab_hash,
            quantity: 1,
            max_quantity: 1,
            damage: 0.0,
            sorting_class: SortingClass::Default,
            fields: BTreeMap::new(),
        }
    }

    /// chainable constructor
    pub fn with_quantity(mut self, quantity: u32) -> Self {
        self.quantity = quantity;
        self
    }

    /// chainable constructor
    pub fn with_max_quantity(mut self, max_quantity: u32) -> Self {
        self.max_quantity = max_quantity;
        self
    }

    /// chainable constructor
    pub fn with_damage(mut self, damage: f64) -> Self {
        self.damage = damage;
        self
    }

    /// chainable constructor
    pub fn with_fields(mut self, fields: BTreeMap<LogicSlotType, LogicField>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// chainable constructor
    pub fn get_fields(&self) -> BTreeMap<LogicSlotType, LogicField> {
        let mut copy = self.fields.clone();
        copy.insert(
            LogicSlotType::PrefabHash,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self.prefab_hash as f64,
            },
        );
        copy.insert(
            LogicSlotType::SortingClass,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self.sorting_class as u32 as f64,
            },
        );
        copy.insert(
            LogicSlotType::Quantity,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self.quantity as f64,
            },
        );
        copy.insert(
            LogicSlotType::MaxQuantity,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self.max_quantity as f64,
            },
        );
        copy.insert(
            LogicSlotType::Damage,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self.damage,
            },
        );
        copy
    }

    pub fn set_field(&mut self, typ: LogicSlotType, val: f64, force: bool) -> Result<(), ICError> {
        if (typ == LogicSlotType::Quantity) && force {
            self.quantity = val as u32;
            Ok(())
        } else if (typ == LogicSlotType::MaxQuantity) && force {
            self.max_quantity = val as u32;
            Ok(())
        } else if (typ == LogicSlotType::Damage) && force {
            self.damage = val;
            Ok(())
        } else if let Some(logic) = self.fields.get_mut(&typ) {
            match logic.field_type {
                MemoryAccess::ReadWrite | MemoryAccess::Write => {
                    logic.value = val;
                    Ok(())
                }
                _ => {
                    if force {
                        logic.value = val;
                        Ok(())
                    } else {
                        Err(ICError::ReadOnlyField(typ.to_string()))
                    }
                }
            }
        } else if force {
            self.fields.insert(
                typ,
                LogicField {
                    field_type: MemoryAccess::ReadWrite,
                    value: val,
                },
            );
            Ok(())
        } else {
            Err(ICError::ReadOnlyField(typ.to_string()))
        }
    }

    pub fn can_logic_read(&self, field: LogicSlotType) -> bool {
        if let Some(logic) = self.fields.get(&field) {
            matches!(
                logic.field_type,
                MemoryAccess::Read | MemoryAccess::ReadWrite
            )
        } else {
            false
        }
    }

    pub fn can_logic_write(&self, field: LogicSlotType) -> bool {
        if let Some(logic) = self.fields.get(&field) {
            matches!(
                logic.field_type,
                MemoryAccess::Write | MemoryAccess::ReadWrite
            )
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Slot {
    pub typ: SlotClass,
    pub occupant: Option<SlotOccupant>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SlotTemplate {
    pub typ: SlotClass,
    pub occupant: Option<SlotOccupantTemplate>,
}

impl Slot {
    pub fn new(typ: SlotClass) -> Self {
        Slot {
            typ,
            occupant: None,
        }
    }
    pub fn with_occupant(typ: SlotClass, occupant: SlotOccupant) -> Self {
        Slot {
            typ,
            occupant: Some(occupant),
        }
    }

    pub fn get_fields(&self) -> BTreeMap<LogicSlotType, LogicField> {
        let mut copy = self
            .occupant
            .as_ref()
            .map(|occupant| occupant.get_fields())
            .unwrap_or_default();
        copy.insert(
            LogicSlotType::Occupied,
            LogicField {
                field_type: MemoryAccess::Read,
                value: if self.occupant.is_some() { 1.0 } else { 0.0 },
            },
        );
        copy.insert(
            LogicSlotType::OccupantHash,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.prefab_hash as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            LogicSlotType::Quantity,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.quantity as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            LogicSlotType::Damage,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.damage)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            LogicSlotType::Class,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self.typ as u32 as f64,
            },
        );
        copy.insert(
            LogicSlotType::MaxQuantity,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.max_quantity as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            LogicSlotType::PrefabHash,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.prefab_hash as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            LogicSlotType::SortingClass,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.sorting_class as u32 as f64)
                    .unwrap_or(0.0),
            },
        );
        copy.insert(
            LogicSlotType::ReferenceId,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self
                    .occupant
                    .as_ref()
                    .map(|occupant| occupant.id as f64)
                    .unwrap_or(0.0),
            },
        );
        copy
    }

    pub fn get_field(&self, field: LogicSlotType) -> f64 {
        let fields = self.get_fields();
        fields
            .get(&field)
            .map(|field| match field.field_type {
                MemoryAccess::Read | MemoryAccess::ReadWrite => field.value,
                _ => 0.0,
            })
            .unwrap_or(0.0)
    }

    pub fn can_logic_read(&self, field: LogicSlotType) -> bool {
        match field {
            LogicSlotType::Pressure | LogicSlotType::Temperature | LogicSlotType::Volume => {
                matches!(
                    self.typ,
                    SlotClass::GasCanister | SlotClass::LiquidCanister | SlotClass::LiquidBottle
                )
            }
            LogicSlotType::Charge | LogicSlotType::ChargeRatio => {
                matches!(self.typ, SlotClass::Battery)
            }
            LogicSlotType::Open => matches!(
                self.typ,
                SlotClass::Helmet | SlotClass::Tool | SlotClass::Appliance
            ),
            LogicSlotType::Lock => matches!(self.typ, SlotClass::Helmet),
            LogicSlotType::FilterType => matches!(self.typ, SlotClass::GasFilter),
            _ => {
                if let Some(occupant) = self.occupant.as_ref() {
                    occupant.can_logic_read(field)
                } else {
                    false
                }
            }
        }
    }

    pub fn can_logic_write(&self, field: LogicSlotType) -> bool {
        match field {
            LogicSlotType::Open => matches!(
                self.typ,
                SlotClass::Helmet
                    | SlotClass::GasCanister
                    | SlotClass::LiquidCanister
                    | SlotClass::LiquidBottle
            ),
            LogicSlotType::On => matches!(
                self.typ,
                SlotClass::Helmet | SlotClass::Tool | SlotClass::Appliance
            ),
            LogicSlotType::Lock => matches!(self.typ, SlotClass::Helmet),
            _ => {
                if let Some(occupant) = self.occupant.as_ref() {
                    occupant.can_logic_write(field)
                } else {
                    false
                }
            }
        }
    }

    pub fn set_field(&mut self, typ: LogicSlotType, val: f64, force: bool) -> Result<(), ICError> {
        if matches!(
            typ,
            LogicSlotType::Occupied
                | LogicSlotType::OccupantHash
                | LogicSlotType::Class
                | LogicSlotType::PrefabHash
                | LogicSlotType::SortingClass
                | LogicSlotType::ReferenceId
        ) {
            return Err(ICError::ReadOnlyField(typ.to_string()));
        }
        if let Some(occupant) = self.occupant.as_mut() {
            occupant.set_field(typ, val, force)
        } else {
            Err(ICError::SlotNotOccupied)
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Prefab {
    pub name: String,
    pub hash: i32,
}

impl Prefab {
    pub fn new(name: &str) -> Self {
        Prefab {
            name: name.to_owned(),
            hash: const_crc32::crc32(name.as_bytes()) as i32,
        }
    }
}

#[derive(Debug, Default)]
pub struct Device {
    pub id: u32,
    pub name: Option<String>,
    pub name_hash: Option<i32>,
    pub prefab: Option<Prefab>,
    pub slots: Vec<Slot>,
    pub reagents: BTreeMap<ReagentMode, BTreeMap<i32, f64>>,
    pub ic: Option<u32>,
    pub connections: Vec<Connection>,
    fields: BTreeMap<LogicType, LogicField>,
}

impl Device {
    pub fn new(id: u32) -> Self {
        Device {
            id,
            name: None,
            name_hash: None,
            prefab: None,
            fields: BTreeMap::new(),
            slots: Vec::new(),
            reagents: BTreeMap::new(),
            ic: None,
            connections: vec![Connection::CableNetwork {
                net: None,
                typ: CableConnectionType::default(),
            }],
        }
    }

    pub fn with_ic(id: u32, ic: u32) -> Self {
        let mut device = Device::new(id);
        device.ic = Some(ic);
        device.connections = vec![
            Connection::CableNetwork {
                net: None,
                typ: CableConnectionType::Data,
            },
            Connection::CableNetwork {
                net: None,
                typ: CableConnectionType::Power,
            },
        ];
        device.prefab = Some(Prefab::new("StructureCircuitHousing"));
        device.fields.extend(vec![
            (
                LogicType::Setting,
                LogicField {
                    field_type: MemoryAccess::ReadWrite,
                    value: 0.0,
                },
            ),
            (
                LogicType::RequiredPower,
                LogicField {
                    field_type: MemoryAccess::Read,
                    value: 0.0,
                },
            ),
            (
                LogicType::PrefabHash,
                LogicField {
                    field_type: MemoryAccess::Read,
                    value: -128473777.0,
                },
            ),
        ]);
        let occupant = SlotOccupant::new(ic, -744098481);
        device.slots.push(Slot::with_occupant(
            SlotClass::ProgrammableChip,
            // -744098481 = ItemIntegratedCircuit10
            occupant,
        ));

        device
    }

    pub fn get_fields(&self, vm: &VM) -> BTreeMap<LogicType, LogicField> {
        let mut copy = self.fields.clone();
        if let Some(ic_id) = &self.ic {
            let ic = vm.circuit_holders.get(ic_id).expect("our own ic to exist").borrow();
            copy.insert(
                LogicType::LineNumber,
                LogicField {
                    field_type: MemoryAccess::ReadWrite,
                    value: ic.ip() as f64,
                },
            );
            copy.insert(
                LogicType::Error,
                LogicField {
                    field_type: MemoryAccess::Read,
                    value: match *ic.state.borrow() {
                        ICState::Error(_) => 1.0,
                        _ => 0.0,
                    },
                },
            );
        }
        if self.has_power_state() {
            copy.insert(
                LogicType::Power,
                LogicField {
                    field_type: MemoryAccess::Read,
                    value: if self.has_power_connection() {
                        1.0
                    } else {
                        0.0
                    },
                },
            );
        }
        copy.insert(
            LogicType::ReferenceId,
            LogicField {
                field_type: MemoryAccess::Read,
                value: self.id as f64,
            },
        );
        copy
    }

    pub fn get_network_id(&self, connection: usize) -> Result<u32, ICError> {
        if connection >= self.connections.len() {
            Err(ICError::ConnectionIndexOutOfRange(
                connection,
                self.connections.len(),
            ))
        } else if let Connection::CableNetwork {
            net: network_id, ..
        } = self.connections[connection]
        {
            if let Some(network_id) = network_id {
                Ok(network_id)
            } else {
                Err(ICError::NetworkNotConnected(connection))
            }
        } else {
            Err(ICError::NotACableConnection(connection))
        }
    }

    pub fn can_logic_read(&self, field: LogicType) -> bool {
        match field {
            LogicType::ReferenceId => true,
            LogicType::LineNumber | LogicType::Error if self.ic.is_some() => true,
            LogicType::Power if self.has_power_state() => true,
            _ => {
                if let Some(logic) = self.fields.get(&field) {
                    matches!(
                        logic.field_type,
                        MemoryAccess::Read | MemoryAccess::ReadWrite
                    )
                } else {
                    false
                }
            }
        }
    }

    pub fn can_logic_write(&self, field: LogicType) -> bool {
        match field {
            LogicType::ReferenceId => false,
            LogicType::LineNumber if self.ic.is_some() => true,
            _ => {
                if let Some(logic) = self.fields.get(&field) {
                    matches!(
                        logic.field_type,
                        MemoryAccess::Write | MemoryAccess::ReadWrite
                    )
                } else {
                    false
                }
            }
        }
    }

    pub fn can_slot_logic_read(&self, field: LogicSlotType, slot: usize) -> bool {
        if self.slots.is_empty() {
            return false;
        }
        let Some(slot) = self.slots.get(slot) else {
            return false;
        };
        slot.can_logic_read(field)
    }

    pub fn can_slot_logic_write(&self, field: LogicSlotType, slot: usize) -> bool {
        if self.slots.is_empty() {
            return false;
        }
        let Some(slot) = self.slots.get(slot) else {
            return false;
        };
        slot.can_logic_write(field)
    }

    pub fn get_field(&self, typ: LogicType, vm: &VM) -> Result<f64, ICError> {
        if typ == LogicType::LineNumber && self.ic.is_some() {
            let ic = vm
                .circuit_holders
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .borrow();
            Ok(ic.ip() as f64)
        } else if let Some(field) = self.get_fields(vm).get(&typ) {
            if field.field_type == MemoryAccess::Read || field.field_type == MemoryAccess::ReadWrite
            {
                Ok(field.value)
            } else {
                Err(ICError::WriteOnlyField(typ.to_string()))
            }
        } else {
            Err(ICError::DeviceHasNoField(typ.to_string()))
        }
    }

    pub fn set_field(
        &mut self,
        typ: LogicType,
        val: f64,
        vm: &VM,
        force: bool,
    ) -> Result<(), ICError> {
        if typ == LogicType::ReferenceId
            || (typ == LogicType::Error && self.ic.is_some())
            || (typ == LogicType::Power && self.has_power_state())
        {
            Err(ICError::ReadOnlyField(typ.to_string()))
        } else if typ == LogicType::LineNumber && self.ic.is_some() {
            let ic = vm
                .circuit_holders
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .borrow();
            ic.set_ip(val as u32);
            Ok(())
        } else if let Some(field) = self.fields.get_mut(&typ) {
            if field.field_type == MemoryAccess::Write
                || field.field_type == MemoryAccess::ReadWrite
                || force
            {
                field.value = val;
                Ok(())
            } else {
                Err(ICError::ReadOnlyField(typ.to_string()))
            }
        } else if force {
            self.fields.insert(
                typ,
                LogicField {
                    field_type: MemoryAccess::ReadWrite,
                    value: val,
                },
            );
            Ok(())
        } else {
            Err(ICError::DeviceHasNoField(typ.to_string()))
        }
    }

    pub fn get_slot_field(&self, index: f64, typ: LogicSlotType, vm: &VM) -> Result<f64, ICError> {
        let slot = self
            .slots
            .get(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?;
        if slot.typ == SlotClass::ProgrammableChip
            && slot.occupant.is_some()
            && self.ic.is_some()
            && typ == LogicSlotType::LineNumber
        {
            let ic = vm
                .circuit_holders
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .borrow();
            Ok(ic.ip() as f64)
        } else {
            Ok(slot.get_field(typ))
        }
    }

    pub fn get_slot_fields(
        &self,
        index: f64,
        vm: &VM,
    ) -> Result<BTreeMap<LogicSlotType, LogicField>, ICError> {
        let slot = self
            .slots
            .get(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?;
        let mut fields = slot.get_fields();
        if slot.typ == SlotClass::ProgrammableChip && slot.occupant.is_some() && self.ic.is_some() {
            let ic = vm
                .circuit_holders
                .get(&self.ic.unwrap())
                .ok_or_else(|| ICError::UnknownDeviceID(self.ic.unwrap() as f64))?
                .borrow();
            fields.insert(
                LogicSlotType::LineNumber,
                LogicField {
                    field_type: MemoryAccess::ReadWrite,
                    value: ic.ip() as f64,
                },
            );
        }
        Ok(fields)
    }

    pub fn set_slot_field(
        &mut self,
        index: f64,
        typ: LogicSlotType,
        val: f64,
        _vm: &VM,
        force: bool,
    ) -> Result<(), ICError> {
        let slot = self
            .slots
            .get_mut(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))?;
        slot.set_field(typ, val, force)
    }

    pub fn get_slot(&self, index: f64) -> Result<&Slot, ICError> {
        self.slots
            .get(index as usize)
            .ok_or(ICError::SlotIndexOutOfRange(index))
    }

    pub fn get_reagent(&self, rm: &ReagentMode, reagent: f64) -> f64 {
        if let Some(mode) = self.reagents.get(rm) {
            if let Some(val) = mode.get(&(reagent as i32)) {
                return *val;
            }
        }
        0.0
    }

    pub fn set_name(&mut self, name: &str) {
        self.name_hash = Some(const_crc32::crc32(name.as_bytes()) as i32);
        self.name = Some(name.to_owned());
    }

    pub fn has_power_state(&self) -> bool {
        self.connections.iter().any(|conn| {
            matches!(
                conn,
                Connection::CableNetwork {
                    typ: CableConnectionType::Power | CableConnectionType::PowerAndData,
                    ..
                }
            )
        })
    }

    pub fn has_power_connection(&self) -> bool {
        self.connections.iter().any(|conn| {
            matches!(
                conn,
                Connection::CableNetwork {
                    net: Some(_),
                    typ: CableConnectionType::Power | CableConnectionType::PowerAndData,
                }
            )
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeviceTemplate {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub prefab_name: Option<String>,
    pub slots: Vec<SlotTemplate>,
    // pub reagents: BTreeMap<ReagentMode, BTreeMap<i32, f64>>,
    pub connections: Vec<Connection>,
    pub fields: BTreeMap<LogicType, LogicField>,
}

impl Device {
    /// create a devive from a template and return the device, does not create it's own IC
    pub fn from_template<F>(template: DeviceTemplate, mut id_fn: F) -> Self
    where
        F: FnMut() -> u32,
    {
        // id_fn *must* be captured not moved
        #[allow(clippy::redundant_closure)]
        let device_id = template.id.unwrap_or_else(|| id_fn());
        let name_hash = template
            .name
            .as_ref()
            .map(|name| const_crc32::crc32(name.as_bytes()) as i32);

        #[allow(clippy::redundant_closure)]
        let slots = template
            .slots
            .into_iter()
            .map(|slot| Slot {
                typ: slot.typ,
                occupant: slot
                    .occupant
                    .map(|occupant| SlotOccupant::from_template(occupant, || id_fn())),
            })
            .collect_vec();

        let ic = slots
            .iter()
            .find_map(|slot| {
                if slot.typ == SlotClass::ProgrammableChip && slot.occupant.is_some() {
                    Some(slot.occupant.clone()).flatten()
                } else {
                    None
                }
            })
            .map(|occupant| occupant.id);

        let fields = template.fields;

        Device {
            id: device_id,
            name: template.name,
            name_hash,
            prefab: template.prefab_name.map(|name| Prefab::new(&name)),
            slots,
            // reagents: template.reagents,
            reagents: BTreeMap::new(),
            ic,
            connections: template.connections,
            fields,
        }
    }
}

impl<T> From<T> for DeviceTemplate
where
    T: Deref<Target = Device>,
{
    fn from(device: T) -> Self {
        DeviceTemplate {
            id: Some(device.id),
            name: device.name.clone(),
            prefab_name: device.prefab.as_ref().map(|prefab| prefab.name.clone()),
            slots: device
                .slots
                .iter()
                .map(|slot| SlotTemplate {
                    typ: slot.typ,
                    occupant: slot.occupant.as_ref().map(|occupant| SlotOccupantTemplate {
                        id: Some(occupant.id),
                        fields: occupant.get_fields(),
                    }),
                })
                .collect_vec(),
            connections: device.connections.clone(),
            fields: device.fields.clone(),
        }
    }
}
