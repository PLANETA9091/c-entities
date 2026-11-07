# JVM hotspots (JFR ExecutionSample)

Samples: 10447

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 2.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 3 | 2.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 1.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5094` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10806` |
| 12 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5031` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 17 | 0.97% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 0.94% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.88% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7423` |
| 21 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 22 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 23 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 24 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |
| 26 | 0.77% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8215` |
| 29 | 0.70% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 30 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 31 | 0.64% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 32 | 0.64% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 34 | 0.57% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 36 | 0.51% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 37 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.44% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 39 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 40 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5340` |
| 42 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 45 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7272` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 49 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5408` |
| 51 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 53 | 0.33% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 54 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 56 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.31% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 59 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 60 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 2 | 2.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 3 | 2.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 4 | 2.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |
| 5 | 2.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 11 | 1.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7761` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 17 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 18 | 1.08% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7591` |
| 20 | 0.95% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 21 | 0.94% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.94% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 24 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3595` |
| 26 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.77% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5341` |
| 29 | 0.70% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.canUse() line: 54` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8211` |

GC pauses: 0, total 0 ms

