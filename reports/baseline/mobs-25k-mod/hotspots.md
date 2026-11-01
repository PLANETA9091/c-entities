# JVM hotspots (JFR ExecutionSample)

Samples: 13833

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 7 | 1.48% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 8 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 10 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 13 | 1.17% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 18 | 0.92% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 19 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.82% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.77% | `java.lang.ref.Reference.get() line: 361` |
| 23 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 24 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 25 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4053` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6889` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 30 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 31 | 0.56% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 32 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 34 | 0.52% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 36 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9044` |
| 39 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 40 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.46% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 45 | 0.42% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 46 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4206` |
| 49 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 55 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 58 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.12% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 3 | 1.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8849` |
| 4 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.50% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 9 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.26% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 16 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.16% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 18 | 1.12% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 19 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 1.06% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 1.04% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3412` |
| 22 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 23 | 0.94% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.85% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 26 | 0.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 27 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5677` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4226` |
| 30 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |

GC pauses: 0, total 0 ms

