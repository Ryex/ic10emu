use crate::{
    errors::ICError,
    interpreter::{i64_to_f64, ICState},
    vm::{
        instructions::{
            operands::{InstOperand, RegisterSpec},
            traits::*,
        },
        object::{
            errors::{LogicError, MemoryError},
            traits::*,
            ObjectID,
        },
    },
};
use stationeers_data::enums::script::LogicReagentMode;
pub trait IC10Marker: IntegratedCircuit {}

impl<T: IC10Marker> SleepInstruction for T {
    /// sleep a(r?|num)
    fn execute_inner(&mut self, a: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let now =
            time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        self.set_state(ICState::Sleep(now, a));
        Ok(())
    }
}

impl<T: IC10Marker> YieldInstruction for T {
    /// yield
    fn execute_inner(&mut self) -> Result<(), ICError> {
        self.set_state(ICState::Yield);
        Ok(())
    }
}

impl<T: IC10Marker> DefineInstruction for T {
    /// define str num
    fn execute_inner(&mut self, string: &InstOperand, num: &InstOperand) -> Result<(), ICError> {
        let ident = string.as_ident()?;
        let num = num.as_number()?;
        if self.get_defines().contains_key(&ident.name) {
            Err(ICError::DuplicateDefine(ident.name.clone()))
        } else {
            self.get_defines_mut()
                .insert(ident.name.clone(), num.value());
            Ok(())
        }
    }
}

impl<T: IC10Marker> AliasInstruction for T {
    /// alias str r?|d?
    fn execute_inner(&mut self, string: &InstOperand, r: &InstOperand) -> Result<(), ICError> {
        let ident = string.as_ident()?;
        let alias = r.as_aliasable()?;
        self.get_aliases_mut().insert(ident.name.clone(), alias);
        Ok(())
    }
}

impl<T: IC10Marker> MoveInstruction for T {
    /// move r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;

        let val = a.as_value(self)?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> BeqInstruction for T {
    /// beq a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a == b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}
impl<T: IC10Marker> BeqalInstruction for T {
    /// beqal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a == b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BreqInstruction for T {
    /// breq a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a == b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BeqzInstruction for T {
    /// beqz a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a == 0.0 {
            self.set_next_instruction(b)
        }
        Ok(())
    }
}

impl<T: IC10Marker> BeqzalInstruction for T {
    /// beqzal a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a == 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BreqzInstruction for T {
    /// breqz a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a == 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BneInstruction for T {
    /// bne a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a != b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BnealInstruction for T {
    /// bneal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a != b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrneInstruction for T {
    /// brne a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a != b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BnezInstruction for T {
    /// bnez a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a != 0.0 {
            self.set_next_instruction(b)
        }
        Ok(())
    }
}

impl<T: IC10Marker> BnezalInstruction for T {
    /// bnezal a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a != 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrnezInstruction for T {
    /// brnez a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a != 0.0 {
            self.set_next_instruction_relative(b)
        }
        Ok(())
    }
}

