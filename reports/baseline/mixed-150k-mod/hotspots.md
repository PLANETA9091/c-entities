# JVM hotspots (JFR ExecutionSample)

Samples: 31854

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 2.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 4 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7525` |
| 6 | 2.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7666` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4440` |
| 9 | 2.00% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4585` |
| 11 | 1.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.49% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 15 | 1.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 1.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1759` |
| 17 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 1.11% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 19 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 25 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 28 | 0.68% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.66% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 31 | 0.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7611` |
| 33 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 37 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 38 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 39 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2444` |
| 40 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 41 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7486` |
| 42 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 43 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.44% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 46 | 0.42% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2928` |
| 49 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2865` |
| 51 | 0.40% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2390` |
| 53 | 0.39% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 55 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 58 | 0.35% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6559` |
| 60 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 3 | 2.81% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 4 | 2.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7521` |
| 5 | 2.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.46% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.45% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.10% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4587` |
| 13 | 1.49% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 14 | 1.47% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 15 | 1.41% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 16 | 1.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.17% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 1.14% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.95% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4440` |
| 25 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1763` |
| 28 | 0.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7513` |
| 29 | 0.74% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5479` |
| 30 | 0.66% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |

GC pauses: 0, total 0 ms

