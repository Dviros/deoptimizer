extern crate keystone;
// use iced_x86::*;
use iced_x86::{
    Code, ConditionCode, Decoder, DecoderOptions, Formatter, GasFormatter, Instruction,
    InstructionInfoFactory, IntelFormatter, MasmFormatter, NasmFormatter, OpKind, Register,
    RflagsBits,
};
use keystone::{AsmResult, Keystone};
use log::{error, info, warn, LevelFilter};
use rand::seq::SliceRandom;
use thiserror::Error;

use crate::x86::{
    apply_ap_transform, apply_itr_transform, apply_li_transform, apply_om_transform,
    apply_otr_transform, apply_rs_transform,
};

#[derive(Error, Debug)]
pub enum DeoptimizerError {
    #[error("Invalid formatter syntax.")]
    InvalidSyntax,
    #[error("Instruction with unexpected operand count.")]
    UnexpectedOperandCount,
    #[error("All available instruction transform gadgets failed.")]
    AllTransformsFailed,
}

enum AssemblySyntax {
    Keystone,
    Nasm,
    Masm,
    Intel,
    Gas,
}

pub struct Deoptimizer {
    /// Immidiate partitioning length.
    pub ipl: u32,
    /// Deoptimization frequency.
    pub freq: f64,
    /// Is the given code analyzed.
    pub is_analyzed: bool,
    disassembly: Option<String>,
    syntax: AssemblySyntax,
    known_addr_table: Option<Vec<u64>>,
    known_branch_targets: Option<Vec<u64>>,
    cfe_addr_table: Option<Vec<u64>>,
}

impl Deoptimizer {
    pub fn new() -> Self {
        Self {
            ipl: 1,
            freq: 0.5,
            is_analyzed: false,
            disassembly: None,
            syntax: AssemblySyntax::Nasm,
            known_addr_table: None,
            known_branch_targets: None,
            cfe_addr_table: None,
        }
    }

    pub fn set_syntax(&mut self, syntax: String) -> Result<(), DeoptimizerError> {
        match syntax.to_lowercase().as_str() {
            "keystone" => self.syntax = AssemblySyntax::Keystone,
            "nasm" => self.syntax = AssemblySyntax::Nasm,
            "masm" => self.syntax = AssemblySyntax::Masm,
            "intel" => self.syntax = AssemblySyntax::Intel,
            "gas" => self.syntax = AssemblySyntax::Gas,
            _ => return Err(DeoptimizerError::InvalidSyntax),
        }
        Ok(())
    }

    fn analyze(&mut self, code: &[u8], mode: u32, start_addr: u64) {
        let mut decoder = Decoder::with_ip(mode, code, start_addr, DecoderOptions::NONE);
        let mut instruction = Instruction::default();
        let mut known_addr_table = Vec::new();
        let mut known_branch_targets = Vec::new();
        let mut cfe_addr_table = Vec::new();
        while decoder.can_decode() {
            decoder.decode_out(&mut instruction);
            // Push to known address table
            known_addr_table.push(instruction.ip());
            let nbt = instruction.near_branch_target();
            if nbt != 0 {
                known_branch_targets.push(nbt);
            }
            // Push to control flow exit address table if it is a JMP of RET
            if instruction.mnemonic() == iced_x86::Mnemonic::Ret
                || instruction.mnemonic() == iced_x86::Mnemonic::Retf
                || instruction.mnemonic() == iced_x86::Mnemonic::Jmp
            {
                cfe_addr_table.push(instruction.ip())
            }
        }
        self.known_addr_table = Some(known_addr_table);
        self.known_branch_targets = Some(known_branch_targets);
        self.cfe_addr_table = Some(cfe_addr_table);
        self.is_analyzed = true;
    }

    fn is_known_address(&mut self, addr: u64) -> bool {
        if let Some(table) = &self.known_addr_table {
            return table.contains(&addr);
        }
        false
    }

    fn is_branch_target(&mut self, addr: u64) -> bool {
        if let Some(table) = &self.known_branch_targets {
            return table.contains(&addr);
        }
        false
    }

    fn get_random_cfe_addr(&self) -> Option<u64> {
        if let Some(table) = &self.cfe_addr_table {
            return table.choose(&mut rand::thread_rng()).copied();
        }
        None
    }

    pub fn format_instruction(&self, inst: &Instruction) -> String {
        let mut result = String::new();
        match self.syntax {
            AssemblySyntax::Keystone => {
                let mut formatter = IntelFormatter::new();
                formatter.options_mut().set_uppercase_keywords(false);
                formatter
                    .options_mut()
                    .set_memory_size_options(iced_x86::MemorySizeOptions::Always);
                formatter.options_mut().set_hex_prefix("0x");
                formatter.options_mut().set_hex_suffix("");
                formatter.format(inst, &mut result);
            }
            AssemblySyntax::Nasm => {
                let mut formatter = NasmFormatter::new();
                formatter.format(inst, &mut result);
            }
            AssemblySyntax::Masm => {
                let mut formatter = MasmFormatter::new();
                formatter.format(inst, &mut result);
            }
            AssemblySyntax::Intel => {
                let mut formatter = IntelFormatter::new();
                formatter.format(inst, &mut result);
            }
            AssemblySyntax::Gas => {
                let mut formatter = GasFormatter::new();
                formatter.format(inst, &mut result);
            }
        };
        result
    }

