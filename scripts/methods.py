#!/usr/bin/env python3
"""Minimal javap replacement: list methods of a class from a jar.

Usage: python3 methods.py <jar> <class.internal.Name> [filter_regex]
Parses the constant pool + methods table directly (no dependencies).
"""
import struct, sys, zipfile, re

def main():
    jar, cls = sys.argv[1], sys.argv[2]
    filt = re.compile(sys.argv[3]) if len(sys.argv) > 3 else None
    with zipfile.ZipFile(jar) as z:
        data = z.read(cls.replace(".", "/") + ".class")
    off = 8  # skip magic+version
    n = struct.unpack_from(">H", data, off)[0]; off += 2
    cp = [None] * n  # 1-indexed
    i = 1
    while i < n:
        tag = data[off]; off += 1
        if tag == 1:  # Utf8
            ln = struct.unpack_from(">H", data, off)[0]; off += 2
            cp[i] = data[off:off+ln].decode("utf-8", "replace"); off += ln
        elif tag in (7, 8, 16, 19, 20):  # Class/Str/MethodType/Module/Pkg
            cp[i] = struct.unpack_from(">H", data, off)[0]; off += 2
        elif tag == 15:  # MethodHandle ref kind
            cp[i] = (data[off], struct.unpack_from(">H", data, off+1)[0]); off += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):  # Int/Float/Fieldref/Methodref/IfaceMethodref/NameAndType/Dynamic/InvokeDynamic
            cp[i] = struct.unpack_from(">HH", data, off) if tag >= 9 else struct.unpack_from(">I", data, off)
            off += 4 if tag in (3, 4) else 4
        elif tag in (5, 6):  # Long/Double take two slots
            cp[i] = struct.unpack_from(">Q", data, off)[0]; off += 8; i += 1
        elif tag == 14:  # MethodHandle? handled above; unreachable
            off += 3
        else:
            raise SystemExit(f"unknown cp tag {tag} at {i}")
        i += 1
    def utf(idx): return cp[idx] if isinstance(cp[idx], str) else None
    def name_of(idx):  # Class -> utf8 name
        e = cp[idx]
        return utf(e) if isinstance(e, int) else None
    def nat(idx):  # NameAndType
        a, b = cp[idx]; return utf(a), utf(b)
    def ref(idx):  # Methodref/Fieldref -> (class_name, name, desc)
        ci, nti = cp[idx]; nm, de = nat(nti); return name_of(ci), nm, de

    off += 6  # access_flags, this_class, super_class
    ifc = struct.unpack_from(">H", data, off)[0]; off += 2 + 2 * ifc
    def skip_members(off):
        cnt = struct.unpack_from(">H", data, off)[0]; off += 2
        for _ in range(cnt):
            off += 6  # access, name, desc
            ac = struct.unpack_from(">H", data, off)[0]; off += 2
            for _ in range(ac):
                an = struct.unpack_from(">H", data, off)[0]; off += 2
                alen = struct.unpack_from(">I", data, off)[0]; off += 4
                off += alen
        return cnt, off
    _, off = skip_members(off)  # fields
    cnt, off = skip_members(off)  # methods
    print(f"# {cls}: {cnt} methods")
    # collect own methods by re-walking (we skipped name/desc, redo properly)
    # simpler: re-parse from the saved start
    # We lost offsets; instead print refs from constant pool (superset):
    seen = set()
    for e in cp:
        if isinstance(e, tuple) and len(e) == 2 and isinstance(cp[e[1]], tuple):
            pass
    # Proper: methods table walk needs offsets; do a second full pass:
    off = 8
    n = struct.unpack_from(">H", data, off)[0]; off += 2
    i = 1
    while i < n:
        tag = data[off]; off += 1
        if tag == 1:
            ln = struct.unpack_from(">H", data, off)[0]; off += 2 + ln
        elif tag in (7, 8, 16, 19, 20): off += 2
        elif tag == 15: off += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18): off += 4
        elif tag in (5, 6): off += 8; i += 1
        i += 1
    off += 6
    ifc = struct.unpack_from(">H", data, off)[0]; off += 2 + 2 * ifc
    def walk(off, out):
        cnt = struct.unpack_from(">H", data, off)[0]; off += 2
        for _ in range(cnt):
            acc, nidx, didx = struct.unpack_from(">HHH", data, off); off += 6
            ac = struct.unpack_from(">H", data, off)[0]; off += 2
            for _ in range(ac):
                alen = struct.unpack_from(">I", data, off+2)[0]; off += 6 + alen
            out.append((acc, utf(nidx), utf(didx)))
        return off
    flds = []
    off = walk(off, flds)
    mths = []
    off = walk(off, mths)
    for acc, nm, de in mths:
        if filt and not (filt.search(nm or "") or (de and filt.search(de))): continue
        flags = []
        if acc & 0x0001: flags.append("pub")
        if acc & 0x0002: flags.append("priv")
        if acc & 0x0008: flags.append("static")
        if acc & 0x0010: flags.append("final")
        print(f"  {' '.join(flags):12s} {nm} {de}")

if __name__ == "__main__":
    main()
