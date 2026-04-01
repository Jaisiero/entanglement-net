use serde::Deserialize;

#[derive(Deserialize)]
struct Schema {
    protocol: Protocol,
    message: Vec<Message>,
}

#[derive(Deserialize)]
struct Protocol {
    version: u16,
    max_payload_bytes: usize,
}

#[derive(Deserialize)]
struct Message {
    id: u16,
    name: String,
    #[allow(dead_code)]
    channel: String,
    #[allow(dead_code)]
    direction: String,
    #[serde(default)]
    fields: Vec<Field>,
    #[serde(default)]
    variable_length: bool,
    #[serde(default)]
    max_entries: u8,
    #[serde(default)]
    entry_type: Option<String>,
}

#[derive(Deserialize)]
struct Field {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

fn pascal_to_screaming_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_uppercase());
    }
    result
}

fn pascal_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

fn type_size(ty: &str) -> usize {
    match ty {
        "u8" => 1,
        "u16" => 2,
        "u32" | "f32" => 4,
        "u64" | "f64" => 8,
        _ => panic!("Unknown type: {}", ty),
    }
}

fn rust_type(ty: &str) -> &str {
    match ty {
        "u8" | "u16" | "u32" | "u64" | "f32" | "f64" => ty,
        _ => panic!("Unknown Rust type: {}", ty),
    }
}

fn c_type(ty: &str) -> &str {
    match ty {
        "u8" => "uint8_t",
        "u16" => "uint16_t",
        "u32" => "uint32_t",
        "u64" => "uint64_t",
        "f32" => "float",
        "f64" => "double",
        _ => panic!("Unknown C type: {}", ty),
    }
}

// ---------------------------------------------------------------------------
// Rust code generation
// ---------------------------------------------------------------------------

