//! Bytecode prefix-hook builder: insert `load args; invokestatic target; pop-result`
//! at the start of a method. Extends the SDK's `weave::insert_call_at_start`
//! with argument loading (needed for probes on setPos / EntityLookup add+remove).
//!
//! Safety properties (mirroring cplug-sdk::weave):
//! * the prefix is stack-neutral (all loaded args are consumed by the static
//!   call, its result — if any — is popped), so max_stack and frames stay valid;
//! * offset-sensitive structures shift uniformly with the insertion delta;
//! * any method containing tableswitch/lookupswitch/wide is REFUSED (fail-safe:
//!   the module logs and leaves the class untouched);
//! * EVERY reader is bounds-checked: malformed or unexpected input yields
//!   Err(...), never a panic — a panic inside a JVMTI callback aborts the JVM.
//!
//! Offset rules used:
//! * exception table: start_pc/end_pc/handler_pc all += delta;
//! * LineNumberTable: start_pc += delta (debug-only, cosmetic);
//! * LocalVariableTable/TypeTable: start_pc == 0 entries keep 0 and grow
//!   their length by delta (the variable is live across the new prefix);
//!   entries with start_pc > 0 shift start_pc by delta, length unchanged —
//!   in both cases the live range END shifts by exactly delta;
//! * StackMapTable: the FIRST frame's offset += delta (frame-to-frame deltas
//!   of later frames are relative and stay untouched).

#![forbid(unsafe_code)]

fn rd_u16(b: &[u8], i: usize) -> Result<u16, String> {
    if b.len() < i + 2 {
        return Err(format!("oob u16 read at {i} (len {})", b.len()));
    }
    Ok(u16::from_be_bytes([b[i], b[i + 1]]))
}
fn rd_u32(b: &[u8], i: usize) -> Result<u32, String> {
    if b.len() < i + 4 {
        return Err(format!("oob u32 read at {i} (len {})", b.len()));
    }
    Ok(u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]))
}
fn w16(b: &mut Vec<u8>, i: usize, v: u16) {
    b[i..i + 2].copy_from_slice(&v.to_be_bytes());
}
fn w32(b: &mut Vec<u8>, i: usize, v: u32) {
    b[i..i + 4].copy_from_slice(&v.to_be_bytes());
}
fn push_utf8(c: &mut Vec<u8>, s: &str) {
    c.push(1);
    let b = s.as_bytes();
    c.extend_from_slice(&(b.len() as u16).to_be_bytes());
    c.extend_from_slice(b);
}

struct CpInfo {
    count: u16,
    end: usize, // offset just past the constant pool
}

fn parse_cp(bytes: &[u8]) -> Result<CpInfo, String> {
    if bytes.len() < 10 || rd_u32(bytes, 0)? != 0xCAFE_BABE {
        return Err("not a classfile".into());
    }
    let count = rd_u16(bytes, 8)?;
    let mut i = 10usize;
    let mut idx = 1u16;
    while idx < count {
        if i >= bytes.len() {
            return Err("cp walk ran past end of file".into());
        }
        let tag = bytes[i];
        i += 1;
        match tag {
            1 => {
                let len = rd_u16(bytes, i)? as usize;
                i += 2 + len;
            }
            7 | 8 | 16 | 19 | 20 => i += 2,
            15 => i += 3,
            5 | 6 => {
                i += 8;
                idx += 1; // long/double take two slots
            }
            3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => i += 4,
            t => return Err(format!("bad cp tag {t} at {i}")),
        }
        idx += 1;
    }
    if i > bytes.len() {
        return Err("cp walk overran file".into());
    }
    Ok(CpInfo { count, end: i })
}

#[allow(dead_code)]
struct CodeRegion {
    code_start: usize,
    code_end: usize,
    code_len_pos: usize,
    attr_len_pos: usize,
    max_stack: u16,
    max_stack_pos: usize,
    exc_start: usize, // absolute offset of exception_table_length u16
    attr_end: usize,  // absolute end of the Code attribute
}

