# JVM hotspots (JFR ExecutionSample)

Samples: 15425

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 7 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 9 | 1.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 14 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.00% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 16 | 0.98% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 18 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.78% | `java.lang.ref.Reference.get() line: 361` |
| 20 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 23 | 0.66% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6295` |
| 25 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3601` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3691` |
| 27 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 30 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3538` |
| 31 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 33 | 0.49% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 34 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 35 | 0.47% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 36 | 0.46% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 38 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8328` |
| 40 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.37% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3846` |
| 44 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 48 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 49 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 52 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8450` |
| 55 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 56 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 58 | 0.30% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 59 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 1.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 4 | 1.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8255` |
| 5 | 1.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 8 | 1.41% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 10 | 1.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8230` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 1.13% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 20 | 1.09% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 0.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 22 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5083` |
| 23 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 24 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.89% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 28 | 0.78% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3711` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.73% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms

