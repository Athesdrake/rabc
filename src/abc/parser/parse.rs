use super::{
    instruction::{Instruction, Op},
    opargs::{self, SerializeTrait},
    OpCode,
};
use crate::{
    abc::Method,
    error::{RabcError, Result},
    StreamReader, StreamWriter,
};
use num_traits::cast::ToPrimitive;
use std::collections::HashMap;

impl Method {
    pub fn parse(&self) -> Result<Vec<Instruction>> {
        let mut stream = StreamReader::new(&self.code);
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut targets: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut addr2idx = HashMap::new();
        while !stream.finished() {
            instructions.push(parse(&mut stream)?);
            let ins = instructions.last().unwrap();
            let i = instructions.len() - 1;
            addr2idx.insert(ins.addr, i);

            if let Op::IfNlt(arg)
            | Op::IfNle(arg)
            | Op::IfNgt(arg)
            | Op::IfNge(arg)
            | Op::Jump(arg)
            | Op::IfTrue(arg)
            | Op::IfFalse(arg)
            | Op::IfEq(arg)
            | Op::IfNe(arg)
            | Op::IfLt(arg)
            | Op::IfLe(arg)
            | Op::IfGt(arg)
            | Op::IfGe(arg)
            | Op::IfStrictEq(arg)
            | Op::IfStrictNe(arg) = &ins.op
            {
                targets.entry(arg.target).or_default().push(ins.addr);
            } else if let Op::LookupSwitch(arg) = &ins.op {
                for target in [arg.default_target].iter().chain(arg.targets.iter()) {
                    targets.entry(*target).or_default().push(ins.addr);
                }
            }
        }

        if !targets.is_empty() {
            for (target, indices) in targets {
                instructions[addr2idx[&target]].jumps_here.extend(&indices);
                for j in indices {
                    instructions[addr2idx[&j]].targets.push(target);
                }
            }
        }

        Ok(instructions)
    }

    pub fn save_instructions(&mut self, instructions: &[Instruction]) -> Result<()> {
        let mut stream = StreamWriter::new(Vec::with_capacity(self.code.len()));
        for ins in instructions {
            serialize(ins, &mut stream)?;
        }

        self.code = stream.move_buffer();
        Ok(())
    }
}