    pub fn disassemble(
        &mut self,
        code: &[u8],
        mode: u32,
        start_addr: u64,
    ) -> Result<String, DeoptimizerError> {
        let mut decoder = Decoder::with_ip(mode, code, start_addr, DecoderOptions::NONE);
        let mut result = String::new();
        // let mut info_factory = InstructionInfoFactory::new();
        let mut instruction = Instruction::default();
        // Analyze the given binary and genereate nessesary tables...
        self.analyze(code, mode, start_addr);
        while decoder.can_decode() {
            decoder.decode_out(&mut instruction);
            // let offsets = decoder.get_constant_offsets(&instruction);
            if instruction.is_invalid() {
                warn!("Found invalid instruction at {}", instruction.ip());
                let start_index = (instruction.ip() - start_addr) as usize;
                let instr_bytes = &code[start_index..start_index + instruction.len()];
                result += &format!(
                    "loc_{:016X}: {}\n",
                    instruction.ip(),
                    to_db_mnemonic(instr_bytes)
                );
                continue;
            }

            let temp = self.format_instruction(&instruction);
            let nbt = instruction.near_branch_target();
            if nbt != 0 {
                if self.is_known_address(nbt) {
                    result += &format!(
                        "loc_{:016X}: {} {}\n",
                        instruction.ip(),
                        temp.split(' ').next().unwrap(),
                        &format!("loc_{:016X}", nbt)
                    );
                    continue;
                } else {
                    warn!("Misaligned instruction detected at {}", instruction.ip())
                }
            }
            result += &format!("loc_{:016X}: {}\n", instruction.ip(), temp);
        }
        self.disassembly = Some(result.clone());
        Ok(result)
    }

    pub fn apply_transform(
        inst: &Instruction,
        mode: u32,
    ) -> Result<Vec<Instruction>, DeoptimizerError> {
        match inst.op_count() {
            0 => todo!("Perform call proxy..."),
            1 => todo!("Handle blacklisted operands. (only 3 exists)"),
            2..=5 => {
                // Priority is important, start with immidate obfuscation
                if let Ok(t) = apply_ap_transform(&mut inst.clone(), mode) {
                    return Ok(t);
                }
                if let Ok(t) = apply_li_transform(&mut inst.clone(), mode) {
                    return Ok(t);
                }
                if let Ok(t) = apply_itr_transform(&mut inst.clone(), mode) {
                    return Ok(t);
                }
                // second target memory obfuscation
                if let Ok(t) = apply_om_transform(&mut inst.clone(), mode) {
                    return Ok(t);
                }
                if let Ok(t) = apply_otr_transform(&mut inst.clone(), mode) {
                    return Ok(t);
                }
                // Now swap registers
                if let Ok(t) = apply_rs_transform(&mut inst.clone(), mode) {
                    return Ok(t);
                }
            }
            _ => return Err(DeoptimizerError::UnexpectedOperandCount), // WTF? this shouldn't happen.
        }
        Err(DeoptimizerError::AllTransformsFailed)
    }

    pub fn assemble(
        &self,
        code: String,
        mode: u32,
        addr: u64,
    ) -> Result<AsmResult, keystone::Error> {
        let m = match mode {
            16 => keystone::MODE_16,
            32 => keystone::MODE_32,
            64 => keystone::MODE_64,
            _ => return Err(keystone::ERR_MODE),
        };
        let engine = Keystone::new(keystone::Arch::X86, m)?;

        match self.syntax {
            AssemblySyntax::Nasm | AssemblySyntax::Keystone => {
                engine.option(keystone::OptionType::SYNTAX, keystone::OPT_SYNTAX_INTEL)?
            }
            AssemblySyntax::Masm => {
                engine.option(keystone::OptionType::SYNTAX, keystone::OPT_SYNTAX_MASM)?
            }
            AssemblySyntax::Intel => {
                engine.option(keystone::OptionType::SYNTAX, keystone::OPT_SYNTAX_INTEL)?
            }
            AssemblySyntax::Gas => {
                engine.option(keystone::OptionType::SYNTAX, keystone::OPT_SYNTAX_GAS)?
            }
        };
        engine.asm(code, addr)
    }
}

pub fn to_db_mnemonic(bytes: &[u8]) -> String {
    let mut db_inst = String::from("db ");
    for b in bytes.iter() {
        db_inst += &format!("0x{:02X}, ", b);
    }
    db_inst.trim_end_matches(", ").to_string()
}