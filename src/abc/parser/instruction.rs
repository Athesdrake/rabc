use super::{opargs::*, opcodes::OpCode};

#[derive(Clone, Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub op: Op,
    pub addr: u32,

    pub targets: Vec<u32>,
    pub jumps_here: Vec<u32>,
}

trait U30Trait {
    fn u30size(&self) -> u32;
}
impl U30Trait for u32 {
    fn u30size(&self) -> u32 {
        let bits = (u32::BITS - self.leading_zeros()) as f64;
        (bits / 7.0).ceil().max(1.0) as Self
    }
}

impl Instruction {
    pub fn is_jump(&self) -> bool {
        self.opcode >= OpCode::IfNlt && self.opcode <= OpCode::LookupSwitch
    }

    pub fn size(&self) -> u32 {
        self.op.size() + 1
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Add(),
    AddI(),
    ApplyType(ArgsCountArg),
    AsType(MultinameArg),
    AsTypeLate(),
    BitAnd(),
    BitNot(),
    BitOr(),
    BitXor(),
    Bkpt(),
    BkptLine(LineArg),
    Call(ArgsCountArg),
    CallMethod(CallMethodDispArg),
    CallProperty(CallPropertyArg),
    CallPropLex(CallPropertyArg),
    CallPropVoid(CallPropertyArg),
    CallStatic(CallMethodArg),
    CallSuper(CallMethodArg),
    CallSuperVoid(CallMethodArg),
    CheckFilter(),
    Coerce(CoerceArg),
    CoerceA(),
    CoerceB(),
    CoerceD(),
    CoerceI(),
    CoerceO(),
    CoerceS(),
    CoerceU(),
    Construct(ArgsCountArg),
    ConstructProp(CallPropertyArg),
    ConstructSuper(ArgsCountArg),
    ConvertB(),
    ConvertD(),
    ConvertF(),
    ConvertF4(),
    ConvertI(),
    ConvertO(),
    ConvertS(),
    ConvertU(),
    Debug(DebugArg),
    DebugFile(DebugFileArg),
    DebugLine(LineArg),
    DecLocal(RegisterArg),
    DecLocalI(RegisterArg),
    Decrement(),
    DecrementI(),
    DeleteProperty(PropertyArg),
    Divide(),
    Dup(),
    Dxns(DxnsArg),
    DxnsLate(),
    Equals(),
    EscXAttr(),
    EscXElem(),
    FindDef(PropertyArg),
    FindProperty(PropertyArg),
    FindPropStrict(PropertyArg),
    GetDescendants(GetDescendantsArg),
    GetGlobalScope(),
    GetGlobalSlot(SlotArg),
    GetLex(PropertyArg),
    GetLocal(RegisterArg),
    GetLocal0(),
    GetLocal1(),
    GetLocal2(),
    GetLocal3(),
    GetOuterScope(ScopeArg),
    GetProperty(PropertyArg),
    GetScopeObject(ScopeArg),
    GetSlot(SlotArg),
    GetSuper(MultinameArg),
    GreaterEquals(),
    GreaterThan(),
    HasNext(),
    HasNext2(HasNext2Arg),
    IfEq(TargetArg),
    IfFalse(TargetArg),
    IfGe(TargetArg),
    IfGt(TargetArg),
    IfLe(TargetArg),
    IfLt(TargetArg),
    IfNe(TargetArg),
    IfNge(TargetArg),
    IfNgt(TargetArg),
    IfNle(TargetArg),
    IfNlt(TargetArg),
    IfStrictEq(TargetArg),
    IfStrictNe(TargetArg),
    IfTrue(TargetArg),
    In(),
    IncLocal(RegisterArg),
    IncLocalI(RegisterArg),
    Increment(),
    IncrementI(),
    InitProperty(PropertyArg),
    InstanceOf(),
    IsType(MultinameArg),
    IsTypeLate(),
    Jump(TargetArg),
    Kill(RegisterArg),
    Label(),
    LessEquals(),
    LessThan(),
    Lf32(),
    Lf32x4(),
    Lf64(),
    Li16(),
    Li32(),
    Li8(),
    LookupSwitch(LookupSwitchArg),
    LShift(),
    Modulo(),
    Multiply(),
    MultiplyI(),
    Negate(),
    NegateI(),
    NewActivation(),
    NewArray(ArgsCountArg),
    NewCatch(NewCatchArg),
    NewClass(NewClassArg),
    NewFunction(NewFunctionArg),
    NewObject(NewObjectArg),
    NextName(),
    NextValue(),
    Nop(),
    Not(),
    Pop(),
    PopScope(),
    PushByte(PushByteArg),
    PushDouble(PushDoubleArg),
    PushFalse(),
    PushFloat(),
    PushFloat4(),
    PushInt(PushIntArg),
    PushNamespace(NamespaceArg),
    PushNan(),
    PushNull(),
    PushScope(),
    PushShort(PushShortArg),
    PushString(PushStringArg),
    PushTrue(),
    PushUint(PushUintArg),
    PushUndefined(),
    PushWith(),
    ReturnValue(),
    ReturnVoid(),
    RShift(),
    SetGlobalSlot(SlotArg),
    SetLocal(RegisterArg),
    SetLocal0(),
    SetLocal1(),
    SetLocal2(),
    SetLocal3(),
    SetProperty(PropertyArg),
    SetSlot(SlotArg),
    SetSuper(MultinameArg),
    Sf32(),
    Sf32x4(),
    Sf64(),
    Si16(),
    Si32(),
    Si8(),
    StrictEquals(),
    Subtract(),
    SubtractI(),
    Swap(),
    Sxi1(),
    Sxi16(),
    Sxi8(),
    Throw(),
    TypeOf(),
    UnPlus(),
    UrShift(),
}

impl Op {
    fn size(&self) -> u32 {
        match self {
            Op::Add()
            | Op::AddI()
            | Op::AsTypeLate()
            | Op::BitAnd()
            | Op::BitNot()
            | Op::BitOr()
            | Op::BitXor()
            | Op::Bkpt()
            | Op::CheckFilter()
            | Op::CoerceA()
            | Op::CoerceB()
            | Op::CoerceD()
            | Op::CoerceI()
            | Op::CoerceO()
            | Op::CoerceS()
            | Op::CoerceU()
            | Op::ConvertB()
            | Op::ConvertD()
            | Op::ConvertF()
            | Op::ConvertF4()
            | Op::ConvertI()
            | Op::ConvertO()
            | Op::ConvertS()
            | Op::ConvertU()
            | Op::Decrement()
            | Op::DecrementI()
            | Op::Divide()
            | Op::Dup()
            | Op::DxnsLate()
            | Op::Equals()
            | Op::EscXAttr()
            | Op::EscXElem()
            | Op::GetGlobalScope()
            | Op::GetLocal0()
            | Op::GetLocal1()
            | Op::GetLocal2()
            | Op::GetLocal3()
            | Op::GreaterEquals()
            | Op::GreaterThan()
            | Op::HasNext()
            | Op::In()
            | Op::Increment()
            | Op::IncrementI()
            | Op::InstanceOf()
            | Op::IsTypeLate()
            | Op::Label()
            | Op::LessEquals()
            | Op::LessThan()
            | Op::Lf32()
            | Op::Lf32x4()
            | Op::Lf64()
            | Op::Li16()
            | Op::Li32()
            | Op::Li8()
            | Op::LShift()
            | Op::Modulo()
            | Op::Multiply()
            | Op::MultiplyI()
            | Op::Negate()
            | Op::NegateI()
            | Op::NewActivation()
            | Op::NextName()
            | Op::NextValue()
            | Op::Nop()
            | Op::Not()
            | Op::Pop()
            | Op::PopScope()
            | Op::PushFalse()
            | Op::PushFloat()
            | Op::PushFloat4()
            | Op::PushNan()
            | Op::PushNull()
            | Op::PushScope()
            | Op::PushTrue()
            | Op::PushUndefined()
            | Op::PushWith()
            | Op::ReturnValue()
            | Op::ReturnVoid()
            | Op::RShift()
            | Op::SetLocal0()
            | Op::SetLocal1()
            | Op::SetLocal2()
            | Op::SetLocal3()
            | Op::Sf32()
            | Op::Sf32x4()
            | Op::Sf64()
            | Op::Si16()
            | Op::Si32()
            | Op::Si8()
            | Op::StrictEquals()
            | Op::Subtract()
            | Op::SubtractI()
            | Op::Swap()
            | Op::Sxi1()
            | Op::Sxi16()
            | Op::Sxi8()
            | Op::Throw()
            | Op::TypeOf()
            | Op::UnPlus()
            | Op::UrShift() => 0,
            Op::PushByte(_) => 1,
            Op::IfNlt(_)
            | Op::IfNle(_)
            | Op::IfNgt(_)
            | Op::IfNge(_)
            | Op::Jump(_)
            | Op::IfTrue(_)
            | Op::IfFalse(_)
            | Op::IfEq(_)
            | Op::IfNe(_)
            | Op::IfLt(_)
            | Op::IfLe(_)
            | Op::IfGt(_)
            | Op::IfGe(_)
            | Op::IfStrictEq(_)
            | Op::IfStrictNe(_) => 3,
            Op::GetSuper(op) | Op::AsType(op) | Op::IsType(op) | Op::SetSuper(op) => {
                op.mn.u30size()
            }
            Op::NewArray(op)
            | Op::ConstructSuper(op)
            | Op::Construct(op)
            | Op::ApplyType(op)
            | Op::Call(op) => op.arg_count.u30size(),
            Op::Dxns(op) => op.uri.u30size(),
            Op::BkptLine(op) | Op::DebugLine(op) => op.line.u30size(),
            Op::CallStatic(op) | Op::CallSuper(op) | Op::CallSuperVoid(op) => {
                op.arg_count.u30size()
            }
            Op::Coerce(op) => op.index.u30size(),
            Op::DebugFile(op) => op.filename.u30size(),
            Op::GetLocal(op) | Op::DecLocal(op) | Op::DecLocalI(op) => op.register.u30size(),
            Op::SetProperty(op)
            | Op::InitProperty(op)
            | Op::GetProperty(op)
            | Op::GetLex(op)
            | Op::DeleteProperty(op)
            | Op::FindDef(op)
            | Op::FindProperty(op)
            | Op::FindPropStrict(op) => op.property.u30size(),
            Op::GetDescendants(op) => op.operand.u30size(),
            Op::SetGlobalSlot(op) | Op::SetSlot(op) | Op::GetSlot(op) | Op::GetGlobalSlot(op) => {
                op.slot.u30size()
            }
            Op::GetOuterScope(op) | Op::GetScopeObject(op) => op.scope.u30size(),
            Op::IncLocal(op) | Op::IncLocalI(op) | Op::SetLocal(op) | Op::Kill(op) => {
                op.register.u30size()
            }
            Op::NewCatch(op) => op.exception.u30size(),
            Op::NewClass(op) => op.class.u30size(),
            Op::NewFunction(op) => op.method.u30size(),
            Op::NewObject(op) => op.property_count.u30size(),
            Op::PushDouble(op) => op.value.u30size(),
            Op::PushInt(op) => op.value.u30size(),
            Op::PushNamespace(op) => op.ns.u30size(),
            Op::PushShort(op) => (op.value as u32).u30size(),
            Op::PushString(op) => op.value.u30size(),
            Op::PushUint(op) => op.value.u30size(),
            Op::ConstructProp(op)
            | Op::CallProperty(op)
            | Op::CallPropLex(op)
            | Op::CallPropVoid(op) => op.arg_count.u30size() + op.property.u30size(),
            Op::CallMethod(op) => op.arg_count.u30size() + op.disp_id.u30size(),
            Op::Debug(op) => 2 + op.reg_name.u30size() + op.extra.u30size(),
            Op::HasNext2(op) => op.index_register.u30size() + op.object_register.u30size(),
            Op::LookupSwitch(op) => {
                (op.targets.len() + 1) as u32 * 3 + (op.targets.len() as u32).u30size()
            }
        }
    }
}
