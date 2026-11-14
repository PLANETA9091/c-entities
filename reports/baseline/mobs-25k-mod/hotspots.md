# JVM hotspots (JFR ExecutionSample)

Samples: 21814

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.99% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 3.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3125` |
| 3 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3126` |
| 5 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.06% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 9 | 1.01% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 0.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 11 | 0.98% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 12 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.87% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 15 | 0.87% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 16 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8768` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3021` |
| 18 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 19 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12892` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8846` |
| 23 | 0.66% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.65% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8599` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8421` |
| 27 | 0.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 28 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.55% | `java.util.ArrayList.elementData(int) line: 412` |
| 30 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 33 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 35 | 0.39% | `java.util.ArrayList.add(Object) line: 497` |
| 36 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 37 | 0.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 38 | 0.35% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8448` |
| 39 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.34% | `net.minecraft.world.entity.ai.control.LookControl.setLookAt(double, double, double) line: 35` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9391` |
| 42 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 43 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5822` |
| 44 | 0.30% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 45 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 46 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 47 | 0.30% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 48 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3108` |
| 50 | 0.28% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5885` |
| 51 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.28% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1392` |
| 53 | 0.28% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6131` |
| 54 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.27% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.27% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 59 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.88% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 6.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 3 | 1.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 1.81% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8861` |
| 6 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.49% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 0.99% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 13 | 0.98% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8846` |
| 15 | 0.92% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 16 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13134` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8767` |
| 19 | 0.87% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8937` |
| 21 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.72% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.67% | `java.util.HashMap.get(Object) line: 565` |
| 26 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3194` |
| 30 | 0.55% | `java.util.ArrayList.get(int) line: 429` |

GC pauses: 0, total 0 ms

