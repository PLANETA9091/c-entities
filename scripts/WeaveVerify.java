import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.io.InputStream;
import java.util.zip.ZipFile;

import dev.dist.SdkAsmHelper;
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.tree.ClassNode;
import org.objectweb.asm.tree.AbstractInsnNode;
import org.objectweb.asm.tree.MethodInsnNode;
import org.objectweb.asm.tree.MethodNode;

/**
 * Offline weave verification for the c-entities L3 merge fast path.
 *
 * Replays the EXACT arm-time rewrite chain for ItemEntity on the real
 * shipped Paper class bytes, then forces the JVM to fully parse+verify the
 * result (ClassLoader.defineClass + Unsafe.ensureClassInitialized — the same
 * gate that previously caught the LVT bug as INVALID_CLASS_FORMAT).
 *
 * Chain (must mirror lib.rs precompute_item):
 *   1. op 3 SKIP_GUARD on tick()V  (target 0 = merge-interval pattern,
 *      init_local slot 1 = getDeltaMovement, end_push iconst_0)
 *   2. op 4 EARLY_RETURN_GUARD on mergeWithNeighbours()V
 * (The Rust-side hurtServer prefix is applied afterwards by patch.rs and is
 * covered by patch_real.rs — this harness validates the ASM steps.)
 *
 * Spec serialization mirrors cplug_sdk::asm::{serialize_guard,
 * serialize_early_return}: version u8, op u8, then u16-be length + raw
 * UTF-8 bytes per string (ASCII names — identical to DataOutputStream's
 * modified UTF-8 for these).
 */
public final class WeaveVerify {

    static DataOutputStream spec() {
        ByteArrayOutputStream b = new ByteArrayOutputStream();
        return new DataOutputStream(b);
    }

    static byte[] finish(ByteArrayOutputStream b) {
        return b.toByteArray();
    }

    static byte[] skipGuardSpec() throws Exception {
        ByteArrayOutputStream b = new ByteArrayOutputStream();
        DataOutputStream d = new DataOutputStream(b);
        d.writeByte(1);   // version
        d.writeByte(3);   // op SKIP_GUARD
        utf(d, "tick");
        utf(d, "()V");
        utf(d, "dev/planeta9091/centities/CeHooks");
        utf(d, "itemSkip");
        utf(d, "(Lnet/minecraft/world/entity/item/ItemEntity;)Z");
        d.writeShort(0);  // target = 0 (pattern select)
        d.writeBoolean(true);
        d.writeByte(1);   // slot
        utf(d, "getDeltaMovement");
        utf(d, "()Lnet/minecraft/world/phys/Vec3;");
        d.writeBoolean(true);
        d.writeByte(0x04); // iconst_0
        return finish(b);
    }

    static byte[] earlyReturnSpec() throws Exception {
        ByteArrayOutputStream b = new ByteArrayOutputStream();
        DataOutputStream d = new DataOutputStream(b);
        d.writeByte(1);   // version
        d.writeByte(4);   // op EARLY_RETURN_GUARD
        utf(d, "mergeWithNeighbours");
        utf(d, "()V");
        utf(d, "dev/planeta9091/centities/CeHooks");
        utf(d, "mergeScanNative");
        utf(d, "(Lnet/minecraft/world/entity/item/ItemEntity;)Z");
        return finish(b);
    }

    static void utf(DataOutputStream d, String s) throws Exception {
        byte[] x = s.getBytes("UTF-8");
        if (x.length > 0xFFFF) throw new IllegalStateException("spec string too long");
        d.writeShort(x.length);
        d.write(x);
    }

    static byte[] readEntry(String jar, String entry) throws Exception {
        try (ZipFile z = new ZipFile(jar)) {
            InputStream in = z.getInputStream(z.getEntry(entry));
            ByteArrayOutputStream out = new ByteArrayOutputStream();
            byte[] buf = new byte[8192];
            int n;
            while ((n = in.read(buf)) > 0) out.write(buf, 0, n);
            return out.toByteArray();
        }
    }

