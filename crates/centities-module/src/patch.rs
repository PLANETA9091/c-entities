//! Bytecode prefix-hook builder: insert `load args; invokestatic target; pop-result`
//! at the start of a method. Extends the SDK's `weave::insert_call_at_start`
//! with argument loading (needed for probes on setPos / EntityLookup add+remove).
//!
//! Safety properties (mirroring cplug-sdk::weave):
//! * the prefix is stack-neutral (all loaded args are consumed by the static
//!   call, its result — if any — is popped), so max_stack and frames stay valid;
//! * offset-sensitive structures shift uniformly with the insertion delta;
//! * any method containing tableswitch/lookupswitch is REFUSED (fail-safe:
//!   the module logs and leaves the class untouched).
//!
//! All offsets patched: exception table entries, the first StackMapTable
//! frame's offset_delta (frame-to-frame deltas are relative), LineNumberTable
//! and LocalVariableTable start_pc/length.

#![forbid(unsafe_code)]

fn u16at(b: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([b[i], b[i + 1]])
}
fn u32at(b: &[u8], i: usize) -> u32 {
    u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
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

/// Walk the method's bytecode instruction by instruction and report whether
/// it contains tableswitch (0xaa), lookupswitch (0xab) or wide (0xc4).
///
/// Variable-length instructions make a naive byte scan meaningless: any
/// operand byte can equal 0xaa. Fixed lengths follow the JVM spec (JVMS §6.5).
fn code_has_switch_or_wide(code: &[u8]) -> bool {
    fn fixed_len(op: u8) -> Option<usize> {
        Some(match op {
            0x00..=0x0f => 1, // consts, aconst_null
            0x10 => 2,        // bipush
            0x11 => 3,        // sipush
            0x12 => 2,        // ldc
            0x13 | 0x14 => 3, // ldc_w, ldc2_w
            0x15..=0x19 => 2, // xload n
            0x1a..=0x35 => 1, // xload_n, array loads
            0x36..=0x3a => 2, // xstore n
            0x3b..=0x56 => 1, // xstore_n, array stores
            0x57..=0x5f => 1, // stack ops
            0x60..=0x83 => 1, // arithmetic, bitwise
            0x84 => 3,        // iinc
            0x85..=0x93 => 1, // conversions
            0x94..=0x98 => 1, // lcmp, fcmpl..dcmpg
            0x99..=0xa8 => 3, // if*, goto, jsr
            0xa9 => 2,        // ret
            0xac..=0xb1 => 1, // returns
            0xb2..=0xb8 => 3, // field/method accesses
            0xb9 | 0xba => 5, // invokeinterface, invokedynamic
            0xbb => 3,        // new
            0xbc => 2,        // newarray
            0xbd => 3,        // anewarray
            0xbe | 0xbf => 1, // arraylength, athrow
            0xc0 | 0xc1 => 3, // checkcast, instanceof
            0xc2 | 0xc3 => 1, // monitor enter/exit
            0xc5 => 4,        // multianewarray
            0xc6 | 0xc7 => 3, // ifnull, ifnonnull
            0xc8 | 0xc9 => 5, // goto_w, jsr_w
            _ => return None, // unknown/reserved — refuse
        })
    }
    let mut pc = 0usize;
    while pc < code.len() {
        let op = code[pc];
        match op {
            0xaa | 0xab | 0xc4 => return true, // tableswitch/lookupswitch/wide
            _ => {}
        }
        let len = match op {
            0xaa => {
                // tableswitch: pad to 4-align (from code start), then
                // default + low + high + (high-low+1) offsets
                let operand = pc + 1;
                let pad = (4 - (operand % 4)) % 4;
                if operand + pad + 12 > code.len() {
                    return true;
                }
                let low = u32at(code, operand + pad + 4) as i32;
                let high = u32at(code, operand + pad + 8) as i32;
                if high < low {
                    return true;
                }
                let count = (high - low + 1) as usize;
                (pad + 12 + count * 4).saturating_add(1)
            }
            0xab => {
                // lookupswitch: pad, default + npairs + npairs*(match+offset)
                let operand = pc + 1;
                let pad = (4 - (operand % 4)) % 4;
                if operand + pad + 8 > code.len() {
                    return true;
                }
                let npairs = u32at(code, operand + pad + 4) as usize;
                (pad + 8 + npairs * 8).saturating_add(1)
            }
            0xc4 => {
                // wide: xload/xstore/ret are 4 bytes extra, iinc 6
                let sub = *code.get(pc + 1).unwrap_or(&0xff);
                if sub == 0x84 {
                    6
                } else if (0x15..=0x19).contains(&sub)
                    || (0x36..=0x3a).contains(&sub)
                    || sub == 0xa9
                {
                    4
                } else {
                    return true; // malformed wide — refuse
                }
            }
            _ => match fixed_len(op) {
                Some(l) => l,
                None => return true, // unknown opcode — refuse
            },
        };
        pc += len;
    }
    // exact landing = clean walk; overshoot = misaligned/malformed — refuse
    pc != code.len()
}

fn parse_cp(bytes: &[u8]) -> Result<CpInfo, String> {
    if bytes.len() < 10 || u32at(bytes, 0) != 0xCAFE_BABE {
        return Err("not a classfile".into());
    }
    let count = u16at(bytes, 8);
    let mut i = 10usize;
    let mut idx = 1u16;
    while idx < count {
        let tag = bytes[i];
        i += 1;
        match tag {
            1 => {
                let len = u16at(bytes, i) as usize;
                i += 2 + len;
            }
            7 | 8 | 16 | 19 | 20 => i += 2,
            15 => i += 3,
            5 | 6 => {
                i += 8;
                idx += 1; // long/double take two slots
            }
            3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => i += 4,
            t => return Err(format!("bad cp tag {t}")),
        }
        idx += 1;
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
    exc_start: usize, // absolute offset of exception_table (after the code array)
    attr_end: usize,  // absolute end of the Code attribute
}

/// Locate `method(name, desc)`'s Code attribute. Generic attribute walker.
fn find_code(bytes: &[u8], cp: &CpInfo, name: &str, desc: &str) -> Result<CodeRegion, String> {
    let cp_end = cp.end;
    let utf8_at = |bytes: &[u8], idx: u16| -> Option<String> {
        // resolve a CONSTANT_Utf8 index
        let mut i = 10usize;
        let mut cur = 1u16;
        while cur < cp.count {
            let tag = bytes[i];
            i += 1;
            match tag {
                1 => {
                    let len = u16at(bytes, i) as usize;
                    if cur == idx {
                        return Some(
                            String::from_utf8_lossy(&bytes[i + 2..i + 2 + len]).into_owned(),
                        );
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
    };

    let mut i = cp_end + 6; // access, this, super
    let icount = u16at(bytes, i);
    i += 2 + icount as usize * 2;
    let fcount = u16at(bytes, i);
    i += 2;
    for _ in 0..fcount {
        i += 6;
        let ac = u16at(bytes, i);
        i += 2;
        for _ in 0..ac {
            i += 2;
            let len = u32at(bytes, i) as usize;
            i += 4 + len;
        }
    }
    let mcount = u16at(bytes, i);
    i += 2;
    for _ in 0..mcount {
        let m_acc = u16at(bytes, i);
        let m_name = u16at(bytes, i + 2);
        let m_desc = u16at(bytes, i + 4);
        i += 6;
        let ac = u16at(bytes, i);
        i += 2;
        let matched = utf8_at(bytes, m_name).as_deref() == Some(name)
            && utf8_at(bytes, m_desc).as_deref() == Some(desc);
        for _ in 0..ac {
            let a_name = u16at(bytes, i);
            let a_len = u32at(bytes, i + 2) as usize;
            let a_start = i + 6;
            if matched && utf8_at(bytes, a_name).as_deref() == Some("Code") {
                if (m_acc & 0x0100) != 0 {
                    return Err("native/abstract method has no Code".into());
                }
                let max_stack_pos = a_start;
                let max_stack = u16at(bytes, a_start);
                let code_len_pos = a_start + 4;
                let clen = u32at(bytes, code_len_pos) as usize;
                let code_start = code_len_pos + 4;
                let code_end = code_start + clen;
                // JVMS Code layout: [max_stack u16][max_locals u16][code_len u32]
                // [code..][exc_table_len u16][exc entries 8*N][attr_count u16][attrs..]
                let exc_start = code_end; // absolute of the exception_table_length u16
                let exc_count = u16at(bytes, exc_start) as usize;
                let attr_pos = exc_start + 2 + exc_count * 8;
                let attr_count = u16at(bytes, attr_pos) as usize;
                let mut attr_end = attr_pos + 2;
                let mut p = attr_pos + 2;
                for _ in 0..attr_count {
                    // attributes have VARIABLE length — walk sequentially
                    let a_len = u32at(bytes, p + 2) as usize;
                    attr_end += 6 + a_len;
                    p += 6 + a_len;
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

    // sanity: target desc must start with the arg types and end with )V or match pop logic
    if !target_desc.ends_with(")V") {
        // only V is supported for probes (keep the patch strictly side-effect-neutral)
        return Err("probe target must return void".into());
    }

    // CP additions
    let (mref_idx, additions) = build_methodref(&cp, target_class, target_name, target_desc);

    // prefix: arg loads + invokestatic
    let mut prefix: Vec<u8> = Vec::new();
    for (slot, ty) in args {
        prefix.extend_from_slice(&load_op(*ty, *slot)?);
    }
    prefix.push(0xb8);
    prefix.extend_from_slice(&mref_idx.to_be_bytes());
    splice_prefix(
        bytes,
        &cp,
        &region,
        prefix,
        additions,
        args.len() as u16 + 1,
    )
}

/// Compute the constant-pool additions for a Methodref (class/name/desc)
/// and return (new_methodref_index, addition_bytes). The caller splices the
/// bytes at `cp.end` AFTER the code-region edits (the CP precedes the code,
/// so a late splice at cp.end shifts only unread positions).
fn build_methodref(
    cp: &CpInfo,
    target_class: &str,
    target_name: &str,
    target_desc: &str,
) -> (u16, Vec<u8>) {
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
    (mref_idx, additions)
}

/// Splice a bytecode prefix at the method's start and shift every
/// offset-sensitive structure. `additions` = the CP bytes the prefix
/// references; `min_stack` = the stack depth the prefix itself needs.
fn splice_prefix(
    bytes: Vec<u8>,
    cp: &CpInfo,
    region: &CodeRegion,
    prefix: Vec<u8>,
    additions: Vec<u8>,
    min_stack: u16,
) -> Result<Vec<u8>, String> {
    let delta = prefix.len();

    // refuse switches conservatively — but detect them by real opcode
    // walking, not a byte scan: a naive `contains(0xaa)` false-positives on
    // immediate/operand bytes (e.g. EntityLookup.addEntity) and refuses
    // perfectly patchable methods. Refusing too much is the safe direction
    // (module falls back to vanilla paths).
    let code = &bytes[region.code_start..region.code_end];
    if code_has_switch_or_wide(code) {
        return Err("switch/wide opcode in method body refused".into());
    }

    // ---- splice: grow the code array by delta, then fix offsets ----
    let old_code_len = region.code_end - region.code_start;
    let mut out = bytes;
    out.splice(region.code_start..region.code_start, prefix.iter().copied());
    // everything after code_start shifted by delta; patch lengths & offsets
    w32(&mut out, region.code_len_pos, (old_code_len + delta) as u32);
    let alen = u32at(&out, region.attr_len_pos) + delta as u32;
    w32(&mut out, region.attr_len_pos, alen);
    w16(
        &mut out,
        region.max_stack_pos,
        region.max_stack.max(min_stack),
    );

    // exception table: start_pc, end_pc, handler_pc shift by delta
    let exc_start = region.exc_start + delta;
    let exc_count = u16at(&out, exc_start) as usize;
    for k in 0..exc_count {
        let p = exc_start + 2 + k * 8;
        for off in 0..3 {
            let q = p + off * 2;
            let v = u16at(&out, q) + delta as u16;
            w16(&mut out, q, v);
        }
    }

    // nested attributes of the Code attribute
    let attr_pos = exc_start + 2 + exc_count * 8;
    let attr_count = u16at(&out, attr_pos) as usize;
    let mut p = attr_pos + 2;
    for _ in 0..attr_count {
        let a_start = p;
        // attribute name index + length
        let name_idx = u16at(&out, a_start);
        let a_len = u32at(&out, a_start + 2) as usize;
        let data = a_start + 6;
        // resolve attribute name via the CP (utf8 entries only)
        let aname = utf8_by_index(&out, &cp, name_idx);
        match aname.as_deref() {
            Some("LineNumberTable") => {
                let n = u16at(&out, data) as usize;
                for k in 0..n {
                    let q = data + 2 + k * 4;
                    let v = u16at(&out, q) + delta as u16;
                    w16(&mut out, q, v);
                }
            }
            Some("LocalVariableTable") | Some("LocalVariableTypeTable") => {
                let n = u16at(&out, data) as usize;
                for k in 0..n {
                    let q = data + 2 + k * 10;
                    // ONLY start_pc shifts: the entry's `length` field is a
                    // SPAN (how long the variable is live), not an absolute
                    // offset — shifting it corrupts the table and HotSpot
                    // rejects the class with ClassFormatError "Invalid
                    // length ... in LocalVariableTable".
                    let v = u16at(&out, q) + delta as u16; // start_pc
                    w16(&mut out, q, v);
                }
            }
            Some("StackMapTable") => {
                let n = u16at(&out, data) as usize;
                let mut q = data + 2;
                for k in 0..n {
                    let frame_type = out[q];
                    q += 1;
                    match frame_type {
                        0..=63 => {
                            // same_frame — no offset field... except it IS the offset
                            if k == 0 {
                                // same_frame encodes the absolute offset-1; shift it
                                // by rewriting the whole entry as same_frame with new offset
                                let new_off = (frame_type as u16) + delta as u16;
                                if new_off > 63 {
                                    // needs re-encoding — refuse for safety
                                    return Err("StackMapTable re-encoding refused".into());
                                }
                                out[q - 1] = new_off as u8;
                            }
                        }
                        64..=127 => {
                            // same_locals_1_stack_item
                            if k == 0 {
                                let new_off = (frame_type as u32) - 64 + delta as u32;
                                if new_off > 63 {
                                    return Err("StackMapTable re-encoding refused".into());
                                }
                                out[q - 1] = (new_off + 64) as u8;
                            }
                            q += skip_verification_info(&out, q, 1);
                        }
                        247 => {
                            // same_locals_1_stack_item_frame_extended
                            if k == 0 {
                                let v = u16at(&out, q) + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                            q += skip_verification_info(&out, q, 1);
                        }
                        248..=250 => {
                            // chop_frame
                            if k == 0 {
                                let v = u16at(&out, q) + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                        }
                        251 => {
                            // same_frame_extended
                            if k == 0 {
                                let v = u16at(&out, q) + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                        }
                        252..=254 => {
                            // append_frame
                            if k == 0 {
                                let v = u16at(&out, q) + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                            q += skip_verification_info(&out, q, (frame_type - 251) as usize);
                        }
                        255 => {
                            // full_frame
                            if k == 0 {
                                let v = u16at(&out, q) + delta as u16;
                                w16(&mut out, q, v);
                            }
                            q += 2;
                            let nl = u16at(&out, q) as usize;
                            q += 2;
                            for _ in 0..nl {
                                q += skip_verification_info(&out, q, 1);
                            }
                            let ns = u16at(&out, q) as usize;
                            q += 2;
                            for _ in 0..ns {
                                q += skip_verification_info(&out, q, 1);
                            }
                        }
                        t => return Err(format!("bad frame type {t}")),
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

fn utf8_by_index(bytes: &[u8], cp: &CpInfo, idx: u16) -> Option<String> {
    let mut i = 10usize;
    let mut cur = 1u16;
    while cur < cp.count {
        let tag = bytes[i];
        i += 1;
        match tag {
            1 => {
                let len = u16at(bytes, i) as usize;
                if cur == idx {
                    return Some(String::from_utf8_lossy(&bytes[i + 2..i + 2 + len]).into_owned());
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

fn skip_verification_info(b: &[u8], mut q: usize, n: usize) -> usize {
    // JVMS §4.7.4 verification_type_info:
    // 0 Top, 1 Integer, 2 Float, 3 Double, 4 Long, 5 Null,
    // 6 UninitializedThis — all 1 byte (no payload);
    // 7 Object — +cpool_index u2; 8 Uninitialized — +offset u2.
    let start = q;
    for _ in 0..n {
        let tag = b[q];
        q += 1;
        match tag {
            7 | 8 => q += 2,
            0..=6 => {}
            _ => {
                // invalid tag: payload size unknown. Keep the walk aligned on
                // the assumption of no payload — only frame 0 is ever written,
                // so a corrupt later frame cannot cause a bad write.
            }
        }
    }
    q - start
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
