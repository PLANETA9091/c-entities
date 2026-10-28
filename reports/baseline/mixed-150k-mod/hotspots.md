# JVM hotspots (JFR ExecutionSample)

Samples: 47806

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.88% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 5 | 2.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.58% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 10 | 1.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 12 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.23% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 17 | 1.03% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 18 | 1.00% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 19 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.55% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 30 | 0.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 31 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 35 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 36 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 42 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 45 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 46 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 48 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 51 | 0.32% | `java.lang.ref.Reference.get() line: 361` |
| 52 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 55 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.29% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7867` |
| 57 | 0.29% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 2.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.97% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 11 | 1.60% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.34% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.24% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 1.03% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.01% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 19 | 0.94% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 24 | 0.75% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.59% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |

GC pauses: 0, total 0 ms

