#!/usr/bin/env python3
"""pos_census.py — the COMPLETE position-mutator census for the NM applier
position mirror (the B27 sit_census.py pattern, applied to Entity.position).

Question the mirror's license depends on: does EVERY write of the entity
position field funnel through Entity.setPosRaw(double,double,double,boolean)?
If any code puts Entity.position directly (PUTFIELD on the field ref) the
census is RED and the mirror stays parked (a foreign write would go
unprobed — a stale mirror = a behavioral change).

Scan: every class entry of the mojang-mapped jar; collect field refs to
Entity.position (name+descriptor) and every CALL to setPosRaw/setPos/
absSnapTo*/moveTo (the known wrapper family). Then javap -c the Entity
class itself to list the position writers inside Entity, and the callers
of setPosRaw across the jar (the probe point must dominate them all).
"""
import sys, zipfile, struct

JAR = sys.argv[1] if len(sys.argv) > 1 else "/home/z/my-project/work/paper/mojang_26.3.jar"

# The field we census: net.minecraft.world.entity.Entity.position (Vec3).
# raw constant-pool walk: find CONSTANT_Fieldref whose class is Entity and
# name/descriptor matches position / Lnet/minecraft/world/phys/Vec3;
TARGET_CLASS = b"net/minecraft/world/entity/Entity"
FIELD_NAME = b"position"
FIELD_DESC = b"Lnet/minecraft/world/phys/Vec3;"

# method-name families of interest (call-site census)
CALL_NAMES = {b"setPosRaw", b"setPos", b"absSnapTo", b"absSnapMovementTo",
              b"absMoveTo", b"moveTo", b"reapplyPosition", b"setPosRawOverride"}

def u1(b, o): return b[o]
def u2(b, o): return struct.unpack(">H", b[o:o+2])[0]

def parse_cp(data):
    """returns (count, list of (tag, bytes-for-string-fields)) minimal"""
    n = u2(data, 8)
    i, cp = 10, [None] * n
    idx = 1
    while idx < n:
        tag = data[i]
        if tag == 1:  # Utf8
            ln = u2(data, i + 1)
            cp[idx] = (1, data[i + 3:i + 3 + ln])
            i += 3 + ln
        elif tag in (7, 8, 16, 19, 20):  # Class/Str/MethodType/Module/Pkg (u2 payload)
            cp[idx] = (tag, u2(data, i + 1)); i += 3
        elif tag == 15:  # MethodHandle
            cp[idx] = (tag, None); i += 4
        elif tag in (3, 4):  # int/float
            cp[idx] = (tag, None); i += 5
        elif tag in (5, 6):  # long/double (two slots)
            cp[idx] = (tag, None); idx += 1; i += 9
        elif tag in (9, 10, 11, 12, 17, 18):  # Fieldref/Methodref/Iface/Name+Type/Dynamic/InvokeDynamic
            cp[idx] = (tag, struct.unpack(">HH", data[i+1:i+5])); i += 5
        else:
            raise ValueError(f"tag {tag} @ {i}")
        idx += 1
    return cp

def resolve_utf(cp, i):
    t = cp[i]
    if t and t[0] == 1: return t[1]
    if t and t[0] in (9, 10, 11, 12, 17, 18):
        nt = cp[t[1][1]]
        return nt[1] if nt and nt[0] == 1 else None
    return None

def walk_fields_calls(data):
    """returns (writes_of_position_count, calls dict name->count) by scanning
    the method bytecode for field/method refs — a coarse but COMPLETE
    constant-pool scan (refs may be uncalled — we over-count, never under)."""
    cp = parse_cp(data)
    n = u2(data, 8)
    writes, calls = 0, {}
    # every CONSTANT_Fieldref: class/name/desc
    for k in range(1, n):
        e = cp[k]
        if not e: continue
        if e[0] == 9:  # Fieldref
            cls_idx, nt_idx = e[1]
            cls_e = cp[cls_idx]
            if cls_e and cls_e[0] == 7 and resolve_utf(cp, cls_e[1]) == TARGET_CLASS:
                # name/desc via the NameAndType entry
                nt_e = cp[nt_idx]
                if nt_e and nt_e[0] == 12:
                    name = resolve_utf(cp, nt_e[1][0]); desc = resolve_utf(cp, nt_e[1][1])
                    if name == FIELD_NAME and desc == FIELD_DESC:
                        writes += 1
        elif e[0] in (10, 11):  # Methodref / InterfaceMethodref
            cls_idx, nt_idx = e[1]
            cls_e = cp[cls_idx]
            if cls_e and cls_e[0] == 7:
                nt_e = cp[nt_idx]
                if nt_e and nt_e[0] == 12:
                    name = resolve_utf(cp, nt_e[1][0])
                    if name in CALL_NAMES:
                        calls[name] = calls.get(name, 0) + 1
    return writes, calls

zf = zipfile.ZipFile(JAR)
names = [n for n in zf.namelist() if n.endswith(".class")]
print(f"classes: {len(names)}")

field_ref_sites = {}   # class -> fieldref count for Entity.position
call_sites = {}        # call name -> {class: count}
for cn in names:
    data = zf.read(cn)
    try:
        w, c = walk_fields_calls(data)
    except Exception as ex:
        print(f"WARN parse {cn}: {ex}"); continue
    if w:
        field_ref_sites[cn] = field_ref_sites.get(cn, 0) + w
    for k, v in c.items():
        call_sites.setdefault(k, {})
        call_sites[k][cn] = call_sites[k].get(cn, 0) + v

print("== Entity.position FIELDREF sites (name+desc exact) ==")
for cn, w in sorted(field_ref_sites.items()):
    print(f"  {w:3d}  {cn}")
print(f"  TOTAL fieldref entries: {sum(field_ref_sites.values())}")

print("== position-writer call-family refs per class ==")
for k, m in sorted(call_sites.items()):
    total = sum(m.values())
    print(f"  {k.decode()}: {total} refs in {len(m)} classes")
