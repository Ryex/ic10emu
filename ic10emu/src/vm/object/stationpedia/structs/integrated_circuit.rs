use crate::{
    errors::ICError,
    interpreter::{instructions::IC10Marker, ICState, Program},
    vm::{
        instructions::{operands::Operand, Instruction},
        object::{
            errors::{LogicError, MemoryError},
            macros::ObjectInterface,
            traits::*,
            LogicField, MemoryAccess, Name, ObjectID, Slot, VMObject,
        },
        VM,
    },
};
use macro_rules_attribute::derive;
use stationeers_data::enums::{
    basic_enums::{Class as SlotClass, GasType, SortingClass},
    script_enums::{LogicSlotType, LogicType},
};
use std::{collections::BTreeMap, rc::Rc};

static RETURN_ADDRESS_INDEX: usize = 17;
static STACK_POINTER_INDEX: usize = 16;

#[derive(ObjectInterface!)]
#[custom(implements(Object { Item, Storage, Logicable, MemoryReadable, MemoryWritable }))]
pub struct ItemIntegratedCircuit10 {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
    #[custom(object_vm_ref)]
    pub vm: Rc<VM>,
    pub fields: BTreeMap<LogicType, LogicField>,
    pub memory: [f64; 512],
    pub parent_slot: Option<ParentSlotInfo>,
    pub registers: [f64; 18],
    /// Instruction Pointer
    pub ip: usize,
    pub next_ip: usize,
    /// Instruction Count since last yield
    pub ic: u16,
    pub aliases: BTreeMap<String, Operand>,
    pub defines: BTreeMap<String, f64>,
    pub pins: [Option<u32>; 6],
    pub state: ICState,
    pub code: String,
    pub program: Program,
    pub damage: f32,
}

impl Item for ItemIntegratedCircuit10 {
    fn consumable(&self) -> bool {
        false
    }
    fn filter_type(&self) -> Option<GasType> {
        None
    }
    fn ingredient(&self) -> bool {
        false
    }
    fn max_quantity(&self) -> u32 {
        1
    }
    fn reagents(&self) -> Option<&BTreeMap<String, f64>> {
        None
    }
    fn slot_class(&self) -> SlotClass {
        SlotClass::ProgrammableChip
    }
    fn sorting_class(&self) -> SortingClass {
        SortingClass::Default
    }
    fn get_parent_slot(&self) -> Option<ParentSlotInfo> {
        self.parent_slot
    }
    fn set_parent_slot(&mut self, info: Option<ParentSlotInfo>) {
        self.parent_slot = info;
    }
    fn get_damage(&self) -> f32 {
        self.damage
    }
    fn set_damage(&mut self, damage: f32) {
        self.damage = damage;
    }
}

impl Storage for ItemIntegratedCircuit10 {
    fn slots_count(&self) -> usize {
        0
    }
    fn get_slot(&self, _index: usize) -> Option<&Slot> {
        None
    }
    fn get_slot_mut(&mut self, _index: usize) -> Option<&mut Slot> {
        None
    }
    fn get_slots(&self) -> Vec<&Slot> {
        vec![]
    }
    fn get_slots_mut(&mut self) -> Vec<&mut Slot> {
        vec![]
    }
}

impl Logicable for ItemIntegratedCircuit10 {
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
        self.fields
            .get(&lt)
            .map(|field| {
                matches!(
                    field.field_type,
                    MemoryAccess::Read | MemoryAccess::ReadWrite
                )
            })
            .unwrap_or(false)
    }
    fn can_logic_write(&self, lt: LogicType) -> bool {
        self.fields
            .get(&lt)
            .map(|field| {
                matches!(
                    field.field_type,
                    MemoryAccess::Write | MemoryAccess::ReadWrite
                )
            })
            .unwrap_or(false)
    }
    fn get_logic(&self, lt: LogicType) -> Result<f64, LogicError> {
        self.fields
            .get(&lt)
            .and_then(|field| match field.field_type {
                MemoryAccess::Read | MemoryAccess::ReadWrite => Some(field.value),
                _ => None,
            })
            .ok_or(LogicError::CantRead(lt))
    }
    fn set_logic(&mut self, lt: LogicType, value: f64, force: bool) -> Result<(), LogicError> {
        self.fields
            .get_mut(&lt)
            .ok_or(LogicError::CantWrite(lt))
            .and_then(|field| match field.field_type {
                MemoryAccess::Write | MemoryAccess::ReadWrite => {
                    field.value = value;
                    Ok(())
                }
                _ if force => {
                    field.value = value;
                    Ok(())
                }
                _ => Err(LogicError::CantWrite(lt)),
            })
    }
    fn can_slot_logic_read(&self, _slt: LogicSlotType, _indexx: f64) -> bool {
        false
    }
    fn get_slot_logic(&self, _slt: LogicSlotType, index: f64) -> Result<f64, LogicError> {
        Err(LogicError::SlotIndexOutOfRange(index, self.slots_count()))
    }
    fn valid_logic_types(&self) -> Vec<LogicType> {
        self.fields.keys().copied().collect()
    }
    fn known_modes(&self) -> Option<Vec<(u32, String)>> {
        None
    }
}

