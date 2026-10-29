# JVM hotspots (JFR ExecutionSample)

Samples: 41549

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 4 | 3.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 6 | 2.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 7 | 1.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4734` |
| 9 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 12 | 1.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.89% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 0.85% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.60% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7905` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.60% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.57% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 35 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 36 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7462` |
| 38 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 39 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 42 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7780` |
| 44 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 45 | 0.41% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.39% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 50 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4884` |
| 52 | 0.38% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 53 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 54 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 57 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2856` |
| 60 | 0.34% | `java.lang.ref.Reference.get() line: 361` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7815` |
| 7 | 1.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.69% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.52% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 12 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7807` |
| 14 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 20 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.85% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.69% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5773` |

GC pauses: 0, total 0 ms

