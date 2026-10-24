# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54898 -> 54898) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.074 / 57.525 / 158.084 / 220.714 |
| MSPT mean, ms | 42.274 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1187.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 10.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 5.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 5.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 5 | 4.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 4.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 4.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.90% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1955` |
| 9 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2393` |
| 10 | 1.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1858` |
| 11 | 1.08% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 12 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.70% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |

