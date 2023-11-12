use super::{
    instruction::{Instruction, Op},
    opargs, OpCode,
};
use crate::{
    abc::Method,
    error::{Error, Result},
    StreamReader,
};
use std::collections::HashMap;

impl Method {
    pub fn parse(&self) -> Result<Vec<Instruction>> {
        // Here, we make a copy, because I didn't register a lifetime for the StreamReader's buffer
        // TODO: don't copy
        let buf = self.code.to_vec();
        let mut stream = StreamReader::new(buf);
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut targets: HashMap<u32, Vec<usize>> = HashMap::new();
        while !stream.finished() {
            instructions.push(parse(&mut stream)?);
            let ins = instructions.last().unwrap();
            let i = instructions.len() - 1;

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
                targets.entry(arg.target).or_default().push(i);
            }
        }

        if !targets.is_empty() {
            for ins in instructions.iter_mut() {
                if let Some(ts) = targets.get(&ins.addr) {
                    ins.targets.extend(ts);
                }
            }
        }

        Ok(instructions)
    }
}

fn parse(stream: &mut StreamReader) -> Result<Instruction> {
    let addr = stream.pos();
    let byte = stream.read_u8()?;
    let opcode = OpCode::from_u8(byte).ok_or(Error::InvalidOpCode(byte))?;
    let op = match opcode {
        OpCode::GetSuper => Op::GetSuper(opargs::MultinameArg {
            mn: stream.read_u30()?,
        }),
        OpCode::SetSuper => Op::SetSuper(opargs::MultinameArg {
            mn: stream.read_u30()?,
        }),
        OpCode::AsType => Op::AsType(opargs::MultinameArg {
            mn: stream.read_u30()?,
        }),
        OpCode::IsType => Op::IsType(opargs::MultinameArg {
            mn: stream.read_u30()?,
        }),
        OpCode::Kill => Op::Kill(opargs::RegisterArg {
            register: stream.read_u30()?,
        }),
        OpCode::GetLocal => Op::GetLocal(opargs::RegisterArg {
            register: stream.read_u30()?,
        }),
        OpCode::SetLocal => Op::SetLocal(opargs::RegisterArg {
            register: stream.read_u30()?,
        }),
        OpCode::IncLocal => Op::IncLocal(opargs::RegisterArg {
            register: stream.read_u30()?,
        }),
        OpCode::DecLocal => Op::DecLocal(opargs::RegisterArg {
            register: stream.read_u30()?,
        }),
        OpCode::IncLocalI => Op::IncLocalI(opargs::RegisterArg {
            register: stream.read_u30()?,
        }),
        OpCode::DecLocalI => Op::DecLocalI(opargs::RegisterArg {
            register: stream.read_u30()?,
        }),
        OpCode::IfNlt
        | OpCode::IfNle
        | OpCode::IfNgt
        | OpCode::IfNge
        | OpCode::Jump
        | OpCode::IfTrue
        | OpCode::IfFalse
        | OpCode::IfEq
        | OpCode::IfNe
        | OpCode::IfLt
        | OpCode::IfLe
        | OpCode::IfGt
        | OpCode::IfGe
        | OpCode::IfStrictEq
        | OpCode::IfStrictNe => {
            let target = (stream.read_i24()? + stream.pos() as i32) as u32;
            let arg = opargs::TargetArg { target };
            match opcode {
                OpCode::IfNlt => Op::IfNlt(arg),
                OpCode::IfNle => Op::IfNle(arg),
                OpCode::IfNgt => Op::IfNgt(arg),
                OpCode::IfNge => Op::IfNge(arg),
                OpCode::Jump => Op::Jump(arg),
                OpCode::IfTrue => Op::IfTrue(arg),
                OpCode::IfFalse => Op::IfFalse(arg),
                OpCode::IfEq => Op::IfEq(arg),
                OpCode::IfNe => Op::IfNe(arg),
                OpCode::IfLt => Op::IfLt(arg),
                OpCode::IfLe => Op::IfLe(arg),
                OpCode::IfGt => Op::IfGt(arg),
                OpCode::IfGe => Op::IfGe(arg),
                OpCode::IfStrictEq => Op::IfStrictEq(arg),
                OpCode::IfStrictNe => Op::IfStrictNe(arg),
                _ => unreachable!(),
            }
        }
        OpCode::LookupSwitch => {
            // The base address for the lookupswitch's targets is always the instruction's address unlike all other jump
            // instructions.
            let default_target = (addr as i32 + stream.read_i24()?) as u32;
            let cases = stream.read_u30()?;
            let mut targets = Vec::with_capacity(cases as usize);
            for _ in 0..cases {
                targets.push((addr as i32 + stream.read_i24()?) as u32);
            }
            Op::LookupSwitch(opargs::LookupSwitchArg {
                default_target,
                targets: targets.into(),
            })
        }
        OpCode::Dxns => Op::Dxns(opargs::DxnsArg {
            uri: stream.read_u30()?,
        }),
        OpCode::PushByte => Op::PushByte(opargs::PushByteArg {
            value: stream.read_u8()?,
        }),
        OpCode::PushShort => Op::PushShort(opargs::PushShortArg {
            value: stream.read_u30()? as i16,
        }),
        OpCode::PushString => Op::PushString(opargs::PushStringArg {
            value: stream.read_u30()?,
        }),
        OpCode::PushInt => Op::PushInt(opargs::PushIntArg {
            value: stream.read_u30()?,
        }),
        OpCode::PushUint => Op::PushUint(opargs::PushUintArg {
            value: stream.read_u30()?,
        }),
        OpCode::PushDouble => Op::PushDouble(opargs::PushDoubleArg {
            value: stream.read_u30()?,
        }),
        OpCode::PushNamespace => Op::PushNamespace(opargs::NamespaceArg {
            ns: stream.read_u30()?,
        }),
        OpCode::HasNext2 => Op::HasNext2(opargs::HasNext2Arg {
            object_register: stream.read_u30()?,
            index_register: stream.read_u30()?,
        }),
        OpCode::NewFunction => Op::NewFunction(opargs::NewFunctionArg {
            method: stream.read_u30()?,
        }),
        OpCode::Call => Op::Call(opargs::ArgsCountArg {
            arg_count: stream.read_u30()?,
        }),
        OpCode::Construct => Op::Construct(opargs::ArgsCountArg {
            arg_count: stream.read_u30()?,
        }),
        OpCode::CallMethod => Op::CallMethod(opargs::CallMethodDispArg {
            disp_id: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::CallStatic => Op::CallStatic(opargs::CallMethodArg {
            method: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::CallSuper => Op::CallSuper(opargs::CallMethodArg {
            method: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::CallProperty => Op::CallProperty(opargs::CallPropertyArg {
            property: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::ConstructSuper => Op::ConstructSuper(opargs::ArgsCountArg {
            arg_count: stream.read_u30()?,
        }),
        OpCode::ConstructProp => Op::ConstructProp(opargs::CallPropertyArg {
            property: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::CallPropLex => Op::CallPropLex(opargs::CallPropertyArg {
            property: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::CallSuperVoid => Op::CallSuperVoid(opargs::CallMethodArg {
            method: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::CallPropVoid => Op::CallPropVoid(opargs::CallPropertyArg {
            property: stream.read_u30()?,
            arg_count: stream.read_u30()?,
        }),
        OpCode::ApplyType => Op::ApplyType(opargs::ArgsCountArg {
            arg_count: stream.read_u30()?,
        }),
        OpCode::NewObject => Op::NewObject(opargs::NewObjectArg {
            property_count: stream.read_u30()?,
        }),
        OpCode::NewArray => Op::NewArray(opargs::ArgsCountArg {
            arg_count: stream.read_u30()?,
        }),
        OpCode::NewClass => Op::NewClass(opargs::NewClassArg {
            class: stream.read_u30()?,
        }),
        OpCode::GetDescendants => Op::GetDescendants(opargs::GetDescendantsArg {
            operand: stream.read_u30()?,
        }),
        OpCode::NewCatch => Op::NewCatch(opargs::NewCatchArg {
            exception: stream.read_u30()?,
        }),
        OpCode::FindPropstrict => Op::FindPropStrict(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::FindProperty => Op::FindProperty(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::FindDef => Op::FindDef(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::GetLex => Op::GetLex(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::SetProperty => Op::SetProperty(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::GetScopeObject => Op::GetScopeObject(opargs::ScopeArg {
            scope: stream.read_u30()?,
        }),
        OpCode::GetProperty => Op::GetProperty(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::GetOuterScope => Op::GetOuterScope(opargs::ScopeArg {
            scope: stream.read_u30()?,
        }),
        OpCode::InitProperty => Op::InitProperty(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::DeleteProperty => Op::DeleteProperty(opargs::PropertyArg {
            property: stream.read_u30()?,
        }),
        OpCode::GetSlot => Op::GetSlot(opargs::SlotArg {
            slot: stream.read_u30()?,
        }),
        OpCode::SetSlot => Op::SetSlot(opargs::SlotArg {
            slot: stream.read_u30()?,
        }),
        OpCode::GetGlobalSlot => Op::GetGlobalSlot(opargs::SlotArg {
            slot: stream.read_u30()?,
        }),
        OpCode::SetGlobalSlot => Op::SetGlobalSlot(opargs::SlotArg {
            slot: stream.read_u30()?,
        }),
        OpCode::Coerce => Op::Coerce(opargs::CoerceArg {
            index: stream.read_u30()?,
        }),
        OpCode::Debug => Op::Debug(opargs::DebugArg {
            debug_type: stream.read_u8()?,
            reg_name: stream.read_u30()?,
            register: stream.read_u8()?,
            extra: stream.read_u30()?,
        }),
        OpCode::DebugLine => Op::DebugLine(opargs::LineArg {
            line: stream.read_u30()?,
        }),
        OpCode::DebugFile => Op::DebugFile(opargs::DebugFileArg {
            filename: stream.read_u30()?,
        }),
        OpCode::BkptLine => Op::BkptLine(opargs::LineArg {
            line: stream.read_u30()?,
        }),
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
