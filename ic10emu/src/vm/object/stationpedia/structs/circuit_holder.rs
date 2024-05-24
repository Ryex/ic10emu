use crate::{
    network::{CableConnectionType, Connection},
    vm::{
        object::{
            errors::LogicError, macros::ObjectInterface, traits::*, Name, ObjectID, Slot, VMObject,
        },
        VM,
    },
};
use macro_rules_attribute::derive;
use stationeers_data::enums::{
    basic::Class as SlotClass,
    prefabs::StationpediaPrefab,
    script::{LogicSlotType, LogicType},
    ConnectionRole,
};
use std::rc::Rc;
use strum::EnumProperty;

#[derive(ObjectInterface!)]
#[custom(implements(Object { Structure, Device, Storage, Logicable, CircuitHolder }))]
pub struct StructureCircuitHousing {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub error: i32,
    pub on: bool,
    pub setting: f64,
    pub slot: Slot,
    pub pins: [Option<ObjectID>; 6],
    pub connections: [crate::network::Connection; 2],
}

#[allow(dead_code)]
impl StructureCircuitHousing {
    pub fn new(id: ObjectID, vm: Rc<VM>) -> Self {
        StructureCircuitHousing {
            id,
            prefab: Name {
                value: StationpediaPrefab::StructureCircuitHousing.to_string(),
                hash: StationpediaPrefab::StructureCircuitHousing as i32,
            },
            name: Name::new(
                StationpediaPrefab::StructureCircuitHousing
                    .get_str("name")
                    .unwrap(),
            ),
            vm,
            error: 0,
            on: true,
            setting: 0.0,
            slot: Slot {
                parent: id,
                index: 0,
                name: "Programmable Chip".to_string(),
                typ: SlotClass::ProgrammableChip,
                readable_logic: vec![
                    LogicSlotType::Class,
                    LogicSlotType::Damage,
                    LogicSlotType::LineNumber,
                    LogicSlotType::MaxQuantity,
                    LogicSlotType::OccupantHash,
                    LogicSlotType::Occupied,
                    LogicSlotType::PrefabHash,
                    LogicSlotType::Quantity,
                    LogicSlotType::ReferenceId,
                    LogicSlotType::SortingClass,
                ],
                writeable_logic: vec![],
                occupant: None,
            },
            pins: [None, None, None, None, None, None],
            connections: [
                Connection::CableNetwork {
                    net: None,
                    typ: CableConnectionType::Data,
                    role: ConnectionRole::Input,
                },
                Connection::CableNetwork {
                    net: None,
                    typ: CableConnectionType::Power,
                    role: ConnectionRole::None,
                },
            ],
        }
    }
}

impl Structure for StructureCircuitHousing {
    fn is_small_grid(&self) -> bool {
        true
    }
}

impl Storage for StructureCircuitHousing {
    fn slots_count(&self) -> usize {
        1
    }
    fn get_slot(&self, index: usize) -> Option<&Slot> {
        if index != 0 {
            None
        } else {
            Some(&self.slot)
        }
    }
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot> {
        if index != 0 {
            None
        } else {
            Some(&mut self.slot)
        }
    }
    fn get_slots(&self) -> Vec<&Slot> {
        vec![&self.slot]
    }
    fn get_slots_mut(&mut self) -> Vec<&mut Slot> {
        vec![&mut self.slot]
    }
}

impl Logicable for StructureCircuitHousing {
    fn prefab_hash(&self) -> i32 {
        self.get_prefab().hash
    }
    fn name_hash(&self) -> i32 {
        self.get_name().hash
    }
    fn is_logic_readable(&self) -> bool {
        true
    }
    fn is_logic_writeable(&self) -> bool {
        true
    }
    fn can_logic_read(&self, lt: LogicType) -> bool {
        use LogicType::*;
        matches!(
            lt,
            Error
                | LineNumber
                | NameHash
                | On
                | Power
                | PrefabHash
                | ReferenceId
                | RequiredPower
                | Setting
        )
    }
    fn can_logic_write(&self, lt: LogicType) -> bool {
        use LogicType::*;
        matches!(lt, LineNumber | On | Setting)
    }
    fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError> {
        match lt {
            LogicType::PrefabHash => Ok(self.get_prefab().hash as f64),
            LogicType::NameHash => Ok(self.get_name().hash as f64),
            LogicType::ReferenceId => Ok(*self.get_id() as f64),
            LogicType::Error => Ok(self.error as f64),
            LogicType::LineNumber => {
                let result = self.slot.occupant.and_then(|info| {
                    self.vm.get_object(info.id).and_then(|obj| {
                        obj.borrow()
                            .as_logicable()
                            .map(|logicable| logicable.get_logic(LogicType::LineNumber))
                    })
                });
                result.unwrap_or(Ok(0.0))
            }
            LogicType::On => Ok(self.on as i32 as f64),
            LogicType::Power => {
                if let Connection::CableNetwork { net, .. } = self.connections[1] {
                    if net.is_some() {
                        Ok(1.0)
                    } else {
                        Ok(0.0)
                    }
                } else {
                    Ok(0.0)
                }
            }
            LogicType::RequiredPower => {
                if let Connection::CableNetwork { net, .. } = self.connections[1] {
                    if net.is_some() {
                        if self.on {
                            Ok(10.0)
                        } else {
                            Ok(0.0)
                        }
                    } else {
                        Ok(-1.0)
                    }
                } else {
                    Ok(-1.0)
                }
            } // 10 if on
            LogicType::Setting => Ok(self.setting),
            _ => Err(LogicError::CantRead(lt)),
        }
    }
    fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError> {
        match lt {
            LogicType::LineNumber => self
                .slot
                .occupant
                .and_then(|info| {
                    self.vm.get_object(info.id).and_then(|obj| {
                        obj.borrow_mut().as_mut_logicable().map(|logicable| {
                            logicable.set_logic(LogicType::LineNumber, value, force)
                        })
                    })
                })
                .unwrap_or(Err(LogicError::CantWrite(lt))),
            LogicType::On => {
                self.on = value != 0.0;
                Ok(())
            }
            LogicType::Setting => {
                self.setting = value;
                Ok(())
            }
            _ => Err(LogicError::CantWrite(lt)),
        }
    }
    fn can_slot_logic_read(&self, _slt: LogicSlotType, _indexx: f64) -> bool {
        false
    }
    fn get_slot_logic(&self, _slt: LogicSlotType, index: f64) -> Result<f64, LogicError> {
        Err(LogicError::SlotIndexOutOfRange(index, self.slots_count()))
    }
    fn valid_logic_types(&self) -> Vec<LogicType> {
        use LogicType::*;
        vec![
            Error,
            LineNumber,
            NameHash,
            On,
            Power,
            PrefabHash,
            ReferenceId,
            RequiredPower,
            Setting,
        ]
    }
    fn known_modes(&self) -> Option<Vec<(u32, String)>> {
        None
    }
}