impl<T: IC10Marker> BltInstruction for T {
    /// blt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a < b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BltalInstruction for T {
    /// bltal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a < b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrltInstruction for T {
    /// brlt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a < b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BltzInstruction for T {
    /// bltz a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a < 0.0 {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BltzalInstruction for T {
    /// bltzal a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a < 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrltzInstruction for T {
    /// brltz a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a < 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BleInstruction for T {
    /// ble a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a <= b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}
impl<T: IC10Marker> BlealInstruction for T {
    /// bleal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a <= b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrleInstruction for T {
    /// brle a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a <= b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BlezInstruction for T {
    /// blez a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a <= 0.0 {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BlezalInstruction for T {
    /// blezal a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a <= 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrlezInstruction for T {
    /// brlez a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a <= 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BgtInstruction for T {
    /// bgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a > b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}
impl<T: IC10Marker> BgtalInstruction for T {
    /// bgtal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a > b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrgtInstruction for T {
    /// brgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a > b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BgtzInstruction for T {
    /// bgtz a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a > 0.0 {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BgtzalInstruction for T {
    /// bgtzal a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a > 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrgtzInstruction for T {
    /// brgtz a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a > 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BgeInstruction for T {
    /// bge a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a >= b {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BgealInstruction for T {
    /// bgeal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a >= b {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrgeInstruction for T {
    /// brge a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a >= b {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BgezInstruction for T {
    /// bgez a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a >= 0.0 {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BgezalInstruction for T {
    /// bgezal a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a >= 0.0 {
            self.set_next_instruction(b);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrgezInstruction for T {
    /// brgez a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a >= 0.0 {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BapInstruction for T {
    /// bap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,

        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
        d: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        let d = d.as_value(self)?;
        if f64::abs(a - b) <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
            self.set_next_instruction(d);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BapalInstruction for T {
    /// bapal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,

        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
        d: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        let d = d.as_value(self)?;
        if f64::abs(a - b) <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
            self.set_next_instruction(d);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrapInstruction for T {
    /// brap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,

        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
        d: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        let d = d.as_value(self)?;
        if f64::abs(a - b) <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
            self.set_next_instruction_relative(d);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BapzInstruction for T {
    /// bapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
        } else {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BapzalInstruction for T {
    /// bapzal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
        } else {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrapzInstruction for T {
    /// brapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
        } else {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BnaInstruction for T {
    /// bna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,

        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
        d: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        let d = d.as_value(self)?;
        if f64::abs(a - b) > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
            self.set_next_instruction(d);
        }
        Ok(())
    }
}
impl<T: IC10Marker> BnaalInstruction for T {
    /// bnaal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,

        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
        d: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        let d = d.as_value(self)?;
        if f64::abs(a - b) > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
            self.set_next_instruction(d);
            self.al();
        }
        Ok(())
    }
}
impl<T: IC10Marker> BrnaInstruction for T {
    /// brna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
        d: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        let d = d.as_value(self)?;
        if f64::abs(a - b) > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
            self.set_next_instruction_relative(d);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BnazInstruction for T {
    /// bnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
            self.set_next_instruction(c);
        }
        Ok(())
    }
}
impl<T: IC10Marker> BnazalInstruction for T {
    /// bnazal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
            self.set_next_instruction(c);
            self.al();
        }
        Ok(())
    }
}
impl<T: IC10Marker> BrnazInstruction for T {
    /// brnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
            self.set_next_instruction_relative(c);
        }
        Ok(())
    }
}
impl<T: IC10Marker> BdseInstruction for T {
    /// bdse d? a(r?|num)
    fn execute_inner(&mut self, d: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let a = a.as_value(self)?;
        if self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .is_some()
        {
            self.set_next_instruction(a);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BdsealInstruction for T {
    /// bdseal d? a(r?|num)
    fn execute_inner(&mut self, d: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let a = a.as_value(self)?;
        if self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .is_some()
        {
            self.set_next_instruction(a);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrdseInstruction for T {
    /// brdse d? a(r?|num)
    fn execute_inner(&mut self, d: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let a = a.as_value(self)?;
        if self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .is_some()
        {
            self.set_next_instruction_relative(a);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BdnsInstruction for T {
    /// bdns d? a(r?|num)
    fn execute_inner(&mut self, d: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let a = a.as_value(self)?;
        if self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .is_none()
        {
            self.set_next_instruction(a);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BdnsalInstruction for T {
    /// bdnsal d? a(r?|num)
    fn execute_inner(&mut self, d: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let a = a.as_value(self)?;
        if self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .is_none()
        {
            self.set_next_instruction(a);
            self.al();
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrdnsInstruction for T {
    /// brdns d? a(r?|num)
    fn execute_inner(&mut self, d: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let a = a.as_value(self)?;
        if self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .is_none()
        {
            self.set_next_instruction_relative(a);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BnanInstruction for T {
    /// bnan a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a.is_nan() {
            self.set_next_instruction(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> BrnanInstruction for T {
    /// brnan a(r?|num) b(r?|num)
    fn execute_inner(&mut self, a: &InstOperand, b: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        if a.is_nan() {
            self.set_next_instruction_relative(b);
        }
        Ok(())
    }
}

impl<T: IC10Marker> JInstruction for T {
    /// j int
    fn execute_inner(&mut self, int: &InstOperand) -> Result<(), ICError> {
        let int = int.as_value(self)?;
        self.set_next_instruction(int);
        Ok(())
    }
}
impl<T: IC10Marker> JalInstruction for T {
    /// jal int
    fn execute_inner(&mut self, int: &InstOperand) -> Result<(), ICError> {
        let int = int.as_value(self)?;
        self.set_next_instruction(int);
        self.al();
        Ok(())
    }
}
impl<T: IC10Marker> JrInstruction for T {
    /// jr int
    fn execute_inner(&mut self, int: &InstOperand) -> Result<(), ICError> {
        let int = int.as_value(self)?;
        self.set_next_instruction_relative(int);
        Ok(())
    }
}
impl<T: IC10Marker> SeqInstruction for T {
    /// seq r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, if a == b { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SeqzInstruction for T {
    /// seqz r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a == 0.0 { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SneInstruction for T {
    /// sne r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, if a != b { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SnezInstruction for T {
    /// snez r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a != 0.0 { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SltInstruction for T {
    /// slt r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, if a < b { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SltzInstruction for T {
    /// sltz r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a < 0.0 { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SleInstruction for T {
    /// sle r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, if a <= b { 1.0 } else { 0.0 })?;
        Ok(())
    }
}
impl<T: IC10Marker> SlezInstruction for T {
    /// slez r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a <= 0.0 { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SgtInstruction for T {
    /// sgt r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, if a > b { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SgtzInstruction for T {
    /// sgtz r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a > 0.0 { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SgeInstruction for T {
    /// sge r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, if a >= b { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SgezInstruction for T {
    /// sgez r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a >= 0.0 { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SapInstruction for T {
    /// sap r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        self.set_register(
            indirection,
            target,
            if f64::abs(a - b) <= f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
                1.0
            } else {
                0.0
            },
        )?;
        Ok(())
    }
}

impl<T: IC10Marker> SapzInstruction for T {
    /// sapz r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(
            indirection,
            target,
            if a.abs() <= f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                1.0
            } else {
                0.0
            },
        )?;
        Ok(())
    }
}

impl<T: IC10Marker> SnaInstruction for T {
    /// sna r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        self.set_register(
            indirection,
            target,
            if f64::abs(a - b) > f64::max(c * f64::max(a.abs(), b.abs()), f64::EPSILON * 8.0) {
                1.0
            } else {
                0.0
            },
        )?;
        Ok(())
    }
}

impl<T: IC10Marker> SnazInstruction for T {
    /// snaz r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(
            indirection,
            target,
            if a.abs() > f64::max(b * a.abs(), f64::EPSILON * 8.0) {
                1.0
            } else {
                0.0
            },
        )?;
        Ok(())
    }
}

impl<T: IC10Marker> SdseInstruction for T {
    /// sdse r? d?
    fn execute_inner(&mut self, r: &InstOperand, d: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let (device, connection) = d.as_device(self)?;
        let is_some = {
            self.get_circuit_holder()
                .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
                .borrow()
                .as_circuit_holder()
                .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
                .get_logicable_from_index(device, connection)
                .is_some()
        };
        self.set_register(indirection, target, if is_some { 1.0 } else { 0.0 })?;
        Ok(())
    }
}
impl<T: IC10Marker> SdnsInstruction for T {
    /// sdns r? d?
    fn execute_inner(&mut self, r: &InstOperand, d: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let (device, connection) = d.as_device(self)?;
        let is_none = {
            self.get_circuit_holder()
                .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
                .borrow()
                .as_circuit_holder()
                .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
                .get_logicable_from_index(device, connection)
                .is_none()
        };
        self.set_register(indirection, target, if is_none { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SnanInstruction for T {
    /// snan r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a.is_nan() { 1.0 } else { 0.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SnanzInstruction for T {
    /// snanz r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, if a.is_nan() { 0.0 } else { 1.0 })?;
        Ok(())
    }
}

impl<T: IC10Marker> SelectInstruction for T {
    /// select r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
        c: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let c = c.as_value(self)?;
        self.set_register(indirection, target, if a != 0.0 { b } else { c })?;
        Ok(())
    }
}

impl<T: IC10Marker> AddInstruction for T {
    /// add r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, a + b)?;
        Ok(())
    }
}

impl<T: IC10Marker> SubInstruction for T {
    /// sub r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, a - b)?;
        Ok(())
    }
}
impl<T: IC10Marker> MulInstruction for T {
    /// mul r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, a * b)?;
        Ok(())
    }
}
impl<T: IC10Marker> DivInstruction for T {
    /// div r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, a / b)?;
        Ok(())
    }
}

impl<T: IC10Marker> ModInstruction for T {
    /// mod r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        let mut m = a % b;
        if m < 0.0 {
            m += b;
        }
        self.set_register(indirection, target, m)?;
        Ok(())
    }
}

impl<T: IC10Marker> ExpInstruction for T {
    /// exp r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::exp(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> LogInstruction for T {
    /// log r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::ln(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> SqrtInstruction for T {
    /// sqrt r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::sqrt(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> MaxInstruction for T {
    /// max r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, f64::max(a, b))?;
        Ok(())
    }
}

impl<T: IC10Marker> MinInstruction for T {
    /// min r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, f64::min(a, b))?;
        Ok(())
    }
}
impl<T: IC10Marker> CeilInstruction for T {
    /// ceil r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::ceil(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> FloorInstruction for T {
    /// floor r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::floor(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> AbsInstruction for T {
    /// abs r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::abs(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> RoundInstruction for T {
    /// round r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::round(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> TruncInstruction for T {
    /// trunc r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::trunc(a))?;
        Ok(())
    }
}

impl<T: IC10Marker> RandInstruction for T {
    /// rand r?
    fn execute_inner(&mut self, r: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let val = self.get_vm().random_f64();
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}
impl<T: IC10Marker> SinInstruction for T {
    /// sin r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::sin(a))?;
        Ok(())
    }
}
impl<T: IC10Marker> CosInstruction for T {
    /// cos r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::cos(a))?;
        Ok(())
    }
}
impl<T: IC10Marker> TanInstruction for T {
    /// tan r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::tan(a))?;
        Ok(())
    }
}
impl<T: IC10Marker> AsinInstruction for T {
    /// asin r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::asin(a))?;
        Ok(())
    }
}
impl<T: IC10Marker> AcosInstruction for T {
    /// acos r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::acos(a))?;
        Ok(())
    }
}
impl<T: IC10Marker> AtanInstruction for T {
    /// atan r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        self.set_register(indirection, target, f64::atan(a))?;
        Ok(())
    }
}
impl<T: IC10Marker> Atan2Instruction for T {
    /// atan2 r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value(self)?;
        let b = b.as_value(self)?;
        self.set_register(indirection, target, f64::atan2(a, b))?;
        Ok(())
    }
}

