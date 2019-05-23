pub use crate::addressing_mode;
use crate::instruction::*;

pub trait Trait {
    type AddressingMode: addressing_mode::Trait;
    fn opcode() -> u8;
}

pub use adc::Inst as Adc;
pub use alr::Inst as Alr;
pub use anc::Inst as Anc;
pub use and::Inst as And;
pub use arr::Inst as Arr;
pub use asl::Inst as Asl;
pub use bcc::Inst as Bcc;
pub use bcs::Inst as Bcs;
pub use beq::Inst as Beq;
pub use bit::Inst as Bit;
pub use bmi::Inst as Bmi;
pub use bne::Inst as Bne;
pub use bpl::Inst as Bpl;
pub use brk::Inst as Brk;
pub use bvc::Inst as Bvc;
pub use bvs::Inst as Bvs;
pub use clc::Inst as Clc;
pub use cld::Inst as Cld;
pub use cli::Inst as Cli;
pub use clv::Inst as Clv;
pub use cmp::Inst as Cmp;
pub use cpx::Inst as Cpx;
pub use cpy::Inst as Cpy;
pub use dcp::Inst as Dcp;
pub use dec::Inst as Dec;
pub use dex::Inst as Dex;
pub use dey::Inst as Dey;
pub use eor::Inst as Eor;
pub use inc::Inst as Inc;
pub use inx::Inst as Inx;
pub use iny::Inst as Iny;
pub use isc::Inst as Isc;
pub use jmp::Inst as Jmp;
pub use jsr::Inst as Jsr;
pub use lda::Inst as Lda;
pub use ldx::Inst as Ldx;
pub use ldy::Inst as Ldy;
pub use lsr::Inst as Lsr;
pub use nop::Inst as Nop;
pub use ora::Inst as Ora;
pub use pha::Inst as Pha;
pub use php::Inst as Php;
pub use pla::Inst as Pla;
pub use plp::Inst as Plp;
pub use rla::Inst as Rla;
pub use rol::Inst as Rol;
pub use ror::Inst as Ror;
pub use rra::Inst as Rra;
pub use rti::Inst as Rti;
pub use rts::Inst as Rts;
pub use sax::Inst as Sax;
pub use sbc::Inst as Sbc;
pub use sec::Inst as Sec;
pub use sed::Inst as Sed;
pub use sei::Inst as Sei;
pub use skb::Inst as Skb;
pub use slo::Inst as Slo;
pub use sre::Inst as Sre;
pub use sta::Inst as Sta;
pub use stx::Inst as Stx;
pub use sty::Inst as Sty;
pub use tax::Inst as Tax;
pub use tay::Inst as Tay;
pub use tsx::Inst as Tsx;
pub use txa::Inst as Txa;
pub use txs::Inst as Txs;
pub use tya::Inst as Tya;
