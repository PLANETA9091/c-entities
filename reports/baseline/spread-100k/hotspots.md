# JVM hotspots (JFR ExecutionSample)

Samples: 73095

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 0.94% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.85% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 17 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 21 | 0.63% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.58% | `java.util.HashMap.getNode(Object) line: 586` |
| 23 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 25 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.48% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.40% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 39 | 0.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 40 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 42 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.35% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 45 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 47 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 48 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 49 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.29% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 56 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 59 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 60 | 0.27% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 13 | 0.91% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.72% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 111` |
| 17 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.63% | `java.util.ArrayList.toArray() line: 370` |
| 19 | 0.61% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 25 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.50% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 28 | 0.50% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 30 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |

GC pauses: 0, total 0 ms

