# JVM hotspots (JFR ExecutionSample)

Samples: 11718

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.07% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 10 | 1.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.41% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 15 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 19 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 21 | 0.91% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 23 | 0.84% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.77% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4017` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.69% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 31 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.56% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 34 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 35 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.51% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 38 | 0.49% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 39 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 42 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.38% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8996` |
| 45 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4170` |
| 51 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 57 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 59 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 2 | 2.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 4 | 1.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.74% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3376` |
| 8 | 1.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 10 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.55% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 1.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 18 | 1.12% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 19 | 1.11% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 22 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.99% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 24 | 0.97% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 25 | 0.91% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4190` |
| 26 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 29 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.74% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms

