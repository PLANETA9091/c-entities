# JVM hotspots (JFR ExecutionSample)

Samples: 39284

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.44% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 3 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 4 | 2.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 9 | 1.86% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 12 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 14 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.02% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 17 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.90% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1448` |
| 21 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1445` |
| 24 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7262` |
| 27 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 28 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.54% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 31 | 0.53% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 32 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 33 | 0.51% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.49% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 36 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 37 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.44% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6210` |
| 41 | 0.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 42 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7137` |
| 43 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.36% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2599` |
| 50 | 0.36% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 51 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 53 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 57 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.30% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 59 | 0.29% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 3 | 3.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7172` |
| 6 | 2.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.01% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.79% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.64% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1449` |
| 12 | 1.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.26% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.24% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 16 | 1.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.02% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.86% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5130` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 28 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7164` |
| 30 | 0.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

