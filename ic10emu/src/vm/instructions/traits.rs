use crate::vm::object::traits::IntegratedCircuit;
use crate::vm::instructions::enums::InstructionOp;
pub trait AbsInstruction: IntegratedCircuit {
    ///abs r? a(r?|num)
    fn execute_abs(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        AbsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Abs,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Abs,
                1usize,
            ),
        )
    }
    ///abs r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait AcosInstruction: IntegratedCircuit {
    ///acos r? a(r?|num)
    fn execute_acos(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        AcosInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Acos,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Acos,
                1usize,
            ),
        )
    }
    ///acos r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait AddInstruction: IntegratedCircuit {
    ///add r? a(r?|num) b(r?|num)
    fn execute_add(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        AddInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Add,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Add,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Add,
                2usize,
            ),
        )
    }
    ///add r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait AliasInstruction: IntegratedCircuit {
    ///alias str r?|d?
    fn execute_alias(
        &mut self,
        string: &crate::vm::instructions::operands::Operand,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        AliasInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                string,
                InstructionOp::Alias,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Alias,
                1usize,
            ),
        )
    }
    ///alias str r?|d?
    fn execute_inner(
        &mut self,
        string: &crate::vm::instructions::operands::InstOperand,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait AndInstruction: IntegratedCircuit {
    ///and r? a(r?|num) b(r?|num)
    fn execute_and(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        AndInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::And,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::And,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::And,
                2usize,
            ),
        )
    }
    ///and r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait AsinInstruction: IntegratedCircuit {
    ///asin r? a(r?|num)
    fn execute_asin(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        AsinInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Asin,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Asin,
                1usize,
            ),
        )
    }
    ///asin r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait AtanInstruction: IntegratedCircuit {
    ///atan r? a(r?|num)
    fn execute_atan(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        AtanInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Atan,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Atan,
                1usize,
            ),
        )
    }
    ///atan r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait Atan2Instruction: IntegratedCircuit {
    ///atan2 r? a(r?|num) b(r?|num)
    fn execute_atan2(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        Atan2Instruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Atan2,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Atan2,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Atan2,
                2usize,
            ),
        )
    }
    ///atan2 r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BapInstruction: IntegratedCircuit {
    ///bap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bap(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BapInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bap,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bap,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bap,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bap,
                3usize,
            ),
        )
    }
    ///bap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BapalInstruction: IntegratedCircuit {
    ///bapal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bapal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BapalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bapal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bapal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bapal,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bapal,
                3usize,
            ),
        )
    }
    ///bapal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BapzInstruction: IntegratedCircuit {
    ///bapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_bapz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BapzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bapz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bapz,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bapz,
                2usize,
            ),
        )
    }
    ///bapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BapzalInstruction: IntegratedCircuit {
    ///bapzal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bapzal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BapzalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bapzal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bapzal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bapzal,
                2usize,
            ),
        )
    }
    ///bapzal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BdnsInstruction: IntegratedCircuit {
    ///bdns d? a(r?|num)
    fn execute_bdns(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BdnsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bdns,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bdns,
                1usize,
            ),
        )
    }
    ///bdns d? a(r?|num)
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BdnsalInstruction: IntegratedCircuit {
    ///bdnsal d? a(r?|num)
    fn execute_bdnsal(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BdnsalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bdnsal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bdnsal,
                1usize,
            ),
        )
    }
    ///bdnsal d? a(r?|num)
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BdseInstruction: IntegratedCircuit {
    ///bdse d? a(r?|num)
    fn execute_bdse(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BdseInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bdse,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bdse,
                1usize,
            ),
        )
    }
    ///bdse d? a(r?|num)
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BdsealInstruction: IntegratedCircuit {
    ///bdseal d? a(r?|num)
    fn execute_bdseal(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BdsealInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bdseal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bdseal,
                1usize,
            ),
        )
    }
    ///bdseal d? a(r?|num)
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BeqInstruction: IntegratedCircuit {
    ///beq a(r?|num) b(r?|num) c(r?|num)
    fn execute_beq(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BeqInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Beq,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Beq,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Beq,
                2usize,
            ),
        )
    }
    ///beq a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BeqalInstruction: IntegratedCircuit {
    ///beqal a(r?|num) b(r?|num) c(r?|num)
    fn execute_beqal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BeqalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Beqal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Beqal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Beqal,
                2usize,
            ),
        )
    }
    ///beqal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BeqzInstruction: IntegratedCircuit {
    ///beqz a(r?|num) b(r?|num)
    fn execute_beqz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BeqzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Beqz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Beqz,
                1usize,
            ),
        )
    }
    ///beqz a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BeqzalInstruction: IntegratedCircuit {
    ///beqzal a(r?|num) b(r?|num)
    fn execute_beqzal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BeqzalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Beqzal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Beqzal,
                1usize,
            ),
        )
    }
    ///beqzal a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgeInstruction: IntegratedCircuit {
    ///bge a(r?|num) b(r?|num) c(r?|num)
    fn execute_bge(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgeInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bge,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bge,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bge,
                2usize,
            ),
        )
    }
    ///bge a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgealInstruction: IntegratedCircuit {
    ///bgeal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bgeal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgealInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bgeal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bgeal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bgeal,
                2usize,
            ),
        )
    }
    ///bgeal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgezInstruction: IntegratedCircuit {
    ///bgez a(r?|num) b(r?|num)
    fn execute_bgez(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bgez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bgez,
                1usize,
            ),
        )
    }
    ///bgez a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgezalInstruction: IntegratedCircuit {
    ///bgezal a(r?|num) b(r?|num)
    fn execute_bgezal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgezalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bgezal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bgezal,
                1usize,
            ),
        )
    }
    ///bgezal a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgtInstruction: IntegratedCircuit {
    ///bgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_bgt(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgtInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bgt,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bgt,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bgt,
                2usize,
            ),
        )
    }
    ///bgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgtalInstruction: IntegratedCircuit {
    ///bgtal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bgtal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgtalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bgtal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bgtal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bgtal,
                2usize,
            ),
        )
    }
    ///bgtal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgtzInstruction: IntegratedCircuit {
    ///bgtz a(r?|num) b(r?|num)
    fn execute_bgtz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgtzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bgtz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bgtz,
                1usize,
            ),
        )
    }
    ///bgtz a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BgtzalInstruction: IntegratedCircuit {
    ///bgtzal a(r?|num) b(r?|num)
    fn execute_bgtzal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BgtzalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bgtzal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bgtzal,
                1usize,
            ),
        )
    }
    ///bgtzal a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BleInstruction: IntegratedCircuit {
    ///ble a(r?|num) b(r?|num) c(r?|num)
    fn execute_ble(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BleInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Ble,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Ble,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Ble,
                2usize,
            ),
        )
    }
    ///ble a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BlealInstruction: IntegratedCircuit {
    ///bleal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bleal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BlealInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bleal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bleal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bleal,
                2usize,
            ),
        )
    }
    ///bleal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BlezInstruction: IntegratedCircuit {
    ///blez a(r?|num) b(r?|num)
    fn execute_blez(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BlezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Blez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Blez,
                1usize,
            ),
        )
    }
    ///blez a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BlezalInstruction: IntegratedCircuit {
    ///blezal a(r?|num) b(r?|num)
    fn execute_blezal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BlezalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Blezal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Blezal,
                1usize,
            ),
        )
    }
    ///blezal a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BltInstruction: IntegratedCircuit {
    ///blt a(r?|num) b(r?|num) c(r?|num)
    fn execute_blt(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BltInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Blt,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Blt,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Blt,
                2usize,
            ),
        )
    }
    ///blt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BltalInstruction: IntegratedCircuit {
    ///bltal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bltal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BltalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bltal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bltal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bltal,
                2usize,
            ),
        )
    }
    ///bltal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BltzInstruction: IntegratedCircuit {
    ///bltz a(r?|num) b(r?|num)
    fn execute_bltz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BltzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bltz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bltz,
                1usize,
            ),
        )
    }
    ///bltz a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BltzalInstruction: IntegratedCircuit {
    ///bltzal a(r?|num) b(r?|num)
    fn execute_bltzal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BltzalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bltzal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bltzal,
                1usize,
            ),
        )
    }
    ///bltzal a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnaInstruction: IntegratedCircuit {
    ///bna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bna(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnaInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bna,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bna,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bna,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bna,
                3usize,
            ),
        )
    }
    ///bna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnaalInstruction: IntegratedCircuit {
    ///bnaal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_bnaal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnaalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bnaal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bnaal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bnaal,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Bnaal,
                3usize,
            ),
        )
    }
    ///bnaal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnanInstruction: IntegratedCircuit {
    ///bnan a(r?|num) b(r?|num)
    fn execute_bnan(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnanInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bnan,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bnan,
                1usize,
            ),
        )
    }
    ///bnan a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnazInstruction: IntegratedCircuit {
    ///bnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_bnaz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnazInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bnaz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bnaz,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bnaz,
                2usize,
            ),
        )
    }
    ///bnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnazalInstruction: IntegratedCircuit {
    ///bnazal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bnazal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnazalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bnazal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bnazal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bnazal,
                2usize,
            ),
        )
    }
    ///bnazal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BneInstruction: IntegratedCircuit {
    ///bne a(r?|num) b(r?|num) c(r?|num)
    fn execute_bne(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BneInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bne,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bne,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bne,
                2usize,
            ),
        )
    }
    ///bne a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnealInstruction: IntegratedCircuit {
    ///bneal a(r?|num) b(r?|num) c(r?|num)
    fn execute_bneal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnealInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bneal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bneal,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Bneal,
                2usize,
            ),
        )
    }
    ///bneal a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnezInstruction: IntegratedCircuit {
    ///bnez a(r?|num) b(r?|num)
    fn execute_bnez(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bnez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bnez,
                1usize,
            ),
        )
    }
    ///bnez a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BnezalInstruction: IntegratedCircuit {
    ///bnezal a(r?|num) b(r?|num)
    fn execute_bnezal(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BnezalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Bnezal,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Bnezal,
                1usize,
            ),
        )
    }
    ///bnezal a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrapInstruction: IntegratedCircuit {
    ///brap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_brap(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrapInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brap,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brap,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brap,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Brap,
                3usize,
            ),
        )
    }
    ///brap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrapzInstruction: IntegratedCircuit {
    ///brapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_brapz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrapzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brapz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brapz,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brapz,
                2usize,
            ),
        )
    }
    ///brapz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrdnsInstruction: IntegratedCircuit {
    ///brdns d? a(r?|num)
    fn execute_brdns(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrdnsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Brdns,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brdns,
                1usize,
            ),
        )
    }
    ///brdns d? a(r?|num)
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrdseInstruction: IntegratedCircuit {
    ///brdse d? a(r?|num)
    fn execute_brdse(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrdseInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Brdse,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brdse,
                1usize,
            ),
        )
    }
    ///brdse d? a(r?|num)
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BreqInstruction: IntegratedCircuit {
    ///breq a(r?|num) b(r?|num) c(r?|num)
    fn execute_breq(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BreqInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Breq,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Breq,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Breq,
                2usize,
            ),
        )
    }
    ///breq a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BreqzInstruction: IntegratedCircuit {
    ///breqz a(r?|num) b(r?|num)
    fn execute_breqz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BreqzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Breqz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Breqz,
                1usize,
            ),
        )
    }
    ///breqz a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrgeInstruction: IntegratedCircuit {
    ///brge a(r?|num) b(r?|num) c(r?|num)
    fn execute_brge(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrgeInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brge,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brge,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brge,
                2usize,
            ),
        )
    }
    ///brge a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrgezInstruction: IntegratedCircuit {
    ///brgez a(r?|num) b(r?|num)
    fn execute_brgez(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrgezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brgez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brgez,
                1usize,
            ),
        )
    }
    ///brgez a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrgtInstruction: IntegratedCircuit {
    ///brgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_brgt(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrgtInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brgt,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brgt,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brgt,
                2usize,
            ),
        )
    }
    ///brgt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrgtzInstruction: IntegratedCircuit {
    ///brgtz a(r?|num) b(r?|num)
    fn execute_brgtz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrgtzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brgtz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brgtz,
                1usize,
            ),
        )
    }
    ///brgtz a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrleInstruction: IntegratedCircuit {
    ///brle a(r?|num) b(r?|num) c(r?|num)
    fn execute_brle(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrleInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brle,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brle,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brle,
                2usize,
            ),
        )
    }
    ///brle a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrlezInstruction: IntegratedCircuit {
    ///brlez a(r?|num) b(r?|num)
    fn execute_brlez(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrlezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brlez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brlez,
                1usize,
            ),
        )
    }
    ///brlez a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrltInstruction: IntegratedCircuit {
    ///brlt a(r?|num) b(r?|num) c(r?|num)
    fn execute_brlt(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrltInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brlt,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brlt,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brlt,
                2usize,
            ),
        )
    }
    ///brlt a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrltzInstruction: IntegratedCircuit {
    ///brltz a(r?|num) b(r?|num)
    fn execute_brltz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrltzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brltz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brltz,
                1usize,
            ),
        )
    }
    ///brltz a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrnaInstruction: IntegratedCircuit {
    ///brna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_brna(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrnaInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brna,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brna,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brna,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Brna,
                3usize,
            ),
        )
    }
    ///brna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrnanInstruction: IntegratedCircuit {
    ///brnan a(r?|num) b(r?|num)
    fn execute_brnan(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrnanInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brnan,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brnan,
                1usize,
            ),
        )
    }
    ///brnan a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrnazInstruction: IntegratedCircuit {
    ///brnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_brnaz(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrnazInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brnaz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brnaz,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brnaz,
                2usize,
            ),
        )
    }
    ///brnaz a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrneInstruction: IntegratedCircuit {
    ///brne a(r?|num) b(r?|num) c(r?|num)
    fn execute_brne(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrneInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brne,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brne,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Brne,
                2usize,
            ),
        )
    }
    ///brne a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait BrnezInstruction: IntegratedCircuit {
    ///brnez a(r?|num) b(r?|num)
    fn execute_brnez(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        BrnezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Brnez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Brnez,
                1usize,
            ),
        )
    }
    ///brnez a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait CeilInstruction: IntegratedCircuit {
    ///ceil r? a(r?|num)
    fn execute_ceil(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        CeilInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Ceil,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Ceil,
                1usize,
            ),
        )
    }
    ///ceil r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait ClrInstruction: IntegratedCircuit {
    ///clr d?
    fn execute_clr(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        ClrInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Clr,
                0usize,
            ),
        )
    }
    ///clr d?
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait ClrdInstruction: IntegratedCircuit {
    ///clrd id(r?|num)
    fn execute_clrd(
        &mut self,
        id: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        ClrdInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                id,
                InstructionOp::Clrd,
                0usize,
            ),
        )
    }
    ///clrd id(r?|num)
    fn execute_inner(
        &mut self,
        id: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait CosInstruction: IntegratedCircuit {
    ///cos r? a(r?|num)
    fn execute_cos(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        CosInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Cos,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Cos,
                1usize,
            ),
        )
    }
    ///cos r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait DefineInstruction: IntegratedCircuit {
    ///define str num
    fn execute_define(
        &mut self,
        string: &crate::vm::instructions::operands::Operand,
        num: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        DefineInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                string,
                InstructionOp::Define,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                num,
                InstructionOp::Define,
                1usize,
            ),
        )
    }
    ///define str num
    fn execute_inner(
        &mut self,
        string: &crate::vm::instructions::operands::InstOperand,
        num: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait DivInstruction: IntegratedCircuit {
    ///div r? a(r?|num) b(r?|num)
    fn execute_div(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        DivInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Div,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Div,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Div,
                2usize,
            ),
        )
    }
    ///div r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait ExpInstruction: IntegratedCircuit {
    ///exp r? a(r?|num)
    fn execute_exp(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        ExpInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Exp,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Exp,
                1usize,
            ),
        )
    }
    ///exp r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait FloorInstruction: IntegratedCircuit {
    ///floor r? a(r?|num)
    fn execute_floor(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        FloorInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Floor,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Floor,
                1usize,
            ),
        )
    }
    ///floor r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait GetInstruction: IntegratedCircuit {
    ///get r? d? address(r?|num)
    fn execute_get(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
        address: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        GetInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Get,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Get,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                address,
                InstructionOp::Get,
                2usize,
            ),
        )
    }
    ///get r? d? address(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
        address: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait GetdInstruction: IntegratedCircuit {
    ///getd r? id(r?|num) address(r?|num)
    fn execute_getd(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        id: &crate::vm::instructions::operands::Operand,
        address: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        GetdInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Getd,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                id,
                InstructionOp::Getd,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                address,
                InstructionOp::Getd,
                2usize,
            ),
        )
    }
    ///getd r? id(r?|num) address(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        id: &crate::vm::instructions::operands::InstOperand,
        address: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait HcfInstruction: IntegratedCircuit {
    ///hcf
    fn execute_hcf(&mut self) -> Result<(), crate::errors::ICError> {
        HcfInstruction::execute_inner(self)
    }
    ///hcf
    fn execute_inner(&mut self) -> Result<(), crate::errors::ICError>;
}
pub trait JInstruction: IntegratedCircuit {
    ///j int
    fn execute_j(
        &mut self,
        int: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        JInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                int,
                InstructionOp::J,
                0usize,
            ),
        )
    }
    ///j int
    fn execute_inner(
        &mut self,
        int: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait JalInstruction: IntegratedCircuit {
    ///jal int
    fn execute_jal(
        &mut self,
        int: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        JalInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                int,
                InstructionOp::Jal,
                0usize,
            ),
        )
    }
    ///jal int
    fn execute_inner(
        &mut self,
        int: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait JrInstruction: IntegratedCircuit {
    ///jr int
    fn execute_jr(
        &mut self,
        int: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        JrInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                int,
                InstructionOp::Jr,
                0usize,
            ),
        )
    }
    ///jr int
    fn execute_inner(
        &mut self,
        int: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LInstruction: IntegratedCircuit {
    ///l r? d? logicType
    fn execute_l(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::L,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::L,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::L,
                2usize,
            ),
        )
    }
    ///l r? d? logicType
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LabelInstruction: IntegratedCircuit {
    ///label d? str
    fn execute_label(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        string: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LabelInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Label,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                string,
                InstructionOp::Label,
                1usize,
            ),
        )
    }
    ///label d? str
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        string: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LbInstruction: IntegratedCircuit {
    ///lb r? deviceHash logicType batchMode
    fn execute_lb(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        device_hash: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
        batch_mode: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LbInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Lb,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                device_hash,
                InstructionOp::Lb,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::Lb,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                batch_mode,
                InstructionOp::Lb,
                3usize,
            ),
        )
    }
    ///lb r? deviceHash logicType batchMode
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        device_hash: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
        batch_mode: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LbnInstruction: IntegratedCircuit {
    ///lbn r? deviceHash nameHash logicType batchMode
    fn execute_lbn(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        device_hash: &crate::vm::instructions::operands::Operand,
        name_hash: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
        batch_mode: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LbnInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Lbn,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                device_hash,
                InstructionOp::Lbn,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                name_hash,
                InstructionOp::Lbn,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::Lbn,
                3usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                batch_mode,
                InstructionOp::Lbn,
                4usize,
            ),
        )
    }
    ///lbn r? deviceHash nameHash logicType batchMode
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        device_hash: &crate::vm::instructions::operands::InstOperand,
        name_hash: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
        batch_mode: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LbnsInstruction: IntegratedCircuit {
    ///lbns r? deviceHash nameHash slotIndex logicSlotType batchMode
    fn execute_lbns(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        device_hash: &crate::vm::instructions::operands::Operand,
        name_hash: &crate::vm::instructions::operands::Operand,
        slot_index: &crate::vm::instructions::operands::Operand,
        logic_slot_type: &crate::vm::instructions::operands::Operand,
        batch_mode: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LbnsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Lbns,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                device_hash,
                InstructionOp::Lbns,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                name_hash,
                InstructionOp::Lbns,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                slot_index,
                InstructionOp::Lbns,
                3usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_slot_type,
                InstructionOp::Lbns,
                4usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                batch_mode,
                InstructionOp::Lbns,
                5usize,
            ),
        )
    }
    ///lbns r? deviceHash nameHash slotIndex logicSlotType batchMode
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        device_hash: &crate::vm::instructions::operands::InstOperand,
        name_hash: &crate::vm::instructions::operands::InstOperand,
        slot_index: &crate::vm::instructions::operands::InstOperand,
        logic_slot_type: &crate::vm::instructions::operands::InstOperand,
        batch_mode: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LbsInstruction: IntegratedCircuit {
    ///lbs r? deviceHash slotIndex logicSlotType batchMode
    fn execute_lbs(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        device_hash: &crate::vm::instructions::operands::Operand,
        slot_index: &crate::vm::instructions::operands::Operand,
        logic_slot_type: &crate::vm::instructions::operands::Operand,
        batch_mode: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LbsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Lbs,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                device_hash,
                InstructionOp::Lbs,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                slot_index,
                InstructionOp::Lbs,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_slot_type,
                InstructionOp::Lbs,
                3usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                batch_mode,
                InstructionOp::Lbs,
                4usize,
            ),
        )
    }
    ///lbs r? deviceHash slotIndex logicSlotType batchMode
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        device_hash: &crate::vm::instructions::operands::InstOperand,
        slot_index: &crate::vm::instructions::operands::InstOperand,
        logic_slot_type: &crate::vm::instructions::operands::InstOperand,
        batch_mode: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LdInstruction: IntegratedCircuit {
    ///ld r? id(r?|num) logicType
    fn execute_ld(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        id: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LdInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Ld,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                id,
                InstructionOp::Ld,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::Ld,
                2usize,
            ),
        )
    }
    ///ld r? id(r?|num) logicType
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        id: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LogInstruction: IntegratedCircuit {
    ///log r? a(r?|num)
    fn execute_log(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LogInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Log,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Log,
                1usize,
            ),
        )
    }
    ///log r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LrInstruction: IntegratedCircuit {
    ///lr r? d? reagentMode int
    fn execute_lr(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
        reagent_mode: &crate::vm::instructions::operands::Operand,
        int: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LrInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Lr,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Lr,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                reagent_mode,
                InstructionOp::Lr,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                int,
                InstructionOp::Lr,
                3usize,
            ),
        )
    }
    ///lr r? d? reagentMode int
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
        reagent_mode: &crate::vm::instructions::operands::InstOperand,
        int: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait LsInstruction: IntegratedCircuit {
    ///ls r? d? slotIndex logicSlotType
    fn execute_ls(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
        slot_index: &crate::vm::instructions::operands::Operand,
        logic_slot_type: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        LsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Ls,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Ls,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                slot_index,
                InstructionOp::Ls,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_slot_type,
                InstructionOp::Ls,
                3usize,
            ),
        )
    }
    ///ls r? d? slotIndex logicSlotType
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
        slot_index: &crate::vm::instructions::operands::InstOperand,
        logic_slot_type: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait MaxInstruction: IntegratedCircuit {
    ///max r? a(r?|num) b(r?|num)
    fn execute_max(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        MaxInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Max,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Max,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Max,
                2usize,
            ),
        )
    }
    ///max r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait MinInstruction: IntegratedCircuit {
    ///min r? a(r?|num) b(r?|num)
    fn execute_min(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        MinInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Min,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Min,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Min,
                2usize,
            ),
        )
    }
    ///min r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait ModInstruction: IntegratedCircuit {
    ///mod r? a(r?|num) b(r?|num)
    fn execute_mod(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        ModInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Mod,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Mod,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Mod,
                2usize,
            ),
        )
    }
    ///mod r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait MoveInstruction: IntegratedCircuit {
    ///move r? a(r?|num)
    fn execute_move(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        MoveInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Move,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Move,
                1usize,
            ),
        )
    }
    ///move r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait MulInstruction: IntegratedCircuit {
    ///mul r? a(r?|num) b(r?|num)
    fn execute_mul(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        MulInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Mul,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Mul,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Mul,
                2usize,
            ),
        )
    }
    ///mul r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait NorInstruction: IntegratedCircuit {
    ///nor r? a(r?|num) b(r?|num)
    fn execute_nor(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        NorInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Nor,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Nor,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Nor,
                2usize,
            ),
        )
    }
    ///nor r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait NotInstruction: IntegratedCircuit {
    ///not r? a(r?|num)
    fn execute_not(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        NotInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Not,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Not,
                1usize,
            ),
        )
    }
    ///not r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait OrInstruction: IntegratedCircuit {
    ///or r? a(r?|num) b(r?|num)
    fn execute_or(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        OrInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Or,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Or,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Or,
                2usize,
            ),
        )
    }
    ///or r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait PeekInstruction: IntegratedCircuit {
    ///peek r?
    fn execute_peek(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        PeekInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Peek,
                0usize,
            ),
        )
    }
    ///peek r?
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait PokeInstruction: IntegratedCircuit {
    ///poke address(r?|num) value(r?|num)
    fn execute_poke(
        &mut self,
        address: &crate::vm::instructions::operands::Operand,
        value: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        PokeInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                address,
                InstructionOp::Poke,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                value,
                InstructionOp::Poke,
                1usize,
            ),
        )
    }
    ///poke address(r?|num) value(r?|num)
    fn execute_inner(
        &mut self,
        address: &crate::vm::instructions::operands::InstOperand,
        value: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait PopInstruction: IntegratedCircuit {
    ///pop r?
    fn execute_pop(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        PopInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Pop,
                0usize,
            ),
        )
    }
    ///pop r?
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait PushInstruction: IntegratedCircuit {
    ///push a(r?|num)
    fn execute_push(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        PushInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Push,
                0usize,
            ),
        )
    }
    ///push a(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait PutInstruction: IntegratedCircuit {
    ///put d? address(r?|num) value(r?|num)
    fn execute_put(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        address: &crate::vm::instructions::operands::Operand,
        value: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        PutInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Put,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                address,
                InstructionOp::Put,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                value,
                InstructionOp::Put,
                2usize,
            ),
        )
    }
    ///put d? address(r?|num) value(r?|num)
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        address: &crate::vm::instructions::operands::InstOperand,
        value: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait PutdInstruction: IntegratedCircuit {
    ///putd id(r?|num) address(r?|num) value(r?|num)
    fn execute_putd(
        &mut self,
        id: &crate::vm::instructions::operands::Operand,
        address: &crate::vm::instructions::operands::Operand,
        value: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        PutdInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                id,
                InstructionOp::Putd,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                address,
                InstructionOp::Putd,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                value,
                InstructionOp::Putd,
                2usize,
            ),
        )
    }
    ///putd id(r?|num) address(r?|num) value(r?|num)
    fn execute_inner(
        &mut self,
        id: &crate::vm::instructions::operands::InstOperand,
        address: &crate::vm::instructions::operands::InstOperand,
        value: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait RandInstruction: IntegratedCircuit {
    ///rand r?
    fn execute_rand(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        RandInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Rand,
                0usize,
            ),
        )
    }
    ///rand r?
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait RoundInstruction: IntegratedCircuit {
    ///round r? a(r?|num)
    fn execute_round(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        RoundInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Round,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Round,
                1usize,
            ),
        )
    }
    ///round r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SInstruction: IntegratedCircuit {
    ///s d? logicType r?
    fn execute_s(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::S,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::S,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::S,
                2usize,
            ),
        )
    }
    ///s d? logicType r?
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SapInstruction: IntegratedCircuit {
    ///sap r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_sap(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SapInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sap,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sap,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sap,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Sap,
                3usize,
            ),
        )
    }
    ///sap r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SapzInstruction: IntegratedCircuit {
    ///sapz r? a(r?|num) b(r?|num)
    fn execute_sapz(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SapzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sapz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sapz,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sapz,
                2usize,
            ),
        )
    }
    ///sapz r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SbInstruction: IntegratedCircuit {
    ///sb deviceHash logicType r?
    fn execute_sb(
        &mut self,
        device_hash: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SbInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                device_hash,
                InstructionOp::Sb,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::Sb,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sb,
                2usize,
            ),
        )
    }
    ///sb deviceHash logicType r?
    fn execute_inner(
        &mut self,
        device_hash: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SbnInstruction: IntegratedCircuit {
    ///sbn deviceHash nameHash logicType r?
    fn execute_sbn(
        &mut self,
        device_hash: &crate::vm::instructions::operands::Operand,
        name_hash: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SbnInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                device_hash,
                InstructionOp::Sbn,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                name_hash,
                InstructionOp::Sbn,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::Sbn,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sbn,
                3usize,
            ),
        )
    }
    ///sbn deviceHash nameHash logicType r?
    fn execute_inner(
        &mut self,
        device_hash: &crate::vm::instructions::operands::InstOperand,
        name_hash: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SbsInstruction: IntegratedCircuit {
    ///sbs deviceHash slotIndex logicSlotType r?
    fn execute_sbs(
        &mut self,
        device_hash: &crate::vm::instructions::operands::Operand,
        slot_index: &crate::vm::instructions::operands::Operand,
        logic_slot_type: &crate::vm::instructions::operands::Operand,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SbsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                device_hash,
                InstructionOp::Sbs,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                slot_index,
                InstructionOp::Sbs,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_slot_type,
                InstructionOp::Sbs,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sbs,
                3usize,
            ),
        )
    }
    ///sbs deviceHash slotIndex logicSlotType r?
    fn execute_inner(
        &mut self,
        device_hash: &crate::vm::instructions::operands::InstOperand,
        slot_index: &crate::vm::instructions::operands::InstOperand,
        logic_slot_type: &crate::vm::instructions::operands::InstOperand,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SdInstruction: IntegratedCircuit {
    ///sd id(r?|num) logicType r?
    fn execute_sd(
        &mut self,
        id: &crate::vm::instructions::operands::Operand,
        logic_type: &crate::vm::instructions::operands::Operand,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SdInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                id,
                InstructionOp::Sd,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_type,
                InstructionOp::Sd,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sd,
                2usize,
            ),
        )
    }
    ///sd id(r?|num) logicType r?
    fn execute_inner(
        &mut self,
        id: &crate::vm::instructions::operands::InstOperand,
        logic_type: &crate::vm::instructions::operands::InstOperand,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SdnsInstruction: IntegratedCircuit {
    ///sdns r? d?
    fn execute_sdns(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SdnsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sdns,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Sdns,
                1usize,
            ),
        )
    }
    ///sdns r? d?
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SdseInstruction: IntegratedCircuit {
    ///sdse r? d?
    fn execute_sdse(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        d: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SdseInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sdse,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Sdse,
                1usize,
            ),
        )
    }
    ///sdse r? d?
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        d: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SelectInstruction: IntegratedCircuit {
    ///select r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_select(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SelectInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Select,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Select,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Select,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Select,
                3usize,
            ),
        )
    }
    ///select r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SeqInstruction: IntegratedCircuit {
    ///seq r? a(r?|num) b(r?|num)
    fn execute_seq(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SeqInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Seq,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Seq,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Seq,
                2usize,
            ),
        )
    }
    ///seq r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SeqzInstruction: IntegratedCircuit {
    ///seqz r? a(r?|num)
    fn execute_seqz(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SeqzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Seqz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Seqz,
                1usize,
            ),
        )
    }
    ///seqz r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SgeInstruction: IntegratedCircuit {
    ///sge r? a(r?|num) b(r?|num)
    fn execute_sge(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SgeInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sge,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sge,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sge,
                2usize,
            ),
        )
    }
    ///sge r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SgezInstruction: IntegratedCircuit {
    ///sgez r? a(r?|num)
    fn execute_sgez(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SgezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sgez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sgez,
                1usize,
            ),
        )
    }
    ///sgez r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SgtInstruction: IntegratedCircuit {
    ///sgt r? a(r?|num) b(r?|num)
    fn execute_sgt(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SgtInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sgt,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sgt,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sgt,
                2usize,
            ),
        )
    }
    ///sgt r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SgtzInstruction: IntegratedCircuit {
    ///sgtz r? a(r?|num)
    fn execute_sgtz(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SgtzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sgtz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sgtz,
                1usize,
            ),
        )
    }
    ///sgtz r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SinInstruction: IntegratedCircuit {
    ///sin r? a(r?|num)
    fn execute_sin(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SinInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sin,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sin,
                1usize,
            ),
        )
    }
    ///sin r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SlaInstruction: IntegratedCircuit {
    ///sla r? a(r?|num) b(r?|num)
    fn execute_sla(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SlaInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sla,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sla,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sla,
                2usize,
            ),
        )
    }
    ///sla r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SleInstruction: IntegratedCircuit {
    ///sle r? a(r?|num) b(r?|num)
    fn execute_sle(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SleInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sle,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sle,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sle,
                2usize,
            ),
        )
    }
    ///sle r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SleepInstruction: IntegratedCircuit {
    ///sleep a(r?|num)
    fn execute_sleep(
        &mut self,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SleepInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sleep,
                0usize,
            ),
        )
    }
    ///sleep a(r?|num)
    fn execute_inner(
        &mut self,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SlezInstruction: IntegratedCircuit {
    ///slez r? a(r?|num)
    fn execute_slez(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SlezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Slez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Slez,
                1usize,
            ),
        )
    }
    ///slez r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SllInstruction: IntegratedCircuit {
    ///sll r? a(r?|num) b(r?|num)
    fn execute_sll(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SllInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sll,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sll,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sll,
                2usize,
            ),
        )
    }
    ///sll r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SltInstruction: IntegratedCircuit {
    ///slt r? a(r?|num) b(r?|num)
    fn execute_slt(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SltInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Slt,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Slt,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Slt,
                2usize,
            ),
        )
    }
    ///slt r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SltzInstruction: IntegratedCircuit {
    ///sltz r? a(r?|num)
    fn execute_sltz(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SltzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sltz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sltz,
                1usize,
            ),
        )
    }
    ///sltz r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SnaInstruction: IntegratedCircuit {
    ///sna r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_sna(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
        c: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SnaInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sna,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sna,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sna,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                c,
                InstructionOp::Sna,
                3usize,
            ),
        )
    }
    ///sna r? a(r?|num) b(r?|num) c(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
        c: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SnanInstruction: IntegratedCircuit {
    ///snan r? a(r?|num)
    fn execute_snan(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SnanInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Snan,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Snan,
                1usize,
            ),
        )
    }
    ///snan r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SnanzInstruction: IntegratedCircuit {
    ///snanz r? a(r?|num)
    fn execute_snanz(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SnanzInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Snanz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Snanz,
                1usize,
            ),
        )
    }
    ///snanz r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SnazInstruction: IntegratedCircuit {
    ///snaz r? a(r?|num) b(r?|num)
    fn execute_snaz(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SnazInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Snaz,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Snaz,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Snaz,
                2usize,
            ),
        )
    }
    ///snaz r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SneInstruction: IntegratedCircuit {
    ///sne r? a(r?|num) b(r?|num)
    fn execute_sne(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SneInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sne,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sne,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sne,
                2usize,
            ),
        )
    }
    ///sne r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SnezInstruction: IntegratedCircuit {
    ///snez r? a(r?|num)
    fn execute_snez(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SnezInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Snez,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Snez,
                1usize,
            ),
        )
    }
    ///snez r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SqrtInstruction: IntegratedCircuit {
    ///sqrt r? a(r?|num)
    fn execute_sqrt(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SqrtInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sqrt,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sqrt,
                1usize,
            ),
        )
    }
    ///sqrt r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SraInstruction: IntegratedCircuit {
    ///sra r? a(r?|num) b(r?|num)
    fn execute_sra(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SraInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sra,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sra,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sra,
                2usize,
            ),
        )
    }
    ///sra r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SrlInstruction: IntegratedCircuit {
    ///srl r? a(r?|num) b(r?|num)
    fn execute_srl(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SrlInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Srl,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Srl,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Srl,
                2usize,
            ),
        )
    }
    ///srl r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SsInstruction: IntegratedCircuit {
    ///ss d? slotIndex logicSlotType r?
    fn execute_ss(
        &mut self,
        d: &crate::vm::instructions::operands::Operand,
        slot_index: &crate::vm::instructions::operands::Operand,
        logic_slot_type: &crate::vm::instructions::operands::Operand,
        r: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SsInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                d,
                InstructionOp::Ss,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                slot_index,
                InstructionOp::Ss,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                logic_slot_type,
                InstructionOp::Ss,
                2usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Ss,
                3usize,
            ),
        )
    }
    ///ss d? slotIndex logicSlotType r?
    fn execute_inner(
        &mut self,
        d: &crate::vm::instructions::operands::InstOperand,
        slot_index: &crate::vm::instructions::operands::InstOperand,
        logic_slot_type: &crate::vm::instructions::operands::InstOperand,
        r: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait SubInstruction: IntegratedCircuit {
    ///sub r? a(r?|num) b(r?|num)
    fn execute_sub(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        SubInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Sub,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Sub,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Sub,
                2usize,
            ),
        )
    }
    ///sub r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait TanInstruction: IntegratedCircuit {
    ///tan r? a(r?|num)
    fn execute_tan(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        TanInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Tan,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Tan,
                1usize,
            ),
        )
    }
    ///tan r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait TruncInstruction: IntegratedCircuit {
    ///trunc r? a(r?|num)
    fn execute_trunc(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        TruncInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Trunc,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Trunc,
                1usize,
            ),
        )
    }
    ///trunc r? a(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait XorInstruction: IntegratedCircuit {
    ///xor r? a(r?|num) b(r?|num)
    fn execute_xor(
        &mut self,
        r: &crate::vm::instructions::operands::Operand,
        a: &crate::vm::instructions::operands::Operand,
        b: &crate::vm::instructions::operands::Operand,
    ) -> Result<(), crate::errors::ICError> {
        XorInstruction::execute_inner(
            self,
            &crate::vm::instructions::operands::InstOperand::new(
                r,
                InstructionOp::Xor,
                0usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                a,
                InstructionOp::Xor,
                1usize,
            ),
            &crate::vm::instructions::operands::InstOperand::new(
                b,
                InstructionOp::Xor,
                2usize,
            ),
        )
    }
    ///xor r? a(r?|num) b(r?|num)
    fn execute_inner(
        &mut self,
        r: &crate::vm::instructions::operands::InstOperand,
        a: &crate::vm::instructions::operands::InstOperand,
        b: &crate::vm::instructions::operands::InstOperand,
    ) -> Result<(), crate::errors::ICError>;
}
pub trait YieldInstruction: IntegratedCircuit {
    ///yield
    fn execute_yield(&mut self) -> Result<(), crate::errors::ICError> {
        YieldInstruction::execute_inner(self)
    }
    ///yield
    fn execute_inner(&mut self) -> Result<(), crate::errors::ICError>;
}
pub trait ICInstructable: AbsInstruction + AcosInstruction + AddInstruction + AliasInstruction + AndInstruction + AsinInstruction + AtanInstruction + Atan2Instruction + BapInstruction + BapalInstruction + BapzInstruction + BapzalInstruction + BdnsInstruction + BdnsalInstruction + BdseInstruction + BdsealInstruction + BeqInstruction + BeqalInstruction + BeqzInstruction + BeqzalInstruction + BgeInstruction + BgealInstruction + BgezInstruction + BgezalInstruction + BgtInstruction + BgtalInstruction + BgtzInstruction + BgtzalInstruction + BleInstruction + BlealInstruction + BlezInstruction + BlezalInstruction + BltInstruction + BltalInstruction + BltzInstruction + BltzalInstruction + BnaInstruction + BnaalInstruction + BnanInstruction + BnazInstruction + BnazalInstruction + BneInstruction + BnealInstruction + BnezInstruction + BnezalInstruction + BrapInstruction + BrapzInstruction + BrdnsInstruction + BrdseInstruction + BreqInstruction + BreqzInstruction + BrgeInstruction + BrgezInstruction + BrgtInstruction + BrgtzInstruction + BrleInstruction + BrlezInstruction + BrltInstruction + BrltzInstruction + BrnaInstruction + BrnanInstruction + BrnazInstruction + BrneInstruction + BrnezInstruction + CeilInstruction + ClrInstruction + ClrdInstruction + CosInstruction + DefineInstruction + DivInstruction + ExpInstruction + FloorInstruction + GetInstruction + GetdInstruction + HcfInstruction + JInstruction + JalInstruction + JrInstruction + LInstruction + LabelInstruction + LbInstruction + LbnInstruction + LbnsInstruction + LbsInstruction + LdInstruction + LogInstruction + LrInstruction + LsInstruction + MaxInstruction + MinInstruction + ModInstruction + MoveInstruction + MulInstruction + NorInstruction + NotInstruction + OrInstruction + PeekInstruction + PokeInstruction + PopInstruction + PushInstruction + PutInstruction + PutdInstruction + RandInstruction + RoundInstruction + SInstruction + SapInstruction + SapzInstruction + SbInstruction + SbnInstruction + SbsInstruction + SdInstruction + SdnsInstruction + SdseInstruction + SelectInstruction + SeqInstruction + SeqzInstruction + SgeInstruction + SgezInstruction + SgtInstruction + SgtzInstruction + SinInstruction + SlaInstruction + SleInstruction + SleepInstruction + SlezInstruction + SllInstruction + SltInstruction + SltzInstruction + SnaInstruction + SnanInstruction + SnanzInstruction + SnazInstruction + SneInstruction + SnezInstruction + SqrtInstruction + SraInstruction + SrlInstruction + SsInstruction + SubInstruction + TanInstruction + TruncInstruction + XorInstruction + YieldInstruction {}
impl<T> ICInstructable for T
where
    T: AbsInstruction + AcosInstruction + AddInstruction + AliasInstruction
        + AndInstruction + AsinInstruction + AtanInstruction + Atan2Instruction
        + BapInstruction + BapalInstruction + BapzInstruction + BapzalInstruction
        + BdnsInstruction + BdnsalInstruction + BdseInstruction + BdsealInstruction
        + BeqInstruction + BeqalInstruction + BeqzInstruction + BeqzalInstruction
        + BgeInstruction + BgealInstruction + BgezInstruction + BgezalInstruction
        + BgtInstruction + BgtalInstruction + BgtzInstruction + BgtzalInstruction
        + BleInstruction + BlealInstruction + BlezInstruction + BlezalInstruction
        + BltInstruction + BltalInstruction + BltzInstruction + BltzalInstruction
        + BnaInstruction + BnaalInstruction + BnanInstruction + BnazInstruction
        + BnazalInstruction + BneInstruction + BnealInstruction + BnezInstruction
        + BnezalInstruction + BrapInstruction + BrapzInstruction + BrdnsInstruction
        + BrdseInstruction + BreqInstruction + BreqzInstruction + BrgeInstruction
        + BrgezInstruction + BrgtInstruction + BrgtzInstruction + BrleInstruction
        + BrlezInstruction + BrltInstruction + BrltzInstruction + BrnaInstruction
        + BrnanInstruction + BrnazInstruction + BrneInstruction + BrnezInstruction
        + CeilInstruction + ClrInstruction + ClrdInstruction + CosInstruction
        + DefineInstruction + DivInstruction + ExpInstruction + FloorInstruction
        + GetInstruction + GetdInstruction + HcfInstruction + JInstruction
        + JalInstruction + JrInstruction + LInstruction + LabelInstruction
        + LbInstruction + LbnInstruction + LbnsInstruction + LbsInstruction
        + LdInstruction + LogInstruction + LrInstruction + LsInstruction + MaxInstruction
        + MinInstruction + ModInstruction + MoveInstruction + MulInstruction
        + NorInstruction + NotInstruction + OrInstruction + PeekInstruction
        + PokeInstruction + PopInstruction + PushInstruction + PutInstruction
        + PutdInstruction + RandInstruction + RoundInstruction + SInstruction
        + SapInstruction + SapzInstruction + SbInstruction + SbnInstruction
        + SbsInstruction + SdInstruction + SdnsInstruction + SdseInstruction
        + SelectInstruction + SeqInstruction + SeqzInstruction + SgeInstruction
        + SgezInstruction + SgtInstruction + SgtzInstruction + SinInstruction
        + SlaInstruction + SleInstruction + SleepInstruction + SlezInstruction
        + SllInstruction + SltInstruction + SltzInstruction + SnaInstruction
        + SnanInstruction + SnanzInstruction + SnazInstruction + SneInstruction
        + SnezInstruction + SqrtInstruction + SraInstruction + SrlInstruction
        + SsInstruction + SubInstruction + TanInstruction + TruncInstruction
        + XorInstruction + YieldInstruction,
{}