fn parse(stream: &mut StreamReader) -> Result<Instruction> {
    let addr = stream.pos();
    let byte = stream.read_u8()?;
    let opcode = OpCode::from_u8(byte).ok_or(RabcError::InvalidOpCode(byte, addr))?;
    let op = match opcode {
        OpCode::GetSuper => Op::GetSuper(opargs::MultinameArg::parse(stream)?),
        OpCode::SetSuper => Op::SetSuper(opargs::MultinameArg::parse(stream)?),
        OpCode::AsType => Op::AsType(opargs::MultinameArg::parse(stream)?),
        OpCode::IsType => Op::IsType(opargs::MultinameArg::parse(stream)?),
        OpCode::Kill => Op::Kill(opargs::RegisterArg::parse(stream)?),
        OpCode::GetLocal => Op::GetLocal(opargs::RegisterArg::parse(stream)?),
        OpCode::SetLocal => Op::SetLocal(opargs::RegisterArg::parse(stream)?),
        OpCode::IncLocal => Op::IncLocal(opargs::RegisterArg::parse(stream)?),
        OpCode::DecLocal => Op::DecLocal(opargs::RegisterArg::parse(stream)?),
        OpCode::IncLocalI => Op::IncLocalI(opargs::RegisterArg::parse(stream)?),
        OpCode::DecLocalI => Op::DecLocalI(opargs::RegisterArg::parse(stream)?),
        OpCode::IfNlt => Op::IfNlt(opargs::TargetArg::parse(stream)?),
        OpCode::IfNle => Op::IfNle(opargs::TargetArg::parse(stream)?),
        OpCode::IfNgt => Op::IfNgt(opargs::TargetArg::parse(stream)?),
        OpCode::IfNge => Op::IfNge(opargs::TargetArg::parse(stream)?),
        OpCode::Jump => Op::Jump(opargs::TargetArg::parse(stream)?),
        OpCode::IfTrue => Op::IfTrue(opargs::TargetArg::parse(stream)?),
        OpCode::IfFalse => Op::IfFalse(opargs::TargetArg::parse(stream)?),
        OpCode::IfEq => Op::IfEq(opargs::TargetArg::parse(stream)?),
        OpCode::IfNe => Op::IfNe(opargs::TargetArg::parse(stream)?),
        OpCode::IfLt => Op::IfLt(opargs::TargetArg::parse(stream)?),
        OpCode::IfLe => Op::IfLe(opargs::TargetArg::parse(stream)?),
        OpCode::IfGt => Op::IfGt(opargs::TargetArg::parse(stream)?),
        OpCode::IfGe => Op::IfGe(opargs::TargetArg::parse(stream)?),
        OpCode::IfStrictEq => Op::IfStrictEq(opargs::TargetArg::parse(stream)?),
        OpCode::IfStrictNe => Op::IfStrictNe(opargs::TargetArg::parse(stream)?),
        OpCode::LookupSwitch => Op::LookupSwitch(opargs::LookupSwitchArg::parse(stream)?),
        OpCode::Dxns => Op::Dxns(opargs::DxnsArg::parse(stream)?),
        OpCode::PushByte => Op::PushByte(opargs::PushByteArg::parse(stream)?),
        OpCode::PushShort => Op::PushShort(opargs::PushShortArg::parse(stream)?),
        OpCode::PushString => Op::PushString(opargs::PushStringArg::parse(stream)?),
        OpCode::PushInt => Op::PushInt(opargs::PushIntArg::parse(stream)?),
        OpCode::PushUint => Op::PushUint(opargs::PushUintArg::parse(stream)?),
        OpCode::PushDouble => Op::PushDouble(opargs::PushDoubleArg::parse(stream)?),
        OpCode::PushNamespace => Op::PushNamespace(opargs::NamespaceArg::parse(stream)?),
        OpCode::HasNext2 => Op::HasNext2(opargs::HasNext2Arg::parse(stream)?),
        OpCode::NewFunction => Op::NewFunction(opargs::NewFunctionArg::parse(stream)?),
        OpCode::Call => Op::Call(opargs::ArgsCountArg::parse(stream)?),
        OpCode::Construct => Op::Construct(opargs::ArgsCountArg::parse(stream)?),
        OpCode::CallMethod => Op::CallMethod(opargs::CallMethodDispArg::parse(stream)?),
        OpCode::CallStatic => Op::CallStatic(opargs::CallMethodArg::parse(stream)?),
        OpCode::CallSuper => Op::CallSuper(opargs::CallMethodArg::parse(stream)?),
        OpCode::CallProperty => Op::CallProperty(opargs::CallPropertyArg::parse(stream)?),
        OpCode::ConstructSuper => Op::ConstructSuper(opargs::ArgsCountArg::parse(stream)?),
        OpCode::ConstructProp => Op::ConstructProp(opargs::CallPropertyArg::parse(stream)?),
        OpCode::CallPropLex => Op::CallPropLex(opargs::CallPropertyArg::parse(stream)?),
        OpCode::CallSuperVoid => Op::CallSuperVoid(opargs::CallMethodArg::parse(stream)?),
        OpCode::CallPropVoid => Op::CallPropVoid(opargs::CallPropertyArg::parse(stream)?),
        OpCode::ApplyType => Op::ApplyType(opargs::ArgsCountArg::parse(stream)?),
        OpCode::NewObject => Op::NewObject(opargs::NewObjectArg::parse(stream)?),
        OpCode::NewArray => Op::NewArray(opargs::ArgsCountArg::parse(stream)?),
        OpCode::NewClass => Op::NewClass(opargs::NewClassArg::parse(stream)?),
        OpCode::GetDescendants => Op::GetDescendants(opargs::GetDescendantsArg::parse(stream)?),
        OpCode::NewCatch => Op::NewCatch(opargs::NewCatchArg::parse(stream)?),
        OpCode::FindPropstrict => Op::FindPropStrict(opargs::PropertyArg::parse(stream)?),
        OpCode::FindProperty => Op::FindProperty(opargs::PropertyArg::parse(stream)?),
        OpCode::FindDef => Op::FindDef(opargs::PropertyArg::parse(stream)?),
        OpCode::GetLex => Op::GetLex(opargs::PropertyArg::parse(stream)?),
        OpCode::SetProperty => Op::SetProperty(opargs::PropertyArg::parse(stream)?),
        OpCode::GetScopeObject => Op::GetScopeObject(opargs::ScopeArg::parse(stream)?),
        OpCode::GetProperty => Op::GetProperty(opargs::PropertyArg::parse(stream)?),
        OpCode::GetOuterScope => Op::GetOuterScope(opargs::ScopeArg::parse(stream)?),
        OpCode::InitProperty => Op::InitProperty(opargs::PropertyArg::parse(stream)?),
        OpCode::DeleteProperty => Op::DeleteProperty(opargs::PropertyArg::parse(stream)?),
        OpCode::GetSlot => Op::GetSlot(opargs::SlotArg::parse(stream)?),
        OpCode::SetSlot => Op::SetSlot(opargs::SlotArg::parse(stream)?),
        OpCode::GetGlobalSlot => Op::GetGlobalSlot(opargs::SlotArg::parse(stream)?),
        OpCode::SetGlobalSlot => Op::SetGlobalSlot(opargs::SlotArg::parse(stream)?),
        OpCode::Coerce => Op::Coerce(opargs::CoerceArg::parse(stream)?),
        OpCode::Debug => Op::Debug(opargs::DebugArg::parse(stream)?),
        OpCode::BkptLine => Op::BkptLine(opargs::LineArg::parse(stream)?),
        OpCode::DebugLine => Op::DebugLine(opargs::LineArg::parse(stream)?),
        OpCode::DebugFile => Op::DebugFile(opargs::DebugFileArg::parse(stream)?),
        OpCode::Bkpt => Op::Bkpt(),
        OpCode::Nop => Op::Nop(),
        OpCode::Throw => Op::Throw(),
        OpCode::DxnsLate => Op::DxnsLate(),
        OpCode::Label => Op::Label(),
        OpCode::Lf32x4 => Op::Lf32x4(),
        OpCode::Sf32x4 => Op::Sf32x4(),
        OpCode::PushWith => Op::PushWith(),
        OpCode::PopScope => Op::PopScope(),
        OpCode::NextName => Op::NextName(),
        OpCode::HasNext => Op::HasNext(),
        OpCode::PushNull => Op::PushNull(),
        OpCode::PushUndefined => Op::PushUndefined(),
        OpCode::PushFloat => Op::PushFloat(),
        OpCode::NextValue => Op::NextValue(),
        OpCode::PushTrue => Op::PushTrue(),
        OpCode::PushFalse => Op::PushFalse(),
        OpCode::PushNan => Op::PushNan(),
        OpCode::Pop => Op::Pop(),
        OpCode::Dup => Op::Dup(),
        OpCode::Swap => Op::Swap(),
        OpCode::PushScope => Op::PushScope(),
        OpCode::Li8 => Op::Li8(),
        OpCode::Li16 => Op::Li16(),
        OpCode::Li32 => Op::Li32(),
        OpCode::Lf32 => Op::Lf32(),
        OpCode::Lf64 => Op::Lf64(),
        OpCode::Si8 => Op::Si8(),
        OpCode::Si16 => Op::Si16(),
        OpCode::Si32 => Op::Si32(),
        OpCode::Sf32 => Op::Sf32(),
        OpCode::Sf64 => Op::Sf64(),
        OpCode::ReturnVoid => Op::ReturnVoid(),
        OpCode::ReturnValue => Op::ReturnValue(),
        OpCode::Sxi1 => Op::Sxi1(),
        OpCode::Sxi8 => Op::Sxi8(),
        OpCode::Sxi16 => Op::Sxi16(),
        OpCode::PushFloat4 => Op::PushFloat4(),
        OpCode::NewActivation => Op::NewActivation(),
        OpCode::GetGlobalScope => Op::GetGlobalScope(),
        OpCode::ConvertS => Op::ConvertS(),
        OpCode::EscXElem => Op::EscXElem(),
        OpCode::EscXAttr => Op::EscXAttr(),
        OpCode::ConvertI => Op::ConvertI(),
        OpCode::ConvertU => Op::ConvertU(),
        OpCode::ConvertD => Op::ConvertD(),
        OpCode::ConvertB => Op::ConvertB(),
        OpCode::ConvertO => Op::ConvertO(),
        OpCode::CheckFilter => Op::CheckFilter(),
        OpCode::ConvertF => Op::ConvertF(),
        OpCode::UnPlus => Op::UnPlus(),
        OpCode::ConvertF4 => Op::ConvertF4(),
        OpCode::CoerceB => Op::CoerceB(),
        OpCode::CoerceA => Op::CoerceA(),
        OpCode::CoerceI => Op::CoerceI(),
        OpCode::CoerceD => Op::CoerceD(),
        OpCode::CoerceS => Op::CoerceS(),
        OpCode::AsTypeLate => Op::AsTypeLate(),
        OpCode::CoerceU => Op::CoerceU(),
        OpCode::CoerceO => Op::CoerceO(),
        OpCode::Negate => Op::Negate(),
        OpCode::Increment => Op::Increment(),
        OpCode::Decrement => Op::Decrement(),
        OpCode::TypeOf => Op::TypeOf(),
        OpCode::Not => Op::Not(),
        OpCode::BitNot => Op::BitNot(),
        OpCode::Add => Op::Add(),
        OpCode::Subtract => Op::Subtract(),
        OpCode::Multiply => Op::Multiply(),
        OpCode::Divide => Op::Divide(),
        OpCode::Modulo => Op::Modulo(),
        OpCode::LShift => Op::LShift(),
        OpCode::RShift => Op::RShift(),
        OpCode::UrShift => Op::UrShift(),
        OpCode::BitAnd => Op::BitAnd(),
        OpCode::BitOr => Op::BitOr(),
        OpCode::BitXor => Op::BitXor(),
        OpCode::Equals => Op::Equals(),
        OpCode::StrictEquals => Op::StrictEquals(),
        OpCode::LessThan => Op::LessThan(),
        OpCode::LessEquals => Op::LessEquals(),
        OpCode::GreaterThan => Op::GreaterThan(),
        OpCode::GreaterEquals => Op::GreaterEquals(),
        OpCode::InstanceOf => Op::InstanceOf(),
        OpCode::IsTypeLate => Op::IsTypeLate(),
        OpCode::In => Op::In(),
        OpCode::IncrementI => Op::IncrementI(),
        OpCode::DecrementI => Op::DecrementI(),
        OpCode::NegateI => Op::NegateI(),
        OpCode::AddI => Op::AddI(),
        OpCode::SubtractI => Op::SubtractI(),
        OpCode::MultiplyI => Op::MultiplyI(),
        OpCode::GetLocal0 => Op::GetLocal0(),
        OpCode::GetLocal1 => Op::GetLocal1(),
        OpCode::GetLocal2 => Op::GetLocal2(),
        OpCode::GetLocal3 => Op::GetLocal3(),
        OpCode::SetLocal0 => Op::SetLocal0(),
        OpCode::SetLocal1 => Op::SetLocal1(),
        OpCode::SetLocal2 => Op::SetLocal2(),
        OpCode::SetLocal3 => Op::SetLocal3(),
    };
    Ok(Instruction {
        opcode,
        op,
        addr,
        targets: Vec::new(),
        jumps_here: Vec::new(),
    })
}