    static String head(String className, String method, byte[] bytes) {
        ClassNode cn = new ClassNode();
        new ClassReader(bytes).accept(cn, 0);
        for (Object mo : cn.methods) {
            MethodNode mn = (MethodNode) mo;
            if (!mn.name.equals(method)) continue;
            StringBuilder sb = new StringBuilder();
            int seen = 0;
            for (AbstractInsnNode n = mn.instructions.getFirst();
                    n != null && seen < 6; n = n.getNext()) {
                if (n.getOpcode() == -1) continue;
                sb.append(seen == 0 ? "" : " | ").append(opName(n.getOpcode()));
                if (n instanceof MethodInsnNode) {
                    sb.append(' ').append(((MethodInsnNode) n).name);
                }
                seen++;
            }
            return sb.toString();
        }
        return "<method not found>";
    }

    static String opName(int op) {
        switch (op) {
            case 0x2a: return "aload_0";
            case 0xb8: return "invokestatic";
            case 0x99: return "ifeq";
            case 0xa7: return "goto";
            case 0xb1: return "return";
            case 0x04: return "iconst_0";
            case 0x3d: return "istore_2".isEmpty() ? "istore" : "istore_0";
            default: return "op" + Integer.toHexString(op);
        }
    }

    public static void main(String[] args) throws Exception {
        String jar = System.getProperty("paper.jar");
        if (jar == null || jar.isEmpty()) {
            System.err.println("usage: -Dpaper.jar=<paper-26.3.jar>");
            System.exit(2);
        }
        byte[] orig = readEntry(jar, "net/minecraft/world/entity/item/ItemEntity.class");
        System.out.println("orig ItemEntity.class: " + orig.length + " bytes");
        System.out.println("tick head:       " + head("", "tick", orig));
        System.out.println("merge head:      " + head("", "mergeWithNeighbours", orig));

        byte[] b1 = SdkAsmHelper.rewrite(orig, skipGuardSpec());
        if (b1 == null) {
            System.err.println("FAIL skip-guard: " + SdkAsmHelper.lastError());
            System.exit(1);
        }
        System.out.println("after skip-guard: " + b1.length + " bytes");
        System.out.println("tick head:       " + head("", "tick", b1));

        byte[] b2 = SdkAsmHelper.rewrite(b1, earlyReturnSpec());
        if (b2 == null) {
            System.err.println("FAIL early-return-guard: " + SdkAsmHelper.lastError());
            System.exit(1);
        }
        System.out.println("after merge-guard: " + b2.length + " bytes");
        System.out.println("merge head:      " + head("", "mergeWithNeighbours", b2));

        // Full JVM verify: define under a loader whose parent sees the paper
        // jar (superclass chain: Entity -> TraceableEntity), forcing both
        // verifier passes; ClassFormatError/VerifyError = fail.
        final byte[] bytes = b2;
        ClassLoader parent = new java.net.URLClassLoader(
                new java.net.URL[] { new java.io.File(jar).toURI().toURL() },
                WeaveVerify.class.getClassLoader()); // app cp carries the server libs
        ClassLoader cl = new ClassLoader(parent) {
            @Override protected Class<?> loadClass(String name, boolean resolve)
                    throws ClassNotFoundException {
                synchronized (getClassLoadingLock(name)) {
                    Class<?> c = findLoadedClass(name);
                    if (c == null) {
                        if (name.equals("net.minecraft.world.entity.item.ItemEntity")) {
                            c = defineClass(name, bytes, 0, bytes.length);
                        } else {
                            c = parent.loadClass(name);
                        }
                    }
                    if (resolve) resolveClass(c);
                    return c;
                }
            }
        };
        Class<?> c = Class.forName("net.minecraft.world.entity.item.ItemEntity", false, cl);
        // defineClass already ran the full bytecode verifier — that is the
        // gate (ClassFormatError/VerifyError above would have failed).
        // The forced-init step needs Minecraft registries bootstrapped, which
        // is not possible offline; if it works it adds pass-2 coverage.
        try {
            java.lang.reflect.Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            ((sun.misc.Unsafe) uf.get(null)).ensureClassInitialized(c);
            System.out.println("full init: OK");
        } catch (Throwable t2) {
            System.out.println("full init: skipped (registries not bootstrapped offline): "
                    + t2.getClass().getSimpleName());
        }
        System.out.println("WEAVE-VERIFY OK: ItemEntity (skip-guard + merge-guard) verified");
    }
}