fn generate_rust(schema: &Schema) -> String {
    let mut out = String::new();

    // File header
    out.push_str("// AUTO-GENERATED — do not edit manually\n");
    out.push_str("// Source: schemas/messages.toml\n\n");

    // Protocol constants
    out.push_str(&format!(
        "pub const PROTOCOL_VERSION: u16 = {};\n",
        schema.protocol.version
    ));
    out.push_str("pub const MSG_HEADER_SIZE: usize = 6;\n");
    out.push_str(&format!(
        "pub const MAX_PAYLOAD_BYTES: usize = {};\n",
        schema.protocol.max_payload_bytes
    ));

    // msg_type module
    out.push_str("\npub mod msg_type {\n");
    for msg in &schema.message {
        let screaming = pascal_to_screaming_snake(&msg.name);
        out.push_str(&format!(
            "    pub const {}: u16 = {:#06x};\n",
            screaming, msg.id
        ));
    }
    out.push_str("}\n");

    // MsgHeader struct
    out.push_str("\n#[repr(C, packed)]\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    out.push_str("pub struct MsgHeader {\n");
    out.push_str("    pub msg_type: u16,\n");
    out.push_str("    pub msg_length: u16,\n");
    out.push_str("    pub msg_flags: u8,\n");
    out.push_str("    pub reserved: u8,\n");
    out.push_str("}\n");

    // Per-message output
    for msg in &schema.message {
        if msg.variable_length {
            let screaming = pascal_to_screaming_snake(&msg.name);
            let snake = pascal_to_snake(&msg.name);
            let entry_type = msg
                .entry_type
                .as_ref()
                .expect("variable_length message must have entry_type");
            let max = msg.max_entries as usize;

            out.push_str(&format!(
                "\n/// Variable-length batch: count (u8) + count × {}\n",
                entry_type
            ));
            out.push_str(&format!("/// Max entries: {}\n", max));
            out.push_str(&format!(
                "pub const {}_MAX_ENTRIES: usize = {};\n",
                screaming, max
            ));

            // write function
            out.push_str(&format!(
                "\n/// Write a {} into a buffer. Returns bytes written.\n",
                msg.name
            ));
            out.push_str(&format!(
                "pub fn write_{}(buf: &mut [u8], inputs: &[{}]) -> Result<usize, ()> {{\n",
                snake, entry_type
            ));
            out.push_str(&format!(
                "    let count = inputs.len().min({}_MAX_ENTRIES);\n",
                screaming
            ));
            out.push_str(&format!(
                "    let entry_size = core::mem::size_of::<{}>();\n",
                entry_type
            ));
            out.push_str("    let total = 1 + count * entry_size; // 1 byte count + entries\n");
            out.push_str("    if total > buf.len() { return Err(()); }\n");
            out.push_str("    buf[0] = count as u8;\n");
            out.push_str("    for i in 0..count {\n");
            out.push_str("        unsafe {\n");
            out.push_str("            core::ptr::write_unaligned(\n");
            out.push_str(&format!(
                "                buf[1 + i * entry_size..].as_mut_ptr() as *mut {},\n",
                entry_type
            ));
            out.push_str("                inputs[i],\n");
            out.push_str("            );\n");
            out.push_str("        }\n");
            out.push_str("    }\n");
            out.push_str("    Ok(total)\n");
            out.push_str("}\n");

            // read function
            out.push_str(&format!(
                "\n/// Read a {} from a buffer.\n",
                msg.name
            ));
            out.push_str(&format!(
                "pub fn read_{}(payload: &[u8]) -> Option<&[u8]> {{\n",
                snake
            ));
            out.push_str("    if payload.is_empty() { return None; }\n");
            out.push_str("    let count = payload[0] as usize;\n");
            out.push_str(&format!(
                "    let entry_size = core::mem::size_of::<{}>();\n",
                entry_type
            ));
            out.push_str("    let total = 1 + count * entry_size;\n");
            out.push_str("    if total > payload.len() { return None; }\n");
            out.push_str("    Some(&payload[1..total])\n");
            out.push_str("}\n");
        } else {
            // Fixed-size struct
            out.push_str("\n#[repr(C, packed)]\n");
            out.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
            out.push_str(&format!("pub struct {} {{\n", msg.name));
            for field in &msg.fields {
                out.push_str(&format!(
                    "    pub {}: {},\n",
                    field.name,
                    rust_type(&field.ty)
                ));
            }
            out.push_str("}\n");
        }
    }

    // Size assertions
    out.push_str("\nconst _: () = assert!(core::mem::size_of::<MsgHeader>() == 6);\n");
    for msg in &schema.message {
        if !msg.variable_length {
            let expected: usize = msg.fields.iter().map(|f| type_size(&f.ty)).sum();
            out.push_str(&format!(
                "const _: () = assert!(core::mem::size_of::<{}>() == {});\n",
                msg.name, expected
            ));
        }
    }

    out
}

// ---------------------------------------------------------------------------
// C header generation
// ---------------------------------------------------------------------------

fn generate_c(schema: &Schema) -> String {
    let mut out = String::new();

    // Header guard
    out.push_str("#ifndef ENTANGLEMENT_NET_H\n");
    out.push_str("#define ENTANGLEMENT_NET_H\n\n");
    out.push_str("#include <stdint.h>\n\n");

    // Protocol defines
    out.push_str(&format!(
        "#define ENT_NET_PROTOCOL_VERSION {}\n",
        schema.protocol.version
    ));
    out.push_str("#define ENT_NET_MSG_HEADER_SIZE 6\n");
    out.push_str(&format!(
        "#define ENT_NET_MAX_PAYLOAD_BYTES {}\n\n",
        schema.protocol.max_payload_bytes
    ));

    // Message type defines
    for msg in &schema.message {
        let screaming = pascal_to_screaming_snake(&msg.name);
        out.push_str(&format!(
            "#define ENT_NET_MSG_{} {:#06x}\n",
            screaming, msg.id
        ));
    }
    out.push('\n');

    // Packed structs
    out.push_str("#pragma pack(push, 1)\n\n");

    // MsgHeader
    out.push_str("typedef struct {\n");
    out.push_str("    uint16_t msg_type;\n");
    out.push_str("    uint16_t msg_length;\n");
    out.push_str("    uint8_t  msg_flags;\n");
    out.push_str("    uint8_t  reserved;\n");
    out.push_str("} ent_net_msg_header_t;\n\n");

    // Per-message structs (skip variable-length)
    for msg in &schema.message {
        if msg.variable_length {
            continue;
        }
        let snake = pascal_to_snake(&msg.name);
        out.push_str("typedef struct {\n");
        for field in &msg.fields {
            out.push_str(&format!(
                "    {} {};\n",
                c_type(&field.ty),
                field.name
            ));
        }
        out.push_str(&format!("}} ent_net_{}_t;\n\n", snake));
    }

    out.push_str("#pragma pack(pop)\n\n");

    // Static assert macro
    out.push_str("#if defined(__cplusplus)\n");
    out.push_str("  #define ENT_NET_STATIC_ASSERT(expr, msg) static_assert(expr, msg)\n");
    out.push_str("#elif defined(_Static_assert)\n");
    out.push_str("  #define ENT_NET_STATIC_ASSERT(expr, msg) _Static_assert(expr, msg)\n");
    out.push_str("#elif __STDC_VERSION__ >= 201112L\n");
    out.push_str("  #define ENT_NET_STATIC_ASSERT(expr, msg) _Static_assert(expr, msg)\n");
    out.push_str("#else\n");
    out.push_str("  #define ENT_NET_STATIC_ASSERT(expr, msg) typedef char _ent_static_assert_##__LINE__[(expr)?1:-1]\n");
    out.push_str("#endif\n\n");

    // Size assertions
    out.push_str("ENT_NET_STATIC_ASSERT(sizeof(ent_net_msg_header_t) == 6, \"MsgHeader size\");\n");
    for msg in &schema.message {
        if !msg.variable_length {
            let snake = pascal_to_snake(&msg.name);
            let expected: usize = msg.fields.iter().map(|f| type_size(&f.ty)).sum();
            out.push_str(&format!(
                "ENT_NET_STATIC_ASSERT(sizeof(ent_net_{}_t) == {}, \"{} size\");\n",
                snake, expected, msg.name
            ));
        }
    }
    out.push('\n');

    out.push_str("#endif /* ENTANGLEMENT_NET_H */\n");

    out
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let schema_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "schemas/messages.toml".to_string());
    let rust_out = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "entanglement-net/src/messages.rs".to_string());
    let c_out = std::env::args()
        .nth(3)
        .unwrap_or_else(|| "include/entanglement_net.h".to_string());

    let content = std::fs::read_to_string(&schema_path).expect("Failed to read schema");
    let schema: Schema = toml::from_str(&content).expect("Failed to parse schema");

    let rust_code = generate_rust(&schema);
    let c_code = generate_c(&schema);

    if let Some(parent) = std::path::Path::new(&rust_out).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    if let Some(parent) = std::path::Path::new(&c_out).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    std::fs::write(&rust_out, rust_code).expect("Failed to write Rust output");
    std::fs::write(&c_out, c_code).expect("Failed to write C output");

    println!("Generated {} and {}", rust_out, c_out);
}
