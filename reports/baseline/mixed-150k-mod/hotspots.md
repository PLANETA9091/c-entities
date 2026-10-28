# JVM hotspots (JFR ExecutionSample)

Samples: 43371

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.00% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7770` |
| 4 | 2.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 7 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4689` |
| 8 | 1.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4544` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 11 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.98% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 18 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 23 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 26 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.61% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3980` |
| 31 | 0.55% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 32 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 34 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2676` |
| 35 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.44% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7715` |
| 39 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 40 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7590` |
| 44 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.37% | `java.lang.String.equals(Object) line: 1926` |
| 46 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3022` |
| 50 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 54 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6663` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7837` |
| 57 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2959` |
| 59 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 60 | 0.30% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7625` |
| 8 | 1.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 9 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.25% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.18% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7642` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 16 | 0.98% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 18 | 0.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 19 | 0.82% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.72% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4544` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4017` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4691` |
| 29 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |

GC pauses: 0, total 0 ms

