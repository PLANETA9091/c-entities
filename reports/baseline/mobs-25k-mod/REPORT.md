# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 0.240 / 0.288 / 0.369 / 2.888 |
| MSPT mean, ms | 0.250 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 874.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 60.37% | `dev.planeta9091.centities.CeHooks.enchTickEffects$(ServerLevel, LivingEntity) line: 9335` |
| 2 | 6.69% | `java.lang.AbstractStringBuilder.ensureCapacityNewCoder(byte[], byte, int, int, byte) line: 287` |
| 3 | 2.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.tickEffects(ServerLevel, LivingEntity)` |
| 4 | 2.17% | `java.lang.StringBuilder.append(String) line: 179` |
| 5 | 1.72% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.46% | `java.util.EnumMap.get(Object) line: 248` |
| 7 | 1.14% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 8 | 0.81% | `java.lang.AbstractStringBuilder.append(String) line: 642` |
| 9 | 0.65% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 10 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.44% | `java.lang.StringUTF16.compress(byte[], int, int) line: 240` |
| 13 | 0.40% | `java.util.EnumMap.containsKey(Object) line: 223` |
| 14 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 15 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

