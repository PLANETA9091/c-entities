# JVM hotspots (JFR ExecutionSample)

Samples: 9304

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 3 | 1.89% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10301` |
| 4 | 1.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 6 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.54% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9699` |
| 12 | 1.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6988` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6413` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4509` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.99% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6920` |
| 18 | 0.91% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4446` |
| 22 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 24 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.63% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7530` |
| 28 | 0.61% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6586` |
| 29 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.60% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 31 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 32 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.52% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6764` |
| 35 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4755` |
| 41 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.42% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 43 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 45 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.39% | `java.lang.Enum.ordinal() line: 128` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 48 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.37% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 50 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 54 | 0.33% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 55 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 56 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10213` |
| 60 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 2.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7003` |
| 4 | 2.27% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 2.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 6 | 1.89% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6988` |
| 9 | 1.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4364` |
| 11 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9847` |
| 15 | 1.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 16 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 17 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 19 | 1.00% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6919` |
| 21 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4756` |
| 23 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.73% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.73% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

