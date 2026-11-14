# JVM hotspots (JFR ExecutionSample)

Samples: 13460

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.55% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 2.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3074` |
| 3 | 1.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 1.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 7 | 1.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 10 | 1.02% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5522` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8304` |
| 12 | 1.00% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 13 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 16 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8135` |
| 18 | 0.82% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7957` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8382` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3075` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5459` |
| 23 | 0.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.56% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8927` |
| 27 | 0.56% | `java.util.ArrayList.elementData(int) line: 412` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2838` |
| 30 | 0.52% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 31 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 34 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 35 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 38 | 0.40% | `java.util.ArrayList.add(Object) line: 497` |
| 39 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 40 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12852` |
| 43 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5768` |
| 44 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.35% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7984` |
| 46 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 47 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 48 | 0.33% | `java.lang.String.hashCode() line: 2430` |
| 49 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3057` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 52 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 54 | 0.29% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 55 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 60 | 0.26% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.13% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 4.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2838` |
| 3 | 2.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 2.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8397` |
| 5 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8382` |
| 12 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 13 | 1.14% | `java.util.ArrayList.add(Object) line: 497` |
| 14 | 1.11% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8303` |
| 16 | 1.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8473` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5769` |
| 22 | 0.70% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 24 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.56% | `java.util.ArrayList.get(int) line: 429` |
| 27 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 29 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.52% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.canUse() line: 54` |

GC pauses: 0, total 0 ms

