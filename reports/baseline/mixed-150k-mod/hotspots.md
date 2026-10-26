# JVM hotspots (JFR ExecutionSample)

Samples: 33623

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 5.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6584` |
| 5 | 2.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6420` |
| 11 | 1.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 1.24% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4057` |
| 17 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 19 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 20 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 1.00% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5906` |
| 22 | 0.85% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.79% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 31 | 0.59% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 32 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 35 | 0.53% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 37 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 38 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 40 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 43 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.42% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 50 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.38% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2501` |
| 53 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4476` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 56 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2026` |
| 57 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 58 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2564` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 3 | 2.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 5 | 2.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 8 | 2.20% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.15% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 11 | 1.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 12 | 1.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 13 | 1.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 15 | 1.56% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.41% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.39% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 1.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 1.18% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 23 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.78% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5005` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5893` |
| 30 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

