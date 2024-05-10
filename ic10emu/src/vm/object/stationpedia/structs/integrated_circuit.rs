use crate::{
    errors::{ICError, LineError},
    grammar,
    vm::{
        enums::{
            basic_enums::{Class as SlotClass, GasType, SortingClass},
            script_enums::LogicType,
        },
        instructions::{
            enums::InstructionOp,
            operands::{DeviceSpec, Operand, RegisterSpec},
            traits::*,
            Instruction,
        },
        object::{
            errors::MemoryError,
            generic::{macros::GWLogicable, traits::GWLogicable},
            macros::ObjectInterface,
            traits::*,
            LogicField, Name, ObjectID, Slot,
        },
        VM,
    },
};
use itertools::Itertools;
use macro_rules_attribute::derive;
use serde_derive::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use ICError::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub errors: Vec<ICError>,
    pub labels: BTreeMap<String, u32>,
}

impl Default for Program {
    fn default() -> Self {
        Program::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: Vec::new(),
            errors: Vec::new(),
            labels: BTreeMap::new(),
        }
    }

    pub fn try_from_code(code: &str) -> Result<Self, ICError> {
        let parse_tree = grammar::parse(code)?;
        let mut labels_set = HashSet::new();
        let mut labels = BTreeMap::new();
        let errors = Vec::new();
        let instructions = parse_tree
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| match line.code {
                None => Ok(Instruction {
                    instruction: InstructionOp::Nop,
                    operands: vec![],
                }),
                Some(code) => match code {
                    grammar::Code::Label(label) => {
                        if labels_set.contains(&label.id.name) {
                            Err(ICError::DuplicateLabel(label.id.name))
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name, line_number as u32);
                            Ok(Instruction {
                                instruction: InstructionOp::Nop,
                                operands: vec![],
                            })
                        }
                    }
                    grammar::Code::Instruction(instruction) => Ok(instruction),
                    grammar::Code::Invalid(err) => Err(err.into()),
                },
            })
            .try_collect()?;
        Ok(Program {
            instructions,
            errors,
            labels,
        })
    }

    pub fn from_code_with_invalid(code: &str) -> Self {
        let parse_tree = grammar::parse_with_invalid(code);
        let mut labels_set = HashSet::new();
        let mut labels = BTreeMap::new();
        let mut errors = Vec::new();
        let instructions = parse_tree
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| match line.code {
                None => Instruction {
                    instruction: InstructionOp::Nop,
                    operands: vec![],
                },
                Some(code) => match code {
                    grammar::Code::Label(label) => {
                        if labels_set.contains(&label.id.name) {
                            errors.push(ICError::DuplicateLabel(label.id.name));
                        } else {
                            labels_set.insert(label.id.name.clone());
                            labels.insert(label.id.name, line_number as u32);
                        }
                        Instruction {
                            instruction: InstructionOp::Nop,
                            operands: vec![],
                        }
                    }
                    grammar::Code::Instruction(instruction) => instruction,
                    grammar::Code::Invalid(err) => {
                        errors.push(err.into());
                        Instruction {
                            instruction: InstructionOp::Nop,
                            operands: vec![],
                        }
                    }
                },
            })
            .collect_vec();
        Program {
            instructions,
            errors,
            labels,
        }
    }

    pub fn get_line(&self, line: usize) -> Result<&Instruction, ICError> {
        self.instructions
            .get(line)
            .ok_or(ICError::InstructionPointerOutOfRange(line))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ICState {
    Start,
    Running,
    Yield,
    Sleep(time::OffsetDateTime, f64),
    HasCaughtFire,
    Error(LineError),
}

static RETURN_ADDRESS_INDEX: usize = 17;
static STACK_POINTER_INDEX: usize = 16;

#[derive(ObjectInterface!, GWLogicable!)]
#[custom(implements(Object { Item, Storage, Logicable, MemoryReadable, MemoryWritable }))]
pub struct ItemIntegratedCircuit10 {
    #[custom(object_id)]
    pub id: ObjectID,
    #[custom(object_prefab)]
    pub prefab: Name,
    #[custom(object_name)]
    pub name: Name,
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
    fn parent_slot(&self) -> Option<ParentSlotInfo> {
        self.parent_slot
    }
}

impl Storage for ItemIntegratedCircuit10 {
    fn slots_count(&self) -> usize {
        0
    }
    fn get_slot(&self, index: usize) -> Option<&Slot> {
        None
    }
    fn get_slot_mut(&mut self, index: usize) -> Option<&mut Slot> {
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
    fn get_instruction_pointer(&self) -> f64 {
        self.ip as f64
    }
    fn set_next_instruction(&mut self, next_instruction: f64) {
        self.next_ip = next_instruction as usize;
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
            Err(ICError::StackUnderflow)
        } else if sp as usize >= self.memory.len() {
            Err(ICError::StackOverflow)
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
            Err(ICError::StackUnderflow)
        } else if sp as usize >= self.memory.len() {
            Err(ICError::StackOverflow)
        } else {
            let last = self.memory[sp as usize];
            Ok(last)
        }
    }
    fn peek_stack(&self) -> Result<f64, ICError> {
        let sp = (self.registers[STACK_POINTER_INDEX] - 1.0).round() as i32;
        if sp < 0 {
            Err(ICError::StackUnderflow)
        } else if sp as usize >= self.memory.len() {
            Err(ICError::StackOverflow)
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
    fn put_stack(&self, addr: f64, val: f64) -> Result<f64, ICError> {
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
    fn get_defines(&self) -> &BTreeMap<String, f64> {
        &self.defines
    }
    fn get_labels(&self) -> &BTreeMap<String, u32> {
        &self.program.labels
    }
}

impl SleepInstruction for ItemIntegratedCircuit10 {
    /// sleep a(r?|num)
    fn execute_sleep(&mut self, vm: &VM, a: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Sleep, 1)?;
        let now =
            time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        self.state = ICState::Sleep(now, a);
        Ok(())
    }
}

impl YieldInstruction for ItemIntegratedCircuit10 {
    /// yield
    fn execute_yield(&mut self, vm: &VM) -> Result<(), ICError> {
        self.state = ICState::Yield;
        Ok(())
    }
}

impl DefineInstruction for ItemIntegratedCircuit10 {
    /// define str num
    fn execute_define(&mut self, vm: &VM, string: &Operand, num: &Operand) -> Result<(), ICError> {
        let &Operand::Identifier(ident) = &string else {
            return Err(IncorrectOperandType {
                inst: InstructionOp::Define,
                index: 1,
                desired: "Name".to_owned(),
            });
        };
        let &Operand::Number(num) = &num else {
            return Err(IncorrectOperandType {
                inst: InstructionOp::Define,
                index: 2,
                desired: "Number".to_owned(),
            });
        };
        if self.defines.contains_key(&ident.name) {
            Err(DuplicateDefine(ident.name.clone()))
        } else {
            self.defines.insert(ident.name.clone(), num.value());
            Ok(())
        }
    }
}

impl AliasInstruction for ItemIntegratedCircuit10 {
    /// alias str r?|d?
    fn execute_alias(&mut self, vm: &VM, string: &Operand, r: &Operand) -> Result<(), ICError> {
        let &Operand::Identifier(ident) = &string else {
            return Err(IncorrectOperandType {
                inst: InstructionOp::Alias,
                index: 1,
                desired: "Name".to_owned(),
            });
        };
        let alias = match &r {
            Operand::RegisterSpec(RegisterSpec {
                indirection,
                target,
            }) => Operand::RegisterSpec(RegisterSpec {
                indirection: *indirection,
                target: *target,
            }),
            Operand::DeviceSpec(DeviceSpec { device, connection }) => {
                Operand::DeviceSpec(DeviceSpec {
                    device: *device,
                    connection: *connection,
                })
            }
            _ => {
                return Err(IncorrectOperandType {
                    inst: InstructionOp::Alias,
                    index: 2,
                    desired: "Device Or Register".to_owned(),
                })
            }
        };
        self.aliases.insert(ident.name.clone(), alias);
        Ok(())
    }
}

impl MoveInstruction for ItemIntegratedCircuit10 {
    /// move r? a(r?|num)
    fn execute_move(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self, InstructionOp::Move, 1)?;

        let val = a.as_value(self, InstructionOp::Move, 2)?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl BeqInstruction for ItemIntegratedCircuit10 {
    /// beq a(r?|num) b(r?|num) c(r?|num)
    fn execute_beq(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Beq, 1)?;
        let b = b.as_value(self, InstructionOp::Beq, 2)?;
        let c = c.as_value(self, InstructionOp::Beq, 3)?;
        if a == b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}
impl BeqalInstruction for ItemIntegratedCircuit10 {
    /// beqal a(r?|num) b(r?|num) c(r?|num)
    fn execute_beqal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Beqal, 1)?;
        let b = b.as_value(self, InstructionOp::Beqal, 2)?;
        let c = c.as_value(self, InstructionOp::Beqal, 3)?;
        if a == b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl BreqInstruction for ItemIntegratedCircuit10 {
    /// breq a(r?|num) b(r?|num) c(r?|num)
    fn execute_breq(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Breq, 1)?;
        let b = b.as_value(self, InstructionOp::Breq, 2)?;
        let c = c.as_value(self, InstructionOp::Breq, 3)?;
        if a == b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl BeqzInstruction for ItemIntegratedCircuit10 {
    /// beqz a(r?|num) b(r?|num)
    fn execute_beqz(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Beqz, 1)?;
        let b = b.as_value(self, InstructionOp::Beqz, 2)?;
        if a == 0.0 {
            self.set_next_instruction(b)
        }
        Ok(())
    }
}

impl BeqzalInstruction for ItemIntegratedCircuit10 {
    /// beqzal a(r?|num) b(r?|num)
    fn execute_beqzal(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Beqzal, 1)?;
        let b = b.as_value(self, InstructionOp::Beqzal, 2)?;
        if a == 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl BreqzInstruction for ItemIntegratedCircuit10 {
    /// breqz a(r?|num) b(r?|num)
    fn execute_breqz(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Breqz, 1)?;
        let b = b.as_value(self, InstructionOp::Breqz, 2)?;
        if a == 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl BneInstruction for ItemIntegratedCircuit10 {
    /// bne a(r?|num) b(r?|num) c(r?|num)
    fn execute_bne(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bne, 1)?;
        let b = b.as_value(self, InstructionOp::Bne, 2)?;
        let c = c.as_value(self, InstructionOp::Bne, 3)?;
        if a != b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}

impl BnealInstruction for ItemIntegratedCircuit10 {
    /// bneal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bneal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bneal, 1)?;
        let b = b.as_value(self, InstructionOp::Bneal, 2)?;
        let c = c.as_value(self, InstructionOp::Bneal, 3)?;
        if a != b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl BrneInstruction for ItemIntegratedCircuit10 {
    /// brne a(r?|num) b(r?|num) c(r?|num)
    fn execute_brne(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brne, 1)?;
        let b = b.as_value(self, InstructionOp::Brne, 2)?;
        let c = c.as_value(self, InstructionOp::Brne, 3)?;
        if a != b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl BnezInstruction for ItemIntegratedCircuit10 {
    /// bnez a(r?|num) b(r?|num)
    fn execute_bnez(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bnez, 1)?;
        let b = b.as_value(self, InstructionOp::Bnez, 2)?;
        if a != 0.0 {
            self.set_next_instruction(b)
        }
        Ok(())
    }
}

impl BnezalInstruction for ItemIntegratedCircuit10 {
    /// bnezal a(r?|num) b(r?|num)
    fn execute_bnezal(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bnezal, 1)?;
        let b = b.as_value(self, InstructionOp::Bnezal, 2)?;
        if a != 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl BrnezInstruction for ItemIntegratedCircuit10 {
    /// brnez a(r?|num) b(r?|num)
    fn execute_brnez(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brnez, 1)?;
        let b = b.as_value(self, InstructionOp::Brnez, 2)?;
        if a != 0.0 {
            self.set_next_instruction_relative(b)
        }
        Ok(())
    }
}

impl BltInstruction for ItemIntegratedCircuit10 {
    /// blt a(r?|num) b(r?|num) c(r?|num)
    fn execute_blt(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Blt, 1)?;
        let b = b.as_value(self, InstructionOp::Blt, 2)?;
        let c = c.as_value(self, InstructionOp::Blt, 3)?;
        if a < b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}

impl BltalInstruction for ItemIntegratedCircuit10 {
    /// bltal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bltal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bltal, 1)?;
        let b = b.as_value(self, InstructionOp::Bltal, 2)?;
        let c = c.as_value(self, InstructionOp::Bltal, 3)?;
        if a < b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl BrltInstruction for ItemIntegratedCircuit10 {
    /// brlt a(r?|num) b(r?|num) c(r?|num)
    fn execute_brlt(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brlt, 1)?;
        let b = b.as_value(self, InstructionOp::Brlt, 2)?;
        let c = c.as_value(self, InstructionOp::Brlt, 3)?;
        if a < b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl BltzInstruction for ItemIntegratedCircuit10 {
    /// bltz a(r?|num) b(r?|num)
    fn execute_bltz(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bltz, 1)?;
        let b = b.as_value(self, InstructionOp::Bltz, 2)?;
        if a < 0.0 {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl BltzalInstruction for ItemIntegratedCircuit10 {
    /// bltzal a(r?|num) b(r?|num)
    fn execute_bltzal(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bltzal, 1)?;
        let b = b.as_value(self, InstructionOp::Bltzal, 2)?;
        if a < 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl BrltzInstruction for ItemIntegratedCircuit10 {
    /// brltz a(r?|num) b(r?|num)
    fn execute_brltz(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brltz, 1)?;
        let b = b.as_value(self, InstructionOp::Brltz, 2)?;
        if a < 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl BleInstruction for ItemIntegratedCircuit10 {
    /// ble a(r?|num) b(r?|num) c(r?|num)
    fn execute_ble(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Ble, 1)?;
        let b = b.as_value(self, InstructionOp::Ble, 2)?;
        let c = c.as_value(self, InstructionOp::Ble, 3)?;
        if a <= b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}
impl BlealInstruction for ItemIntegratedCircuit10 {
    /// bleal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bleal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bleal, 1)?;
        let b = b.as_value(self, InstructionOp::Bleal, 2)?;
        let c = c.as_value(self, InstructionOp::Bleal, 3)?;
        if a <= b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl BrleInstruction for ItemIntegratedCircuit10 {
    /// brle a(r?|num) b(r?|num) c(r?|num)
    fn execute_brle(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brle, 1)?;
        let b = b.as_value(self, InstructionOp::Brle, 2)?;
        let c = c.as_value(self, InstructionOp::Brle, 3)?;
        if a <= b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl BlezInstruction for ItemIntegratedCircuit10 {
    /// blez a(r?|num) b(r?|num)
    fn execute_blez(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Blez, 1)?;
        let b = b.as_value(self, InstructionOp::Blez, 2)?;
        if a <= 0.0 {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl BlezalInstruction for ItemIntegratedCircuit10 {
    /// blezal a(r?|num) b(r?|num)
    fn execute_blezal(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Blezal, 1)?;
        let b = b.as_value(self, InstructionOp::Blezal, 2)?;
        if a <= 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl BrlezInstruction for ItemIntegratedCircuit10 {
    /// brlez a(r?|num) b(r?|num)
    fn execute_brlez(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brlez, 1)?;
        let b = b.as_value(self, InstructionOp::Brlez, 2)?;
        if a <= 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl BgtInstruction for ItemIntegratedCircuit10 {
    /// bgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_bgt(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bgt, 1)?;
        let b = b.as_value(self, InstructionOp::Bgt, 2)?;
        let c = c.as_value(self, InstructionOp::Bgt, 3)?;
        if a > b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}
impl BgtalInstruction for ItemIntegratedCircuit10 {
    /// bgtal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bgtal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bgtal, 1)?;
        let b = b.as_value(self, InstructionOp::Bgtal, 2)?;
        let c = c.as_value(self, InstructionOp::Bgtal, 3)?;
        if a > b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl BrgtInstruction for ItemIntegratedCircuit10 {
    /// brgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_brgt(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brgt, 1)?;
        let b = b.as_value(self, InstructionOp::Brgt, 2)?;
        let c = c.as_value(self, InstructionOp::Brgt, 3)?;
        if a > b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl BgtzInstruction for ItemIntegratedCircuit10 {
    /// bgtz a(r?|num) b(r?|num)
    fn execute_bgtz(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bgtz, 1)?;
        let b = b.as_value(self, InstructionOp::Bgtz, 2)?;
        if a > 0.0 {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl BgtzalInstruction for ItemIntegratedCircuit10 {
    /// bgtzal a(r?|num) b(r?|num)
    fn execute_bgtzal(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Bgtzal, 1)?;
        let b = b.as_value(self, InstructionOp::Bgtzal, 2)?;
        if a > 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl BrgtzInstruction for ItemIntegratedCircuit10 {
    /// brgtz a(r?|num) b(r?|num)
    fn execute_brgtz(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError> {
        let a = a.as_value(self, InstructionOp::Brgtz, 1)?;
        let b = b.as_value(self, InstructionOp::Brgtz, 2)?;
        if a > 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl AbsInstruction for ItemIntegratedCircuit10 {
    /// abs r? a(r?|num)
    fn execute_abs(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}

impl AcosInstruction for ItemIntegratedCircuit10 {
    /// acos r? a(r?|num)
    fn execute_acos(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl AddInstruction for ItemIntegratedCircuit10 {
    /// add r? a(r?|num) b(r?|num)
    fn execute_add(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl AndInstruction for ItemIntegratedCircuit10 {
    /// and r? a(r?|num) b(r?|num)
    fn execute_and(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl AsinInstruction for ItemIntegratedCircuit10 {
    /// asin r? a(r?|num)
    fn execute_asin(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl AtanInstruction for ItemIntegratedCircuit10 {
    /// atan r? a(r?|num)
    fn execute_atan(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl Atan2Instruction for ItemIntegratedCircuit10 {
    /// atan2 r? a(r?|num) b(r?|num)
    fn execute_atan2(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl BapInstruction for ItemIntegratedCircuit10 {
    /// bap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bap(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
        d: &Operand,
    ) -> Result<(), ICError>;
}
impl BapalInstruction for ItemIntegratedCircuit10 {
    /// bapal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bapal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
        d: &Operand,
    ) -> Result<(), ICError>;
}
impl BapzInstruction for ItemIntegratedCircuit10 {
    /// bapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_bapz(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BapzalInstruction for ItemIntegratedCircuit10 {
    /// bapzal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bapzal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BdnsInstruction for ItemIntegratedCircuit10 {
    /// bdns d? a(r?|num)
    fn execute_bdns(&mut self, vm: &VM, d: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl BdnsalInstruction for ItemIntegratedCircuit10 {
    /// bdnsal d? a(r?|num)
    fn execute_bdnsal(&mut self, vm: &VM, d: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl BdseInstruction for ItemIntegratedCircuit10 {
    /// bdse d? a(r?|num)
    fn execute_bdse(&mut self, vm: &VM, d: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl BdsealInstruction for ItemIntegratedCircuit10 {
    /// bdseal d? a(r?|num)
    fn execute_bdseal(&mut self, vm: &VM, d: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl BgeInstruction for ItemIntegratedCircuit10 {
    /// bge a(r?|num) b(r?|num) c(r?|num)
    fn execute_bge(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BgealInstruction for ItemIntegratedCircuit10 {
    /// bgeal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bgeal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BgezInstruction for ItemIntegratedCircuit10 {
    /// bgez a(r?|num) b(r?|num)
    fn execute_bgez(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError>;
}
impl BgezalInstruction for ItemIntegratedCircuit10 {
    /// bgezal a(r?|num) b(r?|num)
    fn execute_bgezal(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError>;
}
impl BnaInstruction for ItemIntegratedCircuit10 {
    /// bna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bna(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
        d: &Operand,
    ) -> Result<(), ICError>;
}
impl BnaalInstruction for ItemIntegratedCircuit10 {
    /// bnaal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bnaal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
        d: &Operand,
    ) -> Result<(), ICError>;
}
impl BnanInstruction for ItemIntegratedCircuit10 {
    /// bnan a(r?|num) b(r?|num)
    fn execute_bnan(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError>;
}
impl BnazInstruction for ItemIntegratedCircuit10 {
    /// bnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_bnaz(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BnazalInstruction for ItemIntegratedCircuit10 {
    /// bnazal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bnazal(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BrapInstruction for ItemIntegratedCircuit10 {
    /// brap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_brap(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
        d: &Operand,
    ) -> Result<(), ICError>;
}
impl BrapzInstruction for ItemIntegratedCircuit10 {
    /// brapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_brapz(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BrdnsInstruction for ItemIntegratedCircuit10 {
    /// brdns d? a(r?|num)
    fn execute_brdns(&mut self, vm: &VM, d: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl BrdseInstruction for ItemIntegratedCircuit10 {
    /// brdse d? a(r?|num)
    fn execute_brdse(&mut self, vm: &VM, d: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl BrgeInstruction for ItemIntegratedCircuit10 {
    /// brge a(r?|num) b(r?|num) c(r?|num)
    fn execute_brge(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl BrgezInstruction for ItemIntegratedCircuit10 {
    /// brgez a(r?|num) b(r?|num)
    fn execute_brgez(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError>;
}
impl BrnaInstruction for ItemIntegratedCircuit10 {
    /// brna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_brna(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
        d: &Operand,
    ) -> Result<(), ICError>;
}
impl BrnanInstruction for ItemIntegratedCircuit10 {
    /// brnan a(r?|num) b(r?|num)
    fn execute_brnan(&mut self, vm: &VM, a: &Operand, b: &Operand) -> Result<(), ICError>;
}
impl BrnazInstruction for ItemIntegratedCircuit10 {
    /// brnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_brnaz(
        &mut self,
        vm: &VM,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl CeilInstruction for ItemIntegratedCircuit10 {
    /// ceil r? a(r?|num)
    fn execute_ceil(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl ClrInstruction for ItemIntegratedCircuit10 {
    /// clr d?
    fn execute_clr(&mut self, vm: &VM, d: &Operand) -> Result<(), ICError>;
}
impl ClrdInstruction for ItemIntegratedCircuit10 {
    /// clrd id(r?|num)
    fn execute_clrd(&mut self, vm: &VM, id: &Operand) -> Result<(), ICError>;
}
impl CosInstruction for ItemIntegratedCircuit10 {
    /// cos r? a(r?|num)
    fn execute_cos(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl DivInstruction for ItemIntegratedCircuit10 {
    /// div r? a(r?|num) b(r?|num)
    fn execute_div(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl ExpInstruction for ItemIntegratedCircuit10 {
    /// exp r? a(r?|num)
    fn execute_exp(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl FloorInstruction for ItemIntegratedCircuit10 {
    /// floor r? a(r?|num)
    fn execute_floor(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl GetInstruction for ItemIntegratedCircuit10 {
    /// get r? d? address(r?|num)
    fn execute_get(
        &mut self,
        vm: &VM,
        r: &Operand,
        d: &Operand,
        address: &Operand,
    ) -> Result<(), ICError>;
}
impl GetdInstruction for ItemIntegratedCircuit10 {
    /// getd r? id(r?|num) address(r?|num)
    fn execute_getd(
        &mut self,
        vm: &VM,
        r: &Operand,
        id: &Operand,
        address: &Operand,
    ) -> Result<(), ICError>;
}
impl HcfInstruction for ItemIntegratedCircuit10 {
    /// hcf
    fn execute_hcf(&mut self, vm: &VM) -> Result<(), ICError>;
}
impl JInstruction for ItemIntegratedCircuit10 {
    /// j int
    fn execute_j(&mut self, vm: &VM, int: &Operand) -> Result<(), ICError>;
}
impl JalInstruction for ItemIntegratedCircuit10 {
    /// jal int
    fn execute_jal(&mut self, vm: &VM, int: &Operand) -> Result<(), ICError>;
}
impl JrInstruction for ItemIntegratedCircuit10 {
    /// jr int
    fn execute_jr(&mut self, vm: &VM, int: &Operand) -> Result<(), ICError>;
}
impl LInstruction for ItemIntegratedCircuit10 {
    /// l r? d? logicType
    fn execute_l(
        &mut self,
        vm: &VM,
        r: &Operand,
        d: &Operand,
        logic_type: &Operand,
    ) -> Result<(), ICError>;
}
impl LabelInstruction for ItemIntegratedCircuit10 {
    /// label d? str
    fn execute_label(&mut self, vm: &VM, d: &Operand, str: &Operand) -> Result<(), ICError>;
}
impl LbInstruction for ItemIntegratedCircuit10 {
    /// lb r? deviceHash logicType batchMode
    fn execute_lb(
        &mut self,
        vm: &VM,
        r: &Operand,
        device_hash: &Operand,
        logic_type: &Operand,
        batch_mode: &Operand,
    ) -> Result<(), ICError>;
}
impl LbnInstruction for ItemIntegratedCircuit10 {
    /// lbn r? deviceHash nameHash logicType batchMode
    fn execute_lbn(
        &mut self,
        vm: &VM,
        r: &Operand,
        device_hash: &Operand,
        name_hash: &Operand,
        logic_type: &Operand,
        batch_mode: &Operand,
    ) -> Result<(), ICError>;
}
impl LbnsInstruction for ItemIntegratedCircuit10 {
    /// lbns r? deviceHash nameHash slotIndex logicSlotType batchMode
    fn execute_lbns(
        &mut self,
        vm: &VM,
        r: &Operand,
        device_hash: &Operand,
        name_hash: &Operand,
        slot_index: &Operand,
        logic_slot_type: &Operand,
        batch_mode: &Operand,
    ) -> Result<(), ICError>;
}
impl LbsInstruction for ItemIntegratedCircuit10 {
    /// lbs r? deviceHash slotIndex logicSlotType batchMode
    fn execute_lbs(
        &mut self,
        vm: &VM,
        r: &Operand,
        device_hash: &Operand,
        slot_index: &Operand,
        logic_slot_type: &Operand,
        batch_mode: &Operand,
    ) -> Result<(), ICError>;
}
impl LdInstruction for ItemIntegratedCircuit10 {
    /// ld r? id(r?|num) logicType
    fn execute_ld(
        &mut self,
        vm: &VM,
        r: &Operand,
        id: &Operand,
        logic_type: &Operand,
    ) -> Result<(), ICError>;
}
impl LogInstruction for ItemIntegratedCircuit10 {
    /// log r? a(r?|num)
    fn execute_log(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl LrInstruction for ItemIntegratedCircuit10 {
    /// lr r? d? reagentMode int
    fn execute_lr(
        &mut self,
        vm: &VM,
        r: &Operand,
        d: &Operand,
        reagent_mode: &Operand,
        int: &Operand,
    ) -> Result<(), ICError>;
}
impl LsInstruction for ItemIntegratedCircuit10 {
    /// ls r? d? slotIndex logicSlotType
    fn execute_ls(
        &mut self,
        vm: &VM,
        r: &Operand,
        d: &Operand,
        slot_index: &Operand,
        logic_slot_type: &Operand,
    ) -> Result<(), ICError>;
}
impl MaxInstruction for ItemIntegratedCircuit10 {
    /// max r? a(r?|num) b(r?|num)
    fn execute_max(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl MinInstruction for ItemIntegratedCircuit10 {
    /// min r? a(r?|num) b(r?|num)
    fn execute_min(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl ModInstruction for ItemIntegratedCircuit10 {
    /// mod r? a(r?|num) b(r?|num)
    fn execute_mod(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl MulInstruction for ItemIntegratedCircuit10 {
    /// mul r? a(r?|num) b(r?|num)
    fn execute_mul(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl NorInstruction for ItemIntegratedCircuit10 {
    /// nor r? a(r?|num) b(r?|num)
    fn execute_nor(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl NotInstruction for ItemIntegratedCircuit10 {
    /// not r? a(r?|num)
    fn execute_not(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl OrInstruction for ItemIntegratedCircuit10 {
    /// or r? a(r?|num) b(r?|num)
    fn execute_or(&mut self, vm: &VM, r: &Operand, a: &Operand, b: &Operand)
        -> Result<(), ICError>;
}
impl PeekInstruction for ItemIntegratedCircuit10 {
    /// peek r?
    fn execute_peek(&mut self, vm: &VM, r: &Operand) -> Result<(), ICError>;
}
impl PokeInstruction for ItemIntegratedCircuit10 {
    /// poke address(r?|num) value(r?|num)
    fn execute_poke(&mut self, vm: &VM, address: &Operand, value: &Operand) -> Result<(), ICError>;
}
impl PopInstruction for ItemIntegratedCircuit10 {
    /// pop r?
    fn execute_pop(&mut self, vm: &VM, r: &Operand) -> Result<(), ICError>;
}
impl PushInstruction for ItemIntegratedCircuit10 {
    /// push a(r?|num)
    fn execute_push(&mut self, vm: &VM, a: &Operand) -> Result<(), ICError>;
}
impl PutInstruction for ItemIntegratedCircuit10 {
    /// put d? address(r?|num) value(r?|num)
    fn execute_put(
        &mut self,
        vm: &VM,
        d: &Operand,
        address: &Operand,
        value: &Operand,
    ) -> Result<(), ICError>;
}
impl PutdInstruction for ItemIntegratedCircuit10 {
    /// putd id(r?|num) address(r?|num) value(r?|num)
    fn execute_putd(
        &mut self,
        vm: &VM,
        id: &Operand,
        address: &Operand,
        value: &Operand,
    ) -> Result<(), ICError>;
}
impl RandInstruction for ItemIntegratedCircuit10 {
    /// rand r?
    fn execute_rand(&mut self, vm: &VM, r: &Operand) -> Result<(), ICError>;
}
impl RoundInstruction for ItemIntegratedCircuit10 {
    /// round r? a(r?|num)
    fn execute_round(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SInstruction for ItemIntegratedCircuit10 {
    /// s d? logicType r?
    fn execute_s(
        &mut self,
        vm: &VM,
        d: &Operand,
        logic_type: &Operand,
        r: &Operand,
    ) -> Result<(), ICError>;
}
impl SapInstruction for ItemIntegratedCircuit10 {
    /// sap r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_sap(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl SapzInstruction for ItemIntegratedCircuit10 {
    /// sapz r? a(r?|num) b(r?|num)
    fn execute_sapz(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SbInstruction for ItemIntegratedCircuit10 {
    /// sb deviceHash logicType r?
    fn execute_sb(
        &mut self,
        vm: &VM,
        device_hash: &Operand,
        logic_type: &Operand,
        r: &Operand,
    ) -> Result<(), ICError>;
}
impl SbnInstruction for ItemIntegratedCircuit10 {
    /// sbn deviceHash nameHash logicType r?
    fn execute_sbn(
        &mut self,
        vm: &VM,
        device_hash: &Operand,
        name_hash: &Operand,
        logic_type: &Operand,
        r: &Operand,
    ) -> Result<(), ICError>;
}
impl SbsInstruction for ItemIntegratedCircuit10 {
    /// sbs deviceHash slotIndex logicSlotType r?
    fn execute_sbs(
        &mut self,
        vm: &VM,
        device_hash: &Operand,
        slot_index: &Operand,
        logic_slot_type: &Operand,
        r: &Operand,
    ) -> Result<(), ICError>;
}
impl SdInstruction for ItemIntegratedCircuit10 {
    /// sd id(r?|num) logicType r?
    fn execute_sd(
        &mut self,
        vm: &VM,
        id: &Operand,
        logic_type: &Operand,
        r: &Operand,
    ) -> Result<(), ICError>;
}
impl SdnsInstruction for ItemIntegratedCircuit10 {
    /// sdns r? d?
    fn execute_sdns(&mut self, vm: &VM, r: &Operand, d: &Operand) -> Result<(), ICError>;
}
impl SdseInstruction for ItemIntegratedCircuit10 {
    /// sdse r? d?
    fn execute_sdse(&mut self, vm: &VM, r: &Operand, d: &Operand) -> Result<(), ICError>;
}
impl SelectInstruction for ItemIntegratedCircuit10 {
    /// select r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_select(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl SeqInstruction for ItemIntegratedCircuit10 {
    /// seq r? a(r?|num) b(r?|num)
    fn execute_seq(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SeqzInstruction for ItemIntegratedCircuit10 {
    /// seqz r? a(r?|num)
    fn execute_seqz(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SgeInstruction for ItemIntegratedCircuit10 {
    /// sge r? a(r?|num) b(r?|num)
    fn execute_sge(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SgezInstruction for ItemIntegratedCircuit10 {
    /// sgez r? a(r?|num)
    fn execute_sgez(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SgtInstruction for ItemIntegratedCircuit10 {
    /// sgt r? a(r?|num) b(r?|num)
    fn execute_sgt(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SgtzInstruction for ItemIntegratedCircuit10 {
    /// sgtz r? a(r?|num)
    fn execute_sgtz(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SinInstruction for ItemIntegratedCircuit10 {
    /// sin r? a(r?|num)
    fn execute_sin(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SlaInstruction for ItemIntegratedCircuit10 {
    /// sla r? a(r?|num) b(r?|num)
    fn execute_sla(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SleInstruction for ItemIntegratedCircuit10 {
    /// sle r? a(r?|num) b(r?|num)
    fn execute_sle(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SlezInstruction for ItemIntegratedCircuit10 {
    /// slez r? a(r?|num)
    fn execute_slez(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SllInstruction for ItemIntegratedCircuit10 {
    /// sll r? a(r?|num) b(r?|num)
    fn execute_sll(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SltInstruction for ItemIntegratedCircuit10 {
    /// slt r? a(r?|num) b(r?|num)
    fn execute_slt(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SltzInstruction for ItemIntegratedCircuit10 {
    /// sltz r? a(r?|num)
    fn execute_sltz(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SnaInstruction for ItemIntegratedCircuit10 {
    /// sna r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_sna(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
        c: &Operand,
    ) -> Result<(), ICError>;
}
impl SnanInstruction for ItemIntegratedCircuit10 {
    /// snan r? a(r?|num)
    fn execute_snan(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SnanzInstruction for ItemIntegratedCircuit10 {
    /// snanz r? a(r?|num)
    fn execute_snanz(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SnazInstruction for ItemIntegratedCircuit10 {
    /// snaz r? a(r?|num) b(r?|num)
    fn execute_snaz(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SneInstruction for ItemIntegratedCircuit10 {
    /// sne r? a(r?|num) b(r?|num)
    fn execute_sne(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SnezInstruction for ItemIntegratedCircuit10 {
    /// snez r? a(r?|num)
    fn execute_snez(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SqrtInstruction for ItemIntegratedCircuit10 {
    /// sqrt r? a(r?|num)
    fn execute_sqrt(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl SraInstruction for ItemIntegratedCircuit10 {
    /// sra r? a(r?|num) b(r?|num)
    fn execute_sra(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SrlInstruction for ItemIntegratedCircuit10 {
    /// srl r? a(r?|num) b(r?|num)
    fn execute_srl(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl SsInstruction for ItemIntegratedCircuit10 {
    /// ss d? slotIndex logicSlotType r?
    fn execute_ss(
        &mut self,
        vm: &VM,
        d: &Operand,
        slot_index: &Operand,
        logic_slot_type: &Operand,
        r: &Operand,
    ) -> Result<(), ICError>;
}
impl SubInstruction for ItemIntegratedCircuit10 {
    /// sub r? a(r?|num) b(r?|num)
    fn execute_sub(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
impl TanInstruction for ItemIntegratedCircuit10 {
    /// tan r? a(r?|num)
    fn execute_tan(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl TruncInstruction for ItemIntegratedCircuit10 {
    /// trunc r? a(r?|num)
    fn execute_trunc(&mut self, vm: &VM, r: &Operand, a: &Operand) -> Result<(), ICError>;
}
impl XorInstruction for ItemIntegratedCircuit10 {
    /// xor r? a(r?|num) b(r?|num)
    fn execute_xor(
        &mut self,
        vm: &VM,
        r: &Operand,
        a: &Operand,
        b: &Operand,
    ) -> Result<(), ICError>;
}
