# JVM hotspots (JFR ExecutionSample)

Samples: 35519

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.62% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 3 | 3.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 4 | 3.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 5 | 3.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 8 | 2.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 10 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5365` |
| 12 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 16 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.80% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 27 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8560` |
| 29 | 0.55% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 30 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 33 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8435` |
| 35 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 39 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 40 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.38% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 42 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3420` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 45 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 50 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5515` |
| 51 | 0.34% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 52 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 53 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.32% | `java.util.EnumMap.get(Object) line: 248` |
| 55 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.30% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 57 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.93% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8470` |
| 7 | 2.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4820` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8462` |
| 13 | 1.44% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.41% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 18 | 1.15% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 1.02% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

