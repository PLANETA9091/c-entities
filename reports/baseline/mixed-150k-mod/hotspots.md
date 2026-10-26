# JVM hotspots (JFR ExecutionSample)

Samples: 37181

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3860` |
| 6 | 2.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.11% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5687` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4024` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 10 | 1.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 14 | 1.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.33% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5550` |
| 16 | 1.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 18 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 1.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3888` |
| 20 | 1.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4364` |
| 21 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 1.06% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 23 | 1.01% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 33 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 34 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 36 | 0.56% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 37 | 0.54% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 38 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.53% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.47% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 46 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 47 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 49 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2552` |
| 50 | 0.42% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 51 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 54 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 55 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 57 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 60 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 3.73% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 4 | 3.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.18% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.11% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 2.08% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 10 | 1.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 11 | 1.80% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4026` |
| 13 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 15 | 1.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 16 | 1.32% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 18 | 1.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 1.21% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 1.17% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 1.04% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 1.00% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 24 | 0.94% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4364` |
| 28 | 0.83% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 29 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4406` |

GC pauses: 0, total 0 ms