/// Resolve a CONSTANT_Utf8 entry by index.
fn utf8_at(bytes: &[u8], cp_count: u16, idx: u16) -> Option<String> {
    let mut i = 10usize;
    let mut cur = 1u16;
    while cur < cp_count {
        let tag = *bytes.get(i)?;
        i += 1;
        match tag {
            1 => {
                let len = rd_u16(bytes, i).ok()? as usize;
                if cur == idx {
                    return Some(String::from_utf8_lossy(bytes.get(i + 2..i + 2 + len)?).into_owned());
                }
                i += 2 + len;
            }
            7 | 8 | 16 | 19 | 20 => i += 2,
            15 => i += 3,
            5 | 6 => {
                i += 8;
                cur += 1;
            }
            3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => i += 4,
            _ => return None,
        }
        cur += 1;
    }
    None
}

/// Locate `method(name, desc)`'s Code attribute. Bounds-checked throughout.
fn find_code(bytes: &[u8], cp: &CpInfo, name: &str, desc: &str) -> Result<CodeRegion, String> {
    let cp_end = cp.end;
    let mut i = cp_end + 6; // access, this, super
    if i > bytes.len() {
        return Err("class header past end".into());
    }
    let icount = rd_u16(bytes, i)? as usize;
    i += 2 + icount * 2;
    let fcount = rd_u16(bytes, i)? as usize;
    i += 2;
    // fields
    for _ in 0..fcount {
        if i + 6 > bytes.len() {
            return Err("field header past end".into());
        }
        i += 6;
        let ac = rd_u16(bytes, i)? as usize;
        i += 2;
        for _ in 0..ac {
            let alen = rd_u32(bytes, i + 2)? as usize; // name(2) + len(4)
            i += 6 + alen;
        }
    }
    let mcount = rd_u16(bytes, i)? as usize;
    i += 2;
    for _ in 0..mcount {
        if i + 6 > bytes.len() {
            return Err("method header past end".into());
        }
        let m_acc = rd_u16(bytes, i)?;
        let m_name = rd_u16(bytes, i + 2)?;
        let m_desc = rd_u16(bytes, i + 4)?;
        i += 6;
        let ac = rd_u16(bytes, i)? as usize;
        i += 2;
        let matched = utf8_at(bytes, cp.count, m_name).as_deref() == Some(name)
            && utf8_at(bytes, cp.count, m_desc).as_deref() == Some(desc);
        for _ in 0..ac {
            let a_name = rd_u16(bytes, i)?;
            let a_len = rd_u32(bytes, i + 2)? as usize;
            let a_start = i + 6;
            if matched && utf8_at(bytes, cp.count, a_name).as_deref() == Some("Code") {
                if (m_acc & 0x0100) != 0 {
                    return Err("native/abstract method has no Code".into());
                }
                if a_start + 8 > bytes.len() {
                    return Err("code header past end".into());
                }
                let max_stack_pos = a_start;
                let max_stack = rd_u16(bytes, a_start)?;
                let code_len_pos = a_start + 4;
                let clen = rd_u32(bytes, code_len_pos)? as usize;
                let code_start = code_len_pos + 4;
                let code_end = code_start
                    .checked_add(clen)
                    .ok_or("code length overflow")?;
                if code_end > bytes.len() {
                    return Err("code array past end".into());
                }
                // exception_table_length sits IMMEDIATELY after the code array
                let exc_start = code_end;
                let exc_count = rd_u16(bytes, exc_start)? as usize;
                let attr_pos = exc_start + 2 + exc_count * 8;
                if attr_pos + 2 > bytes.len() {
                    return Err("code attr table past end".into());
                }
                let attr_count = rd_u16(bytes, attr_pos)? as usize;
                // nested attributes are VARIABLE-length: name(2)+len(4)+data
                let mut q = attr_pos + 2;
                for _ in 0..attr_count {
                    let al = rd_u32(bytes, q + 2)? as usize;
                    q += 6 + al;
                }
                let attr_end = q;
                if attr_end > bytes.len() {
                    return Err("code attr overruns file".into());
                }
                return Ok(CodeRegion {
                    code_start,
                    code_end,
                    code_len_pos,
                    attr_len_pos: a_start - 4, // attribute_length of the Code attribute
                    max_stack,
                    max_stack_pos,
                    exc_start,
                    attr_end,
                });
            }
            i = a_start + a_len;
        }
    }
    Err(format!("method {name}{desc} not found"))
}

