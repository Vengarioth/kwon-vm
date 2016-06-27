pub struct Pointer {
    address: u32
}

pub struct TypeDefinition {
    type_id: u32,
    type_size: u16
}

pub struct StructureDefinition {
    type_id: u32,
    type_size: u16,
    field_count: u8,
    field_definitions: [FieldDefinition]
}

pub struct FieldDefinition {
    field_id: u8,
    type_definition: Pointer
}
