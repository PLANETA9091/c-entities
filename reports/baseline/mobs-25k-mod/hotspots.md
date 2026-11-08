# JVM hotspots (JFR ExecutionSample)

Samples: 15397

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 2 | 3.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 1.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10806` |
| 7 | 1.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.19% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 9 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.86% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.81% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.71% | `java.util.HashMap.getNode(Object) line: 586` |
| 16 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 17 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |
| 20 | 0.60% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 21 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5031` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7423` |
| 24 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 26 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.49% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 29 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 30 | 0.49% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 31 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 33 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 34 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 35 | 0.42% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 36 | 0.40% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 37 | 0.39% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7272` |
| 38 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 40 | 0.37% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5094` |
| 42 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 44 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8215` |
| 45 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 46 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.32% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 51 | 0.31% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5340` |
| 52 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.31% | `net.minecraft.world.scores.Scoreboard.getPlayersTeam(String) line: 310` |
| 54 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.30% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5006` |
| 56 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 57 | 0.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.29% | `net.minecraft.core.Holder$Reference.boundTags() line: 186` |
| 59 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 2 | 3.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 3 | 2.31% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 1.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 5 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 6 | 1.39% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.27% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 10 | 1.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |
| 14 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 16 | 0.95% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 18 | 0.87% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7591` |
| 21 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.76% | `java.util.HashMap.get(Object) line: 565` |
| 24 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7761` |
| 26 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.58% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7407` |

GC pauses: 0, total 0 ms