impl<T: IC10Marker> SllInstruction for T {
    /// sll r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        let b = b.as_value_i32(self, true)?;
        self.set_register(indirection, target, i64_to_f64(a << b))?;
        Ok(())
    }
}

impl<T: IC10Marker> SlaInstruction for T {
    /// sla r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        let b = b.as_value_i32(self, true)?;
        self.set_register(indirection, target, i64_to_f64(a << b))?;
        Ok(())
    }
}

impl<T: IC10Marker> SrlInstruction for T {
    /// srl r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, false)?;
        let b = b.as_value_i32(self, true)?;
        self.set_register(indirection, target, i64_to_f64((a as u64 >> b) as i64))?;
        Ok(())
    }
}

impl<T: IC10Marker> SraInstruction for T {
    /// sra r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        let b = b.as_value_i32(self, true)?;
        self.set_register(indirection, target, i64_to_f64(a >> b))?;
        Ok(())
    }
}

impl<T: IC10Marker> AndInstruction for T {
    /// and r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        let b = b.as_value_i64(self, true)?;
        self.set_register(indirection, target, i64_to_f64(a & b))?;
        Ok(())
    }
}

impl<T: IC10Marker> OrInstruction for T {
    /// or r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        let b = b.as_value_i64(self, true)?;
        self.set_register(indirection, target, i64_to_f64(a | b))?;
        Ok(())
    }
}

