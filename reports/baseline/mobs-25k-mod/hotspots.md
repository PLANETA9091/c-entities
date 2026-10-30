# JVM hotspots (JFR ExecutionSample)

Samples: 14748

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.96% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 8 | 1.46% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 9 | 1.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 10 | 1.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.25% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 13 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.14% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 21 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 22 | 0.83% | `java.lang.ref.Reference.get() line: 361` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.60% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3988` |
| 30 | 0.57% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6749` |
| 31 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 33 | 0.55% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.52% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 35 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 37 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3925` |
| 42 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 44 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 45 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 46 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.41% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 50 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 52 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4078` |
| 58 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.32% | `java.lang.ThreadLocal.get() line: 171` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 1.95% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 2 | 1.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8709` |
| 4 | 1.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.69% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3308` |
| 7 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 9 | 1.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.39% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 12 | 1.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 16 | 1.14% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 1.05% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 20 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5537` |
| 24 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 26 | 0.83% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 27 | 0.82% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4098` |
| 28 | 0.76% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 29 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.73% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