impl MemoryReadable for ItemIntegratedCircuit10 {
    fn memory_size(&self) -> usize {
        self.memory.len()
    }
    fn get_memory(&self, index: i32) -> Result<f64, MemoryError> {
        if index < 0 {
            Err(MemoryError::StackUnderflow(index, self.memory.len()))
        } else if index as usize >= self.memory.len() {
            Err(MemoryError::StackOverflow(index, self.memory.len()))
        } else {
            Ok(self.memory[index as usize])
        }
    }
    fn get_memory_slice(&self) -> &[f64] {
        &self.memory
    }
}

impl MemoryWritable for ItemIntegratedCircuit10 {
    fn set_memory(&mut self, index: i32, val: f64) -> Result<(), MemoryError> {
        if index < 0 {
            Err(MemoryError::StackUnderflow(index, self.memory.len()))
        } else if index as usize >= self.memory.len() {
            Err(MemoryError::StackOverflow(index, self.memory.len()))
        } else {
            self.memory[index as usize] = val;
            Ok(())
        }
    }
    fn clear_memory(&mut self) -> Result<(), MemoryError> {
        self.memory.fill(0.0);
        Ok(())
    }
}

impl SourceCode for ItemIntegratedCircuit10 {
    fn set_source_code(&mut self, code: &str) -> Result<(), ICError> {
        self.program = Program::try_from_code(code)?;
        self.code = code.to_string();
        Ok(())
    }
    fn set_source_code_with_invalid(&mut self, code: &str) {
        self.program = Program::from_code_with_invalid(code);
        self.code = code.to_string();
    }
    fn get_source_code(&self) -> String {
        self.code.clone()
    }
    fn get_line(&self, line: usize) -> Result<&Instruction, ICError> {
        self.program.get_line(line)
    }
}

