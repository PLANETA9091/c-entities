# JVM hotspots (JFR ExecutionSample)

Samples: 40581

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 53.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3918` |
| 2 | 4.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3990` |
| 3 | 1.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3900` |
| 4 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 5 | 0.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3932` |
| 6 | 0.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3917` |
| 7 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3908` |
| 10 | 0.60% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 11 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10849` |
| 17 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 18 | 0.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10927` |
| 19 | 0.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.38% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10680` |
| 21 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15438` |
| 22 | 0.35% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10502` |
| 23 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 24 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6996` |
| 25 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6933` |
| 26 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.27% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 10529` |
| 28 | 0.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 29 | 0.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3901` |
| 30 | 0.24% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 32 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3950` |
| 34 | 0.22% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.22% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 36 | 0.21% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 37 | 0.21% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11472` |
| 38 | 0.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.putAll(Map) line: 226` |
| 41 | 0.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3916` |
| 42 | 0.18% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 43 | 0.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 44 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 45 | 0.16% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 46 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.16% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 48 | 0.15% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 16217` |
| 49 | 0.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.15% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.15% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.15% | `java.util.EnumMap.get(Object) line: 248` |
| 53 | 0.15% | `java.util.ImmutableCollections$SetN.probe(Object) line: 1258` |
| 54 | 0.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10241` |
| 55 | 0.14% | `java.lang.String.hashCode() line: 2430` |
| 56 | 0.13% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 57 | 0.13% | `ca.spottedleaf.concurrentutil.map.SWMRLong2ObjectHashTable.getEntryForOpaque(long) line: 178` |
| 58 | 0.13% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 59 | 0.12% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 60 | 0.12% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 63.29% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2047` |
| 2 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 3 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10942` |
| 5 | 0.90% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 6 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 8 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10927` |
| 11 | 0.56% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 12 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10848` |
| 13 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11018` |
| 14 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.48% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 16 | 0.43% | `java.util.ArrayList.grow(int) line: 238` |
| 17 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.40% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 0.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15718` |
| 20 | 0.36% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10664` |
| 21 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 23 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 25 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.26% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 28 | 0.22% | `net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal.findTarget() line: 66` |
| 29 | 0.21% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.20% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

