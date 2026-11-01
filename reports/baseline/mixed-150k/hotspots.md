# JVM hotspots (JFR ExecutionSample)

Samples: 119672

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 43.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.76% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.08% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 9 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.90% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 0.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 0.62% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 16 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 17 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 0.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 23 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.41% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 25 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.39% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.39% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.29% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 37 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 41 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 42 | 0.24% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 43 | 0.24% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 44 | 0.23% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.22% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 48 | 0.20% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 51 | 0.20% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 52 | 0.19% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 54 | 0.18% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 55 | 0.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 56 | 0.18% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 57 | 0.17% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 58 | 0.17% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 59 | 0.17% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 60 | 0.17% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 43.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.08% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 10 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.63% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 15 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.45% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 20 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 22 | 0.40% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.39% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 24 | 0.36% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 25 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.36% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 29 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 30 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

