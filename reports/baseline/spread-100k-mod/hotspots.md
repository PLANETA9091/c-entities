# JVM hotspots (JFR ExecutionSample)

Samples: 39767

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8832` |
| 2 | 3.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 4 | 2.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5727` |
| 5 | 2.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 10 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4936` |
| 11 | 1.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.11% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5582` |
| 15 | 1.07% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 16 | 1.07% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.98% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.96% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 0.94% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 22 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.79% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.78% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 27 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 29 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 31 | 0.67% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 32 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 33 | 0.62% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.61% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.59% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8777` |
| 40 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8652` |
| 44 | 0.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 45 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 46 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 49 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.37% | `java.lang.ref.Reference.get() line: 361` |
| 51 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 52 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 56 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3983` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 60 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 2 | 4.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 7 | 2.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.82% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8704` |
| 10 | 1.52% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.30% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8679` |
| 15 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.07% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5037` |
| 18 | 1.04% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2594` |
| 20 | 0.92% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 21 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 22 | 0.81% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 26 | 0.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 27 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

