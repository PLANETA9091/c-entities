# JVM hotspots (JFR ExecutionSample)

Samples: 89038

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 102` |
| 2 | 5.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 3 | 2.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 4 | 2.16% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 12 | 1.21% | `java.lang.Enum.ordinal() line: 128` |
| 13 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 1.14% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 1.12% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 16 | 1.09% | `java.util.HashMap.getNode(Object) line: 585` |
| 17 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 1.04% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 19 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 21 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 23 | 0.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 24 | 0.85% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 25 | 0.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 26 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 27 | 0.77% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 28 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 30 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 31 | 0.67% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 32 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 33 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 34 | 0.65% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 35 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 36 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 38 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 39 | 0.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 40 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 44 | 0.46% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 45 | 0.44% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.getRootVehicle() line: 4556` |
| 48 | 0.41% | `org.bukkit.craftbukkit.entity.CraftPlayer.canSee(Entity) line: 2217` |
| 49 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 50 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.push(Entity) line: 2245` |
| 52 | 0.38% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 53 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 54 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 55 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 56 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 58 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 59 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 2 | 6.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 37` |
| 3 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 2.18% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 6 | 2.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.53% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 8 | 1.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 9 | 1.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.15% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 14 | 1.14% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 15 | 1.12% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 16 | 1.09% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.99% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 21 | 0.86% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 25 | 0.80% | `net.minecraft.world.entity.EntitySelector.lambda$pushable$9(boolean, Entity, Team, Team$CollisionRule, Entity) line: 58` |
| 26 | 0.78% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 30 | 0.66% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |

GC pauses: 0, total 0 ms

