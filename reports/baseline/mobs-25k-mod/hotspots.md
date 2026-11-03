# JVM hotspots (JFR ExecutionSample)

Samples: 9207

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 3 | 1.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 1.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 6 | 1.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 10 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 14 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4506` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 18 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 19 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.83% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 25 | 0.78% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 26 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 28 | 0.68% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.66% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4815` |
| 31 | 0.65% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 32 | 0.63% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.59% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7590` |
| 36 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 39 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 40 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 47 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 48 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10454` |
| 50 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 52 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 54 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 57 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 59 | 0.33% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 2.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 3 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.15% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 2.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 6 | 1.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 7 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 11 | 1.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 13 | 1.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 16 | 1.16% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6979` |
| 18 | 1.03% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 21 | 0.96% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.91% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.89% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4816` |
| 25 | 0.81% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 29 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms

