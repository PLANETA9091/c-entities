# JVM hotspots (JFR ExecutionSample)

Samples: 15607

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 7 | 1.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 8 | 1.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 16 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.79% | `java.lang.ref.Reference.get() line: 361` |
| 19 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 26 | 0.63% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 28 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 31 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.50% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9044` |
| 33 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4053` |
| 36 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.46% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4206` |
| 38 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6889` |
| 41 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.41% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 43 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 44 | 0.40% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 46 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4361` |
| 48 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3411` |
| 55 | 0.31% | `java.lang.ThreadLocal.get() line: 171` |
| 56 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4028` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8849` |
| 3 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 1.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 6 | 1.60% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 8 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 11 | 1.31% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 13 | 1.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3412` |
| 16 | 1.10% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5677` |
| 19 | 0.95% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 20 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.84% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4226` |
| 27 | 0.78% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 30 | 0.68% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |

GC pauses: 0, total 0 ms

