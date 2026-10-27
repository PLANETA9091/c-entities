# JVM hotspots (JFR ExecutionSample)

Samples: 14993

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 3 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.31% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 13 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 14 | 0.82% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 16 | 0.77% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 17 | 0.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 18 | 0.72% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.68% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 20 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.53% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 27 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 31 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 32 | 0.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 33 | 0.44% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 37 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6692` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 41 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.37% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7670` |
| 43 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.37% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2975` |
| 45 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1693` |
| 48 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 49 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 51 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5539` |
| 57 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 59 | 0.31% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 60 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 3 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7475` |
| 7 | 1.53% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 10 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 14 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 0.83% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 17 | 0.78% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.75% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 19 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6479` |
| 22 | 0.69% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.69% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 24 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.65% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6481` |
| 28 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3109` |

GC pauses: 0, total 0 ms

