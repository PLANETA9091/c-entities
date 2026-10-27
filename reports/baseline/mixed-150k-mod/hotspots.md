# JVM hotspots (JFR ExecutionSample)

Samples: 37154

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.23% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 3 | 2.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 4 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 7 | 1.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 11 | 1.45% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.82% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 20 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 23 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 28 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.57% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 30 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.56% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 32 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 34 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.45% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4527` |
| 39 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 42 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 48 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 50 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 53 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 54 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 56 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 58 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.28% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 4 | 2.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.37% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 8 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.35% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 12 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.20% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 16 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.03% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 20 | 0.85% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.81% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.69% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5416` |
| 28 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

