# JVM hotspots (JFR ExecutionSample)

Samples: 14779

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8965` |
| 6 | 1.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 8 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.03% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 0.97% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9032` |
| 17 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.85% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5860` |
| 20 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.76% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 25 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.58% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5715` |
| 27 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8771` |
| 29 | 0.55% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6877` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 31 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.50% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4206` |
| 33 | 0.49% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 34 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4053` |
| 35 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 36 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 41 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 43 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 45 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 47 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4361` |
| 52 | 0.36% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 53 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 54 | 0.35% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 55 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.33% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 1.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 1.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 4 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8812` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8837` |
| 9 | 1.49% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.24% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 14 | 1.14% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 16 | 1.09% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 17 | 1.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 18 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 1.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.93% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 22 | 0.92% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5665` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3412` |
| 26 | 0.76% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4226` |
| 28 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 29 | 0.75% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