/// Load instruction for a local slot by JVM type char.
fn load_op(ty: u8, slot: u16) -> Result<Vec<u8>, String> {
    if slot > 255 {
        return Err("wide slots unsupported".into());
    }
    let op = match ty {
        b'L' | b'[' => 0x19, // aload (indexed; 0x2a is the no-operand aload_0)
        b'D' => 0x18,        // dload
        b'J' => 0x1e,        // lload
        b'F' => 0x17,        // fload
        b'I' | b'Z' | b'B' | b'C' | b'S' => 0x15, // iload
        t => return Err(format!("bad arg type {t}")),
    };
    Ok(vec![op, slot as u8])
}

/// Insert a stack-neutral static call prefix with loaded arguments.
/// `args`: (local slot, type char) in call order. Returns patched bytes.
pub fn insert_static_call_at_start(
    bytes: Vec<u8>,
    method_name: &str,
    method_desc: &str,
    target_class: &str,
    target_name: &str,
    target_desc: &str,
    args: &[(u16, u8)],
) -> Result<Vec<u8>, String> {
    let cp = parse_cp(&bytes)?;
    let region = find_code(&bytes, &cp, method_name, method_desc)?;

    // only V is supported for probes (keeps the patch strictly side-effect-neutral)
    if !target_desc.ends_with(")V") {
        return Err("probe target must return void".into());
    }

    // CP additions
    let mut additions: Vec<u8> = Vec::new();
    let mut next = cp.count - 1;
    let class_name_idx = next + 1;
    push_utf8(&mut additions, target_class);
    next += 1;
    let name_idx = next + 1;
    push_utf8(&mut additions, target_name);
    next += 1;
    let desc_idx = next + 1;
    push_utf8(&mut additions, target_desc);
    next += 1;
    let class_idx = next + 1;
    additions.push(7);
    additions.extend_from_slice(&class_name_idx.to_be_bytes());
    next += 1;
    let nat_idx = next + 1;
    additions.push(12);
    additions.extend_from_slice(&name_idx.to_be_bytes());
    additions.extend_from_slice(&desc_idx.to_be_bytes());
    next += 1;
    let mref_idx = next + 1;
    additions.push(10);
    additions.extend_from_slice(&class_idx.to_be_bytes());
    additions.extend_from_slice(&nat_idx.to_be_bytes());
    if cp.count as usize + additions_count(&additions) as usize > 0xFFFF {
        return Err("constant pool overflow".into());
    }

    // prefix: arg loads + invokestatic
    let mut prefix: Vec<u8> = Vec::new();
    for (slot, ty) in args {
        prefix.extend_from_slice(&load_op(*ty, *slot)?);
    }
    prefix.push(0xb8);
    prefix.extend_from_slice(&mref_idx.to_be_bytes());
    let delta = prefix.len();
    if delta > u16::MAX as usize {
        return Err("prefix too large".into());
    }

    // Precise linear scan of the method body: walk opcode-by-opcode with an
    // operand-length table and refuse only if a REAL switch/wide instruction
    // is present (their 4-byte alignment padding would shift non-uniformly).
    // The old naive byte-scan produced false refusals on constant bytes.
    scan_no_switch(&bytes[region.code_start..region.code_end])?;

    // ---- splice: grow the code array by delta, then fix offsets ----
    let old_code_len = region.code_end - region.code_start;
    let mut out = bytes;
    out.splice(region.code_start..region.code_start, prefix.iter().copied());
    // everything after code_start shifted by delta; patch lengths & offsets
    w32(&mut out, region.code_len_pos, (old_code_len + delta) as u32);
    let alen = rd_u32(&out, region.attr_len_pos)? + delta as u32;
    w32(&mut out, region.attr_len_pos, alen);
    w16(&mut out, region.max_stack_pos, region.max_stack.max(1));

    // exception table: start_pc, end_pc, handler_pc shift by delta
    let exc_start = region.exc_start + delta;
    let exc_count = rd_u16(&out, exc_start)? as usize;
    for k in 0..exc_count {
        let p = exc_start + 2 + k * 8;
        for off in 0..3 {
            let q = p + off * 2;
            let v = rd_u16(&out, q)? + delta as u16;
            w16(&mut out, q, v);
        }
    }

    // nested attributes of the Code attribute
    let attr_pos = exc_start + 2 + exc_count * 8;
    let attr_count = rd_u16(&out, attr_pos)? as usize;
    let mut p = attr_pos + 2;
    for _ in 0..attr_count {
        let a_start = p;
        if a_start + 6 > out.len() {
            return Err("nested attr header past end".into());
        }
        let a_name = rd_u16(&out, a_start)?;
        let a_len = rd_u32(&out, a_start + 2)? as usize;
        let data = a_start + 6;
        if data + a_len > out.len() {
            return Err("nested attr overruns file".into());
        }
        let aname = utf8_at(&out, cp.count, a_name);
        match aname.as_deref() {
            Some("LineNumberTable") => {
                let n = rd_u16(&out, data)? as usize;
                for k in 0..n {
                    let q = data + 2 + k * 4;
                    let v = rd_u16(&out, q)? + delta as u16;
                    w16(&mut out, q, v);
                }
            }
            Some("LocalVariableTable") | Some("LocalVariableTypeTable") => {
                let n = rd_u16(&out, data)? as usize;
                for k in 0..n {
                    let q = data + 2 + k * 10;
                    let start = rd_u16(&out, q)?;
                    let len = rd_u16(&out, q + 2)?;
                    if start == 0 {
                        // live across the inserted prefix: keep 0, grow length
                        w16(&mut out, q + 2, len + delta as u16);
                    } else {
                        // live range fully inside the original bytecode:
                        // both ends shift by delta, length is unchanged
                        w16(&mut out, q, start + delta as u16);
                    }
                }
            }
            Some("StackMapTable") => {
                let n = rd_u16(&out, data)? as usize;
                let mut q = data + 2;
                for k in 0..n {
                    if q >= out.len() {
                        return Err("stack map frame past end".into());
                    }
                    let frame_type = out[q];
                    q += 1;
                    match frame_type {
                        0..=63 => {
                            // same_frame: the offset-1 IS the tag
                            if k == 0 {
                                let new_off = (frame_type as u16) + delta as u16;
                                if new_off > 63 {
                                    return Err("StackMapTable re-encoding refused".into());
                                }
                                out[q - 1] = new_off as u8;
                            }
                        }
                        64..=127 => {
                            // same_locals_1_stack_item: offset-1 in the tag
                            if k == 0 {
                                let new_off = (frame_type as u32) - 64 + delta as u32;
                                if new_off > 63 {
                                    return Err("StackMapTable re-encoding refused".into());
                                }
                                out[q - 1] = (new_off + 64) as u8;
                            }
                            q += skip_verification_info(&out, q, 1)?;
                        }
                        247 => {
                            if k == 0 {
                                let v = rd_u16(&out, q)? + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                            q += skip_verification_info(&out, q, 1)?;
                        }
                        248..=250 => {
                            // chop_frame
                            if k == 0 {
                                let v = rd_u16(&out, q)? + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                        }
                        251 => {
                            // same_frame_extended
                            if k == 0 {
                                let v = rd_u16(&out, q)? + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                        }
                        252..=254 => {
                            // append_frame
                            if k == 0 {
                                let v = rd_u16(&out, q)? + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                            q += skip_verification_info(&out, q, (frame_type - 251) as usize)?;
                        }
                        255 => {
                            // full_frame
                            if k == 0 {
                                let v = rd_u16(&out, q)? + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                            let nl = rd_u16(&out, q)? as usize;
                            q += 2;
                            for _ in 0..nl {
                                q += skip_verification_info(&out, q, 1)?;
                            }
                            let ns = rd_u16(&out, q)? as usize;
                            q += 2;
                            for _ in 0..ns {
                                q += skip_verification_info(&out, q, 1)?;
                            }
                        }
                        t => return Err(format!("bad frame type {t}")),
                    }
                    if q > out.len() {
                        return Err("stack map walk overran".into());
                    }
                }
            }
            _ => {}
        }
        p = data + a_len;
    }

    // constant pool count += additions
    w16(&mut out, 8, cp.count + additions_count(&additions));
    out.splice(cp.end..cp.end, additions.iter().copied());
    Ok(out)
}

/// Replace the ENTIRE body of `method(name, desc)` with
/// `load args; invokestatic bridge; areturn`.
///
/// Used for the Level query swap: the replacement body is straight-line
/// (no branches), so the method needs NO StackMapTable; we also drop the
/// exception table and the debug tables (LNT/LVT) of the old body. The
/// whole rewrite is pure Rust with bounds-checked reads — crucially it
/// performs ZERO JVM calls inside the ClassFileLoadHook (no defineClass,
/// no ASM), which deadlocks when invoked from a retransform while the
/// world is stopped.
///
/// `bridge_desc` must return a reference type matching the method's return.
pub fn replace_body_with_static(
    bytes: Vec<u8>,
    method_name: &str,
    method_desc: &str,
    target_class: &str,
    target_name: &str,
    target_desc: &str,
    arg_slots: &[u16],
) -> Result<Vec<u8>, String> {
    let cp = parse_cp(&bytes)?;
    let region = find_code(&bytes, &cp, method_name, method_desc)?;

    // CP additions (same as the prefix builder)
    let mut additions: Vec<u8> = Vec::new();
    let mut next = cp.count - 1;
    let class_name_idx = next + 1;
    push_utf8(&mut additions, target_class);
    next += 1;
    let name_idx = next + 1;
    push_utf8(&mut additions, target_name);
    next += 1;
    let desc_idx = next + 1;
    push_utf8(&mut additions, target_desc);
    next += 1;
    let class_idx = next + 1;
    additions.push(7);
    additions.extend_from_slice(&class_name_idx.to_be_bytes());
    next += 1;
    let nat_idx = next + 1;
    additions.push(12);
    additions.extend_from_slice(&name_idx.to_be_bytes());
    additions.extend_from_slice(&desc_idx.to_be_bytes());
    next += 1;
    let mref_idx = next + 1;
    additions.push(10);
    additions.extend_from_slice(&class_idx.to_be_bytes());
    additions.extend_from_slice(&nat_idx.to_be_bytes());
    if cp.count as usize + additions_count(&additions) as usize > 0xFFFF {
        return Err("constant pool overflow".into());
    }

    // new straight-line body: arg loads + invokestatic + areturn
    let mut new_code: Vec<u8> = Vec::new();
    for slot in arg_slots {
        // all Level query methods take 4 reference args (this, ...)
        new_code.extend_from_slice(&load_op(b'L', *slot)?);
    }
    new_code.push(0xb8);
    new_code.extend_from_slice(&mref_idx.to_be_bytes());
    new_code.push(0xb0); // areturn
    let new_clen = new_code.len();
    if new_clen > u32::MAX as usize {
        return Err("new body too large".into());
    }

    // reassemble the Code attribute:
    //   max_stack(2) max_locals(2) code_length(4) code exc_count(2) attrs_count(2)
    let max_locals = rd_u16(&bytes, region.max_stack_pos + 2)?;
    let mut attr: Vec<u8> = Vec::with_capacity(8 + new_clen + 4);
    attr.extend_from_slice(&4u16.to_be_bytes()); // max_stack: 4 refs on stack
    attr.extend_from_slice(&max_locals.to_be_bytes());
    attr.extend_from_slice(&(new_clen as u32).to_be_bytes());
    attr.extend_from_slice(&new_code);
    attr.extend_from_slice(&0u16.to_be_bytes()); // no exception table
    attr.extend_from_slice(&0u16.to_be_bytes()); // no nested attributes

    // splice: replace [max_stack .. attr_end) — the whole Code data — with
    // the new header+body (`attr` starts with max_stack/max_locals)
    let mut out = bytes;
    let attr_len = attr.len() as u32;
    out.splice(region.max_stack_pos..region.attr_end, attr);
    // Code attribute_length: the replaced region starts at max_stack, so the
    // new attribute data length is exactly attr.len(); the u4 field sits at
    // attr_len_pos (before code_len_pos — unaffected by the splice).
    w32(&mut out, region.attr_len_pos, attr_len);

    w16(&mut out, 8, cp.count + additions_count(&additions));
    out.splice(cp.end..cp.end, additions.iter().copied());
    Ok(out)
}

fn utf8_by_index(bytes: &[u8], cp: &CpInfo, idx: u16) -> Option<String> {
    let mut i = 10usize;
    let mut cur = 1u16;
    while cur < cp.count {
        let tag = *bytes.get(i)?;
        i += 1;
        match tag {
            1 => {
                let len = rd_u16(bytes, i).ok()? as usize;
                if cur == idx {
                    return Some(String::from_utf8_lossy(bytes.get(i + 2..i + 2 + len)?).into_owned());
                }
                i += 2 + len;
            }
            7 | 8 | 16 | 19 | 20 => i += 2,
            15 => i += 3,
            5 | 6 => {
                i += 8;
                cur += 1;
            }
            3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => i += 4,
            _ => return None,
        }
        cur += 1;
    }
    None
}

fn skip_verification_info(b: &[u8], mut q: usize, n: usize) -> Result<usize, String> {
    let start = q;
    for _ in 0..n {
        let tag = *b.get(q).ok_or("verification info past end")?;
        q += 1;
        match tag {
            // ITEM_Top..ITEM_UninitializedThis carry no payload
            0..=6 => {}
            // ITEM_Object (cpool index) and ITEM_Uninitialized (offset)
            7 | 8 => q += 2,
            t => return Err(format!("bad verification_info tag {t}")),
        }
    }
    Ok(q - start)
}

fn additions_count(add: &[u8]) -> u16 {
    let mut n = 0u16;
    let mut i = 0usize;
    while i < add.len() {
        let tag = add[i];
        i += 1;
        match tag {
            1 => {
                let len = u16::from_be_bytes([add[i], add[i + 1]]) as usize;
                i += 2 + len;
                n += 1;
            }
            7 | 8 | 16 | 19 | 20 => {
                i += 2;
                n += 1;
            }
            15 => {
                i += 3;
                n += 1;
            }
            5 | 6 => {
                i += 8;
                n += 2;
            }
            _ => {
                i += 4;
                n += 1;
            }
        }
    }
    n
}

pub const OP_LEN: [u8; 202] = [
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    1,2,1,2,2,1,1,1,1,1,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,2,2,2,2,2,2,2,
    2,2,2,2,2,2,2,2,2,1,255,255,0,0,0,0,
    0,0,2,2,2,2,2,2,2,4,4,2,1,2,0,0,
    2,2,0,0,255,3,2,2,4,4,
];

/// Linear scan: refuse if the body contains a real tableswitch, lookupswitch
/// or wide instruction (their 4-byte alignment padding would shift
/// non-uniformly); also validates that the code disassembles cleanly.
fn scan_no_switch(code: &[u8]) -> Result<(), String> {
    let mut i = 0usize;
    while i < code.len() {
        let op = code[i] as usize;
        match OP_LEN.get(op) {
            Some(255) => return Err(format!("switch/wide opcode {op:#04x} refused")),
            Some(&n) => i += 1 + n as usize,
            None => return Err(format!("invalid opcode {op:#04x} at {i}")),
        }
    }
    if i != code.len() {
        return Err("opcode scan misaligned".into());
    }
    Ok(())
}
