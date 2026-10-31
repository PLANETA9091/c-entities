# JVM hotspots (JFR ExecutionSample)

Samples: 12543

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9044` |
| 11 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.25% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 1.18% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 17 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 21 | 1.03% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 24 | 0.88% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 26 | 0.83% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.81% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 28 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 31 | 0.62% | `java.lang.ref.Reference.get() line: 361` |
| 32 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 33 | 0.58% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.57% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 35 | 0.56% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 36 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 38 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4053` |
| 39 | 0.51% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 41 | 0.46% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 45 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 50 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4361` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4206` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 55 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6889` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 59 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.38% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3412` |
| 3 | 1.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 1.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8849` |
| 5 | 1.88% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.63% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.37% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.29% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 12 | 1.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.25% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 16 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 18 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 1.08% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 1.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 23 | 1.04% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 24 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 25 | 0.94% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.89% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5677` |
| 29 | 0.85% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

