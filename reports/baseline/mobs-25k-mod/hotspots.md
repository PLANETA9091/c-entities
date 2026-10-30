# JVM hotspots (JFR ExecutionSample)

Samples: 14443

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 6 | 1.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 12 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.94% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.73% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3773` |
| 22 | 0.63% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.63% | `java.lang.ref.Reference.get() line: 361` |
| 25 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 26 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3710` |
| 28 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 29 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.46% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.46% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3863` |
| 37 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8689` |
| 39 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.44% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6534` |
| 41 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.37% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 47 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4018` |
| 50 | 0.35% | `java.lang.ThreadLocal.get() line: 171` |
| 51 | 0.34% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 52 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 55 | 0.32% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.29% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8567` |
| 59 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 2 | 2.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 1.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.49% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.45% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3119` |
| 8 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 10 | 1.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 12 | 1.19% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5322` |
| 14 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 23 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.75% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.74% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 29 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4019` |

GC pauses: 0, total 0 ms