impl IntegratedCircuit for ItemIntegratedCircuit10 {
    fn get_circuit_holder(&self) -> Option<VMObject> {
        self.get_parent_slot()
            .and_then(|parent_slot| self.get_vm().get_object(parent_slot.parent))
    }
    fn get_instruction_pointer(&self) -> f64 {
        self.ip as f64
    }
    fn set_next_instruction(&mut self, next_instruction: f64) {
        self.next_ip = next_instruction as usize;
    }
    fn set_next_instruction_relative(&mut self, offset: f64) {
        self.next_ip = (self.ip as f64 + offset) as usize
    }
    fn reset(&mut self) {
        self.ip = 0;
        self.ic = 0;
        self.registers.fill(0.0);
        self.memory.fill(0.0);
        self.aliases.clear();
        self.defines.clear();
        self.state = ICState::Start;
    }
    fn get_real_target(&self, indirection: u32, target: u32) -> Result<f64, ICError> {
        let mut i = indirection;
        let mut t = target as f64;
        while i > 0 {
            if let Some(new_t) = self.registers.get(t as usize) {
                t = *new_t;
            } else {
                return Err(ICError::RegisterIndexOutOfRange(t));
            }
            i -= 1;
        }
        Ok(t)
    }
    fn get_register(&self, indirection: u32, target: u32) -> Result<f64, ICError> {
        let t = self.get_real_target(indirection, target)?;
        self.registers
            .get(t as usize)
            .ok_or(ICError::RegisterIndexOutOfRange(t))
            .copied()
    }
    fn set_register(&mut self, indirection: u32, target: u32, val: f64) -> Result<f64, ICError> {
        let t = self.get_real_target(indirection, target)?;
        let old_val = self
            .registers
            .get(t as usize)
            .ok_or(ICError::RegisterIndexOutOfRange(t))
            .copied()?;
        self.registers[t as usize] = val;
        Ok(old_val)
    }
    fn set_return_address(&mut self, addr: f64) {
        self.registers[RETURN_ADDRESS_INDEX] = addr;
    }
    fn push_stack(&mut self, val: f64) -> Result<f64, ICError> {
        let sp = (self.registers[STACK_POINTER_INDEX].round()) as i32;
        if sp < 0 {
            Err(MemoryError::StackUnderflow(sp, self.memory.len()).into())
        } else if sp as usize >= self.memory.len() {
            Err(MemoryError::StackOverflow(sp, self.memory.len()).into())
        } else {
            let last = self.memory[sp as usize];
            self.memory[sp as usize] = val;
            self.registers[STACK_POINTER_INDEX] += 1.0;
            Ok(last)
        }
    }
    fn pop_stack(&mut self) -> Result<f64, ICError> {
        self.registers[STACK_POINTER_INDEX] -= 1.0;
        let sp = (self.registers[STACK_POINTER_INDEX].round()) as i32;
        if sp < 0 {
            Err(MemoryError::StackUnderflow(sp, self.memory.len()).into())
        } else if sp as usize >= self.memory.len() {
            Err(MemoryError::StackOverflow(sp, self.memory.len()).into())
        } else {
            let last = self.memory[sp as usize];
            Ok(last)
        }
    }
    fn peek_stack(&self) -> Result<f64, ICError> {
        let sp = (self.registers[STACK_POINTER_INDEX] - 1.0).round() as i32;
        if sp < 0 {
            Err(MemoryError::StackUnderflow(sp, self.memory.len()).into())
        } else if sp as usize >= self.memory.len() {
            Err(MemoryError::StackOverflow(sp, self.memory.len()).into())
        } else {
            let last = self.memory[sp as usize];
            Ok(last)
        }
    }
    fn get_stack(&self, addr: f64) -> Result<f64, ICError> {
        let sp = (addr) as i32;
        if !(0..(self.memory.len() as i32)).contains(&sp) {
            Err(ICError::StackIndexOutOfRange(addr))
        } else {
            let val = self.memory[sp as usize];
            Ok(val)
        }
    }
    fn put_stack(&mut self, addr: f64, val: f64) -> Result<f64, ICError> {
        let sp = addr.round() as i32;
        if !(0..(self.memory.len() as i32)).contains(&sp) {
            Err(ICError::StackIndexOutOfRange(addr))
        } else {
            let last = self.memory[sp as usize];
            self.memory[sp as usize] = val;
            Ok(last)
        }
    }
    fn get_aliases(&self) -> &BTreeMap<String, Operand> {
        &self.aliases
    }
    fn get_aliases_mut(&mut self) -> &mut BTreeMap<String, Operand> {
        &mut self.aliases
    }
    fn get_defines(&self) -> &BTreeMap<String, f64> {
        &self.defines
    }
    fn get_defines_mut(&mut self) -> &mut BTreeMap<String, f64> {
        &mut self.defines
    }
    fn get_labels(&self) -> &BTreeMap<String, u32> {
        &self.program.labels
    }
    fn get_state(&self) -> crate::interpreter::ICState {
        self.state.clone()
    }
    fn set_state(&mut self, state: crate::interpreter::ICState) {
        self.state = state;
    }
}

impl IC10Marker for ItemIntegratedCircuit10 {}

impl Programmable for ItemIntegratedCircuit10 {
    fn step(&mut self, advance_ip_on_err: bool) -> Result<(), crate::errors::ICError> {
        if matches!(&self.state, ICState::HasCaughtFire) {
            return Ok(());
        }
        if matches!(&self.state, ICState::Error(_)) && !advance_ip_on_err {
            return Ok(());
        }
        if let ICState::Sleep(then, sleep_for) = &self.state {
            if let Some(duration) = time::Duration::checked_seconds_f64(*sleep_for) {
                if let Some(sleep_till) = then.checked_add(duration) {
                    if sleep_till
                        <= time::OffsetDateTime::now_local()
                            .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
                    {
                        return Ok(());
                    }
                    // else sleep duration ended, continue
                } else {
                    return Err(ICError::SleepAddtionError(duration, *then));
                }
            } else {
                return Err(ICError::SleepDurationError(*sleep_for));
            }
        }
        if self.ip >= self.program.len() || self.program.is_empty() {
            self.state = ICState::Ended;
            return Ok(());
        }
        self.next_ip = self.ip + 1;
        self.state = ICState::Running;
        let line = self.program.get_line(self.ip)?.clone();
        let operands = &line.operands;
        let instruction = line.instruction;
        let result = instruction.execute(self, operands);

        let was_error = if let Err(_err) = result {
            self.get_circuit_holder()
                .ok_or(ICError::NoCircuitHolder(self.id))?
                .borrow_mut()
                .as_mut_circuit_holder()
                .ok_or(ICError::CircuitHolderNotLogicable(self.id))?
                .set_error(1);
            true
        } else {
            false
        };

        if !was_error || advance_ip_on_err {
            self.ip = self.next_ip;
            if self.ip >= self.program.len() {
                self.state = ICState::Ended;
            }
        }

        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(self.id))?
            .borrow_mut()
            .as_mut_logicable()
            .ok_or(ICError::CircuitHolderNotLogicable(self.id))?
            .set_logic(LogicType::LineNumber, self.ip as f64, true)?;

        Ok(())
    }
}