impl Device for StructureCircuitHousing {
    fn has_reagents(&self) -> bool {
        false
    }
    fn has_atmosphere(&self) -> bool {
        false
    }
    fn has_lock_state(&self) -> bool {
        false
    }
    fn has_mode_state(&self) -> bool {
        false
    }
    fn has_open_state(&self) -> bool {
        false
    }
    fn has_color_state(&self) -> bool {
        false
    }
    fn has_activate_state(&self) -> bool {
        false
    }
    fn has_on_off_state(&self) -> bool {
        true
    }
    fn get_reagents(&self) -> Vec<(i32, f64)> {
        vec![]
    }
    fn set_reagents(&mut self, _reagents: &[(i32, f64)]) {
        // nope
    }
    fn add_reagents(&mut self, _reagents: &[(i32, f64)]) {
        // nope
    }
    fn connection_list(&self) -> &[crate::network::Connection] {
        &self.connections
    }
    fn connection_list_mut(&mut self) -> &mut [crate::network::Connection] {
        &mut self.connections
    }
    fn device_pins(&self) -> Option<&[Option<ObjectID>]> {
        Some(&self.pins)
    }
    fn device_pins_mut(&mut self) -> Option<&mut [Option<ObjectID>]> {
        Some(&mut self.pins)
    }
    fn can_slot_logic_write(&self, _slt: LogicSlotType, _index: f64) -> bool {
        false
    }
    fn set_slot_logic(
        &mut self,
        slt: LogicSlotType,
        index: f64,
        _value: f64,
        _force: bool,
    ) -> Result<(), LogicError> {
        Err(LogicError::CantSlotWrite(slt, index))
    }
}

impl CircuitHolder for StructureCircuitHousing {
    fn clear_error(&mut self) {
        self.error = 0
    }
    fn set_error(&mut self, state: i32) {
        self.error = state;
    }
    /// i32::MAX is db
    fn get_logicable_from_index(
        &self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRef> {
        if device == i32::MAX {
            // self
            if let Some(connection) = connection {
                self.connections.get(connection).and_then(|conn| {
                    if let Connection::CableNetwork { net: Some(net), .. } = conn {
                        self.vm.get_network(*net).map(ObjectRef::from_vm_object)
                    } else {
                        None
                    }
                })
            } else {
                Some(ObjectRef::from_ref(self.as_object()))
            }
        } else {
            if device < 0 {
                return None;
            }
            self.pins.get(device as usize).and_then(|pin| {
                pin.and_then(|id| self.vm.get_object(id).map(ObjectRef::from_vm_object))
            })
        }
    }
    /// i32::MAX is db
    fn get_logicable_from_index_mut(
        &mut self,
        device: i32,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        if device == i32::MAX {
            // self
            if let Some(connection) = connection {
                self.connections.get(connection).and_then(|conn| {
                    if let Connection::CableNetwork { net: Some(net), .. } = conn {
                        self.vm.get_network(*net).map(ObjectRefMut::from_vm_object)
                    } else {
                        None
                    }
                })
            } else {
                Some(ObjectRefMut::from_ref(self.as_mut_object()))
            }
        } else {
            if device < 0 {
                return None;
            }
            self.pins.get(device as usize).and_then(|pin| {
                pin.and_then(|id| self.vm.get_object(id).map(ObjectRefMut::from_vm_object))
            })
        }
    }

    fn get_logicable_from_id(
        &self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRef> {
        if connection.is_some() {
            return None; // this functionality is disabled in the game, no network access via ReferenceId
        }
        if device == self.id {
            return Some(ObjectRef::from_ref(self.as_object()));
        }
        self.vm.get_object(device).map(ObjectRef::from_vm_object)
    }

    fn get_logicable_from_id_mut(
        &mut self,
        device: ObjectID,
        connection: Option<usize>,
    ) -> Option<ObjectRefMut> {
        if connection.is_some() {
            return None; // this functionality is disabled in the game, no network access via ReferenceId
        }
        if device == self.id {
            return Some(ObjectRefMut::from_ref(self.as_mut_object()));
        }
        self.vm.get_object(device).map(ObjectRefMut::from_vm_object)
    }

    fn get_ic(&self) -> Option<VMObject> {
        self.slot.occupant.and_then(|info| self.vm.get_object(info.id))
    }

    fn get_ic_mut(&self) -> Option<VMObject> {
        self.slot.occupant.and_then(|info| self.vm.get_object(info.id))
    }

    fn hault_and_catch_fire(&mut self) {
        // TODO: do something here??
    }
}