fn serialize(ins: &Instruction, stream: &mut StreamWriter) -> Result<()> {
    stream.write_u8(ins.opcode.to_u8().unwrap())?;
    match &ins.op {
        Op::ApplyType(args)
        | Op::Call(args)
        | Op::Construct(args)
        | Op::ConstructSuper(args)
        | Op::NewArray(args) => args.serialize(stream),
        Op::AsType(args) | Op::GetSuper(args) | Op::IsType(args) | Op::SetSuper(args) => {
            args.serialize(stream)
        }
        Op::BkptLine(args) | Op::DebugLine(args) => args.serialize(stream),
        Op::CallMethod(args) => args.serialize(stream),
        Op::CallProperty(args)
        | Op::CallPropLex(args)
        | Op::CallPropVoid(args)
        | Op::ConstructProp(args) => args.serialize(stream),
        Op::CallStatic(args) | Op::CallSuper(args) | Op::CallSuperVoid(args) => {
            args.serialize(stream)
        }
        Op::Coerce(args) => args.serialize(stream),
        Op::Debug(args) => args.serialize(stream),
        Op::DebugFile(args) => args.serialize(stream),
        Op::DecLocal(args)
        | Op::DecLocalI(args)
        | Op::GetLocal(args)
        | Op::IncLocal(args)
        | Op::IncLocalI(args)
        | Op::Kill(args)
        | Op::SetLocal(args) => args.serialize(stream),
        Op::DeleteProperty(args)
        | Op::FindDef(args)
        | Op::FindProperty(args)
        | Op::FindPropStrict(args)
        | Op::GetLex(args)
        | Op::GetProperty(args)
        | Op::InitProperty(args)
        | Op::SetProperty(args) => args.serialize(stream),
        Op::Dxns(args) => args.serialize(stream),
        Op::GetDescendants(args) => args.serialize(stream),
        Op::GetGlobalSlot(args)
        | Op::GetSlot(args)
        | Op::SetGlobalSlot(args)
        | Op::SetSlot(args) => args.serialize(stream),
        Op::GetOuterScope(args) | Op::GetScopeObject(args) => args.serialize(stream),
        Op::HasNext2(args) => args.serialize(stream),
        Op::IfEq(args)
        | Op::IfFalse(args)
        | Op::IfGe(args)
        | Op::IfGt(args)
        | Op::IfLe(args)
        | Op::IfLt(args)
        | Op::IfNe(args)
        | Op::IfNge(args)
        | Op::IfNgt(args)
        | Op::IfNle(args)
        | Op::IfNlt(args)
        | Op::IfStrictEq(args)
        | Op::IfStrictNe(args)
        | Op::IfTrue(args)
        | Op::Jump(args) => args.serialize(stream),
        Op::LookupSwitch(args) => args.serialize(stream),
        Op::NewCatch(args) => args.serialize(stream),
        Op::NewClass(args) => args.serialize(stream),
        Op::NewFunction(args) => args.serialize(stream),
        Op::NewObject(args) => args.serialize(stream),
        Op::PushByte(args) => args.serialize(stream),
        Op::PushDouble(args) => args.serialize(stream),
        Op::PushInt(args) => args.serialize(stream),
        Op::PushNamespace(args) => args.serialize(stream),
        Op::PushShort(args) => args.serialize(stream),
        Op::PushString(args) => args.serialize(stream),
        Op::PushUint(args) => args.serialize(stream),
        _ => Ok(()),
    }
}
