# JVM hotspots (JFR ExecutionSample)

Samples: 61697

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.90% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 4 | 2.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 2.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 8 | 2.15% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.84% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 12 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 14 | 1.33% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 15 | 1.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 16 | 1.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 17 | 1.08% | `java.lang.ThreadLocal.get() line: 172` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 19 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 22 | 0.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 24 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 25 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 26 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 27 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 28 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 31 | 0.67% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 32 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 33 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 34 | 0.64% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 35 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 36 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.59% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 38 | 0.58% | `java.lang.Enum.ordinal() line: 128` |
| 39 | 0.56% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 40 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 42 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 43 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 44 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 45 | 0.52% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 46 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 47 | 0.50% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |
| 48 | 0.50% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 49 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 50 | 0.46% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 51 | 0.43% | `java.util.ArrayDeque.size() line: 648` |
| 52 | 0.42% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 53 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 54 | 0.40% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 56 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 57 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 58 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 59 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 60 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 2.91% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 6 | 2.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 2.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 2.42% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 9 | 2.16% | `java.util.HashMap.containsKey(Object) line: 602` |
| 10 | 2.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.84% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 12 | 1.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 13 | 1.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 1.34% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 15 | 1.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 16 | 1.27% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 17 | 1.12% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 18 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 19 | 1.02% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 21 | 1.00% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 23 | 0.93% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 24 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1423` |
| 25 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 26 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 27 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 28 | 0.72% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 30 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |

GC pauses: 0, total 0 ms