impl<T: IC10Marker> XorInstruction for T {
    /// xor r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        let b = b.as_value_i64(self, true)?;
        self.set_register(indirection, target, i64_to_f64(a ^ b))?;
        Ok(())
    }
}

impl<T: IC10Marker> NorInstruction for T {
    /// nor r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        a: &InstOperand,
        b: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        let b = b.as_value_i64(self, true)?;
        self.set_register(indirection, target, i64_to_f64(!(a | b)))?;
        Ok(())
    }
}

impl<T: IC10Marker> NotInstruction for T {
    /// not r? a(r?|num)
    fn execute_inner(&mut self, r: &InstOperand, a: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let a = a.as_value_i64(self, true)?;
        self.set_register(indirection, target, i64_to_f64(!a))?;
        Ok(())
    }
}

impl<T: IC10Marker> PushInstruction for T {
    /// push a(r?|num)
    fn execute_inner(&mut self, a: &InstOperand) -> Result<(), ICError> {
        let a = a.as_value(self)?;
        self.push_stack(a)?;
        Ok(())
    }
}

impl<T: IC10Marker> PopInstruction for T {
    /// pop r?
    fn execute_inner(&mut self, r: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let val = self.pop_stack()?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> PokeInstruction for T {
    /// poke address(r?|num) value(r?|num)
    fn execute_inner(&mut self, address: &InstOperand, value: &InstOperand) -> Result<(), ICError> {
        let address = address.as_value(self)?;
        let value = value.as_value(self)?;
        self.put_stack(address, value)?;
        Ok(())
    }
}

impl<T: IC10Marker> PeekInstruction for T {
    /// peek r?
    fn execute_inner(&mut self, r: &InstOperand) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let val = self.peek_stack()?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> GetInstruction for T {
    /// get r? d? address(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        d: &InstOperand,
        address: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let address = address.as_value(self)?;
        let (device, connection) = d.as_device(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|obj| {
                obj.map(|obj_ref| {
                    let val = obj_ref
                        .as_memory_readable()
                        .ok_or(MemoryError::NotWriteable)?
                        .get_memory(address as i32)?;
                    self.set_register(indirection, target, val)
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> GetdInstruction for T {
    /// getd r? id(r?|num) address(r?|num)
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        id: &InstOperand,
        address: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let id = id.as_value(self)?;
        let address = address.as_value(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_id(id as ObjectID, None)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|obj| {
                obj.map(|obj_ref| {
                    let val = obj_ref
                        .as_memory_readable()
                        .ok_or(MemoryError::NotWriteable)?
                        .get_memory(address as i32)?;
                    self.set_register(indirection, target, val)
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> PutInstruction for T {
    /// put d? address(r?|num) value(r?|num)
    fn execute_inner(
        &mut self,
        d: &InstOperand,
        address: &InstOperand,
        value: &InstOperand,
    ) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let address = address.as_value(self)?;
        let value = value.as_value(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow_mut()
            .as_mut_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index_mut(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|mut obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_mut_memory_writable()
                        .ok_or(MemoryError::NotWriteable)?
                        .set_memory(address as i32, value)
                        .map_err(Into::into)
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> PutdInstruction for T {
    /// putd id(r?|num) address(r?|num) value(r?|num)
    fn execute_inner(
        &mut self,
        id: &InstOperand,
        address: &InstOperand,
        value: &InstOperand,
    ) -> Result<(), ICError> {
        let id = id.as_value(self)?;
        let address = address.as_value(self)?;
        let value = value.as_value(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow_mut()
            .as_mut_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_id_mut(id as ObjectID, None)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|mut obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_mut_memory_writable()
                        .ok_or(MemoryError::NotWriteable)?
                        .set_memory(address as i32, value)
                        .map_err(Into::into)
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> ClrInstruction for T {
    /// clr d?
    fn execute_inner(&mut self, d: &InstOperand) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow_mut()
            .as_mut_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index_mut(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|mut obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_mut_memory_writable()
                        .ok_or(MemoryError::NotWriteable)?
                        .clear_memory();
                    Ok(())
                })
            })?;
        Ok(())
    }
}
impl<T: IC10Marker> ClrdInstruction for T {
    /// clrd id(r?|num)
    fn execute_inner(&mut self, id: &InstOperand) -> Result<(), ICError> {
        let id = id.as_value(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow_mut()
            .as_mut_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_id_mut(id as ObjectID, None)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|mut obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_mut_memory_writable()
                        .ok_or(MemoryError::NotWriteable)?
                        .clear_memory();
                    Ok(())
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> SInstruction for T {
    /// s d? logicType r?
    fn execute_inner(
        &mut self,
        d: &InstOperand,
        logic_type: &InstOperand,
        r: &InstOperand,
    ) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let val = r.as_value(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow_mut()
            .as_mut_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index_mut(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|mut obj| {
                let obj_id = obj.get_id();
                obj.map(|obj_ref| {
                    obj_ref
                        .as_mut_logicable()
                        .ok_or(ICError::NotLogicable(obj_id))
                        .and_then(|logicable| {
                            if !logicable.can_logic_write(logic_type) {
                                Err(LogicError::CantWrite(logic_type).into())
                            } else {
                                logicable
                                    .set_logic(logic_type, val, false)
                                    .map_err(Into::into)
                            }
                        })
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> SdInstruction for T {
    /// sd id(r?|num) logicType r?
    fn execute_inner(
        &mut self,
        id: &InstOperand,
        logic_type: &InstOperand,
        r: &InstOperand,
    ) -> Result<(), ICError> {
        let id = id.as_value(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let val = r.as_value(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow_mut()
            .as_mut_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_id_mut(id as ObjectID, None)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|mut obj| {
                let obj_id = obj.get_id();
                obj.map(|obj_ref| {
                    obj_ref
                        .as_mut_logicable()
                        .ok_or(ICError::NotLogicable(obj_id))
                        .and_then(|logicable| {
                            if !logicable.can_logic_write(logic_type) {
                                Err(LogicError::CantWrite(logic_type).into())
                            } else {
                                logicable
                                    .set_logic(logic_type, val, false)
                                    .map_err(Into::into)
                            }
                        })
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> SsInstruction for T {
    /// ss d? slotIndex logicSlotType r?
    fn execute_inner(
        &mut self,
        d: &InstOperand,
        slot_index: &InstOperand,
        logic_slot_type: &InstOperand,
        r: &InstOperand,
    ) -> Result<(), ICError> {
        let (device, connection) = d.as_device(self)?;
        let slot_index = slot_index.as_value(self)?;
        let logic_slot_type = logic_slot_type.as_slot_logic_type(self)?;
        let val = r.as_value(self)?;
        self.get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow_mut()
            .as_mut_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index_mut(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|mut obj| {
                let obj_id = obj.get_id();
                obj.map(|obj_ref| {
                    obj_ref
                        .as_mut_device()
                        .ok_or(ICError::NotLogicable(obj_id))
                        .and_then(|logicable| {
                            if !logicable.can_slot_logic_write(logic_slot_type, slot_index) {
                                Err(LogicError::CantSlotWrite(logic_slot_type, slot_index).into())
                            } else {
                                logicable
                                    .set_slot_logic(logic_slot_type, slot_index, val, false)
                                    .map_err(Into::into)
                            }
                        })
                })
            })?;
        Ok(())
    }
}

impl<T: IC10Marker> SbInstruction for T {
    /// sb deviceHash logicType r?
    fn execute_inner(
        &mut self,

        device_hash: &InstOperand,
        logic_type: &InstOperand,
        r: &InstOperand,
    ) -> Result<(), ICError> {
        let prefab = device_hash.as_value(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let val = r.as_value(self)?;
        self.get_vm()
            .set_batch_device_field(*self.get_id(), prefab, logic_type, val, false)?;
        Ok(())
    }
}

impl<T: IC10Marker> SbsInstruction for T {
    /// sbs deviceHash slotIndex logicSlotType r?
    fn execute_inner(
        &mut self,

        device_hash: &InstOperand,
        slot_index: &InstOperand,
        logic_slot_type: &InstOperand,
        r: &InstOperand,
    ) -> Result<(), ICError> {
        let prefab = device_hash.as_value(self)?;
        let slot_index = slot_index.as_value(self)?;
        let logic_slot_type = logic_slot_type.as_slot_logic_type(self)?;
        let val = r.as_value(self)?;
        self.get_vm().set_batch_device_slot_field(
            *self.get_id(),
            prefab,
            slot_index,
            logic_slot_type,
            val,
            false,
        )?;
        Ok(())
    }
}

impl<T: IC10Marker> SbnInstruction for T {
    /// sbn deviceHash nameHash logicType r?
    fn execute_inner(
        &mut self,

        device_hash: &InstOperand,
        name_hash: &InstOperand,
        logic_type: &InstOperand,
        r: &InstOperand,
    ) -> Result<(), ICError> {
        let prefab = device_hash.as_value(self)?;
        let name = name_hash.as_value(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let val = r.as_value(self)?;
        self.get_vm().set_batch_name_device_field(
            *self.get_id(),
            prefab,
            name,
            logic_type,
            val,
            false,
        )?;
        Ok(())
    }
}

impl<T: IC10Marker> LInstruction for T {
    /// l r? d? logicType
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        d: &InstOperand,
        logic_type: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let (device, connection) = d.as_device(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let val = self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_logicable()
                        .ok_or(ICError::NotLogicable(*obj_ref.get_id()))
                        .and_then(|logicable| {
                            if !logicable.can_logic_read(logic_type) {
                                Err(LogicError::CantRead(logic_type).into())
                            } else {
                                logicable.get_logic(logic_type).map_err(Into::into)
                            }
                        })
                })
            })?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> LdInstruction for T {
    /// ld r? id(r?|num) logicType
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        id: &InstOperand,
        logic_type: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let id = id.as_value(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let val = self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_id(id as ObjectID, None)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_logicable()
                        .ok_or(ICError::NotLogicable(*obj_ref.get_id()))
                        .and_then(|logicable| {
                            if !logicable.can_logic_read(logic_type) {
                                Err(LogicError::CantRead(logic_type).into())
                            } else {
                                logicable.get_logic(logic_type).map_err(Into::into)
                            }
                        })
                })
            })?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> LsInstruction for T {
    /// ls r? d? slotIndex logicSlotType
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        d: &InstOperand,
        slot_index: &InstOperand,
        logic_slot_type: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let (device, connection) = d.as_device(self)?;
        let slot_index = slot_index.as_value(self)?;
        let logic_slot_type = logic_slot_type.as_slot_logic_type(self)?;
        let val = self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_logicable()
                        .ok_or(ICError::NotLogicable(*obj_ref.get_id()))
                        .and_then(|logicable| {
                            if !logicable.can_slot_logic_read(logic_slot_type, slot_index) {
                                Err(LogicError::CantSlotRead(logic_slot_type, slot_index).into())
                            } else {
                                logicable
                                    .get_slot_logic(logic_slot_type, slot_index)
                                    .map_err(Into::into)
                            }
                        })
                })
            })?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> LrInstruction for T {
    /// lr r? d? reagentMode int
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        d: &InstOperand,
        reagent_mode: &InstOperand,
        int: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let (device, connection) = d.as_device(self)?;
        let reagent_mode = reagent_mode.as_reagent_mode(self)?;
        let int = int.as_value(self)?;
        let val = self
            .get_circuit_holder()
            .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
            .borrow()
            .as_circuit_holder()
            .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
            .get_logicable_from_index(device, connection)
            .ok_or(ICError::DeviceNotSet)
            .and_then(|obj| {
                obj.map(|obj_ref| {
                    obj_ref
                        .as_logicable()
                        .ok_or(ICError::NotLogicable(*obj_ref.get_id()))
                        .and_then(|logicable| {
                            let result = match reagent_mode {
                                LogicReagentMode::Contents => {
                                    let device = logicable
                                        .as_device()
                                        .ok_or(ICError::NotReagentReadable(*logicable.get_id()))?;
                                    device
                                        .get_reagents()
                                        .iter()
                                        .find(|(hash, _)| *hash as f64 == int)
                                        .map(|(_, quantity)| *quantity)
                                        .unwrap_or(0.0)
                                }
                                LogicReagentMode::TotalContents => {
                                    let device = logicable
                                        .as_device()
                                        .ok_or(ICError::NotReagentReadable(*logicable.get_id()))?;
                                    device
                                        .get_reagents()
                                        .iter()
                                        .map(|(_, quantity)| quantity)
                                        .sum()
                                }
                                LogicReagentMode::Required => {
                                    let reagent_interface = logicable
                                        .as_reagent_interface()
                                        .ok_or(ICError::NotReagentReadable(*logicable.get_id()))?;
                                    reagent_interface
                                        .get_current_required()
                                        .iter()
                                        .find(|(hash, _)| *hash as f64 == int)
                                        .map(|(_, quantity)| *quantity)
                                        .unwrap_or(0.0)
                                }
                                LogicReagentMode::Recipe => {
                                    let reagent_interface = logicable
                                        .as_reagent_interface()
                                        .ok_or(ICError::NotReagentReadable(*logicable.get_id()))?;
                                    reagent_interface
                                        .get_current_recipie()
                                        .iter()
                                        .find(|(hash, _)| *hash as f64 == int)
                                        .map(|(_, quantity)| *quantity)
                                        .unwrap_or(0.0)
                                }
                            };
                            Ok(result)
                        })
                })
            })?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> LbInstruction for T {
    /// lb r? deviceHash logicType batchMode
    fn execute_inner(
        &mut self,
        r: &InstOperand,
        device_hash: &InstOperand,
        logic_type: &InstOperand,
        batch_mode: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;

        let prefab = device_hash.as_value(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let batch_mode = batch_mode.as_batch_mode(self)?;
        let val =
            self.get_vm()
                .get_batch_device_field(*self.get_id(), prefab, logic_type, batch_mode)?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> LbnInstruction for T {
    /// lbn r? deviceHash nameHash logicType batchMode
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        device_hash: &InstOperand,
        name_hash: &InstOperand,
        logic_type: &InstOperand,
        batch_mode: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let prefab = device_hash.as_value(self)?;
        let name = name_hash.as_value(self)?;
        let logic_type = logic_type.as_logic_type(self)?;
        let batch_mode = batch_mode.as_batch_mode(self)?;
        let val = self.get_vm().get_batch_name_device_field(
            *self.get_id(),
            prefab,
            name,
            logic_type,
            batch_mode,
        )?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> LbnsInstruction for T {
    /// lbns r? deviceHash nameHash slotIndex logicSlotType batchMode
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        device_hash: &InstOperand,
        name_hash: &InstOperand,
        slot_index: &InstOperand,
        logic_slot_type: &InstOperand,
        batch_mode: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let prefab = device_hash.as_value(self)?;
        let name = name_hash.as_value(self)?;
        let slot_index = slot_index.as_value(self)?;
        let logic_slot_type = logic_slot_type.as_slot_logic_type(self)?;
        let batch_mode = batch_mode.as_batch_mode(self)?;
        let val = self.get_vm().get_batch_name_device_slot_field(
            *self.get_id(),
            prefab,
            name,
            slot_index,
            logic_slot_type,
            batch_mode,
        )?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> LbsInstruction for T {
    /// lbs r? deviceHash slotIndex logicSlotType batchMode
    fn execute_inner(
        &mut self,

        r: &InstOperand,
        device_hash: &InstOperand,
        slot_index: &InstOperand,
        logic_slot_type: &InstOperand,
        batch_mode: &InstOperand,
    ) -> Result<(), ICError> {
        let RegisterSpec {
            indirection,
            target,
        } = r.as_register(self)?;
        let prefab = device_hash.as_value(self)?;
        let slot_index = slot_index.as_value(self)?;
        let logic_slot_type = logic_slot_type.as_slot_logic_type(self)?;
        let batch_mode = batch_mode.as_batch_mode(self)?;
        let val = self.get_vm().get_batch_device_slot_field(
            *self.get_id(),
            prefab,
            slot_index,
            logic_slot_type,
            batch_mode,
        )?;
        self.set_register(indirection, target, val)?;
        Ok(())
    }
}

impl<T: IC10Marker> HcfInstruction for T {
    /// hcf
    fn execute_inner(&mut self) -> Result<(), ICError> {
        {
            self.get_circuit_holder()
                .ok_or(ICError::NoCircuitHolder(*self.get_id()))?
                .borrow_mut()
                .as_mut_circuit_holder()
                .ok_or(ICError::CircuitHolderNotLogicable(*self.get_id()))?
                .hault_and_catch_fire();
        }
        self.set_state(ICState::HasCaughtFire);
        Ok(())
    }
}

impl<T: IC10Marker> LabelInstruction for T {
    /// label d? str
    fn execute_inner(&mut self, _d: &InstOperand, _str: &InstOperand) -> Result<(), ICError> {
        // No op, handled by program compilation, should never be called?
        Ok(())
    }
}
