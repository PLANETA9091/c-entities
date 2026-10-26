# JVM hotspots (JFR ExecutionSample)

Samples: 27327

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6703` |
| 4 | 3.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 5 | 2.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 6 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 10 | 1.08% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6539` |
| 11 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.94% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 17 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 18 | 0.88% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2110` |
| 26 | 0.77% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4726` |
| 28 | 0.74% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 29 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.65% | `java.lang.String.hashCode() line: 2433` |
| 34 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 35 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.61% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.61% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 41 | 0.58% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 311` |
| 42 | 0.56% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 43 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 44 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 45 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4208` |
| 47 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4669` |
| 48 | 0.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 49 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 53 | 0.38% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4370` |
| 54 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 55 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.40% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 3.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 8 | 2.21% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4736` |
| 11 | 1.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4008` |
| 15 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.19% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 1.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 1.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 20 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.99% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.94% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 28 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

GC pauses: 0, total 0 ms

