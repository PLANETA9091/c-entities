# JVM hotspots (JFR ExecutionSample)

Samples: 32582

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9877` |
| 2 | 3.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9374` |
| 4 | 2.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6230` |
| 6 | 2.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5461` |
| 8 | 1.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.41% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 10 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 1.10% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.executeMainThreadTask() line: 329` |
| 20 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9822` |
| 24 | 0.75% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 25 | 0.74% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 26 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.64% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 34 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 36 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 38 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4387` |
| 40 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 41 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 42 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.38% | `java.lang.ref.Reference.get() line: 361` |
| 44 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2758` |
| 47 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6243` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 52 | 0.33% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7273` |
| 54 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 55 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 58 | 0.29% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 3.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9483` |
| 4 | 3.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.10% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.41% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 10 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9473` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2102` |
| 13 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.09% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.08% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.93% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 17 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.89% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 23 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.82% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 25 | 0.81% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

