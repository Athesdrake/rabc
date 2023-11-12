#[derive(Clone, Debug)]
pub struct ArgsCountArg {
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct MultinameArg {
    pub mn: u32,
}
#[derive(Clone, Debug)]
pub struct CallPropertyArg {
    pub property: u32,
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct CallMethodArg {
    pub method: u32,
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct CallMethodDispArg {
    pub disp_id: u32,
    pub arg_count: u32,
}
#[derive(Clone, Debug)]
pub struct PropertyArg {
    pub property: u32,
}
#[derive(Clone, Debug)]
pub struct LineArg {
    pub line: u32,
}
#[derive(Clone, Debug)]
pub struct RegisterArg {
    pub register: u32,
}
#[derive(Clone, Debug)]
pub struct SlotArg {
    pub slot: u32,
}
#[derive(Clone, Debug)]
pub struct ScopeArg {
    pub scope: u32,
}
#[derive(Clone, Debug)]
pub struct TargetArg {
    pub target: u32,
}
#[derive(Clone, Debug)]
pub struct CoerceArg {
    pub index: u32,
}
#[derive(Clone, Debug)]
pub struct DebugArg {
    pub debug_type: u8,
    pub reg_name: u32,
    pub register: u8,
    pub extra: u32,
}
#[derive(Clone, Debug)]
pub struct DebugFileArg {
    pub filename: u32,
}
#[derive(Clone, Debug)]
pub struct DxnsArg {
    pub uri: u32,
}
#[derive(Clone, Debug)]
pub struct GetDescendantsArg {
    pub operand: u32,
}
#[derive(Clone, Debug)]
pub struct HasNext2Arg {
    pub object_register: u32,
    pub index_register: u32,
}
#[derive(Clone, Debug)]
pub struct LookupSwitchArg {
    pub default_target: u32,
    pub targets: Box<[u32]>,
}
#[derive(Clone, Debug)]
pub struct NewCatchArg {
    pub exception: u32,
}
#[derive(Clone, Debug)]
pub struct NewClassArg {
    pub class: u32,
}
#[derive(Clone, Debug)]
pub struct NewFunctionArg {
    pub method: u32,
}
#[derive(Clone, Debug)]
pub struct NewObjectArg {
    pub property_count: u32,
}
#[derive(Clone, Debug)]
pub struct PushByteArg {
    pub value: u8,
}
#[derive(Clone, Debug)]
pub struct PushDoubleArg {
    pub value: u32,
}
#[derive(Clone, Debug)]
pub struct PushIntArg {
    pub value: u32,
}
#[derive(Clone, Debug)]
pub struct NamespaceArg {
    pub ns: u32,
}
#[derive(Clone, Debug)]
pub struct PushShortArg {
    pub value: i16,
}
#[derive(Clone, Debug)]
pub struct PushStringArg {
    pub value: u32,
}
#[derive(Clone, Debug)]
pub struct PushUintArg {
    pub value: u32,
}
