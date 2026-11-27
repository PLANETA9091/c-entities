# JVM hotspots (JFR ExecutionSample)

Samples: 70244

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.17% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.96% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 16 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.88% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.68% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 24 | 0.56% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 25 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 26 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 28 | 0.53% | `java.util.HashMap.getNode(Object) line: 586` |
| 29 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.49% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 32 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.49% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 34 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 35 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 36 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 46 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 47 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 49 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 52 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 56 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 57 | 0.29% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 59 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 2.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 7 | 1.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.82% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 111` |
| 16 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.70% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.69% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 20 | 0.67% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.61% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 26 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 27 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.54% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.51% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

GC pauses: 0, total 0 ms

