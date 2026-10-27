# JVM hotspots (JFR ExecutionSample)

Samples: 41256

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.27% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 3 | 3.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 4 | 3.06% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 2.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 6 | 2.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 9 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.75% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 12 | 1.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 15 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.11% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 22 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 26 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.70% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 28 | 0.64% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 31 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 34 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 36 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 38 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.47% | `java.lang.ref.Reference.get() line: 361` |
| 40 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.41% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 46 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 49 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.39% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4527` |
| 52 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.37% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 54 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2327` |
| 57 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 3 | 3.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 8 | 2.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 9 | 1.83% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.78% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.42% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 0.97% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.83% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.71% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1431` |
| 30 | 0.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

