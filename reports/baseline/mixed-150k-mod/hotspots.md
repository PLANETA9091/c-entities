# JVM hotspots (JFR ExecutionSample)

Samples: 49443

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.31% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 5 | 2.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 8 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.55% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 14 | 1.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.04% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 18 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 20 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 22 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.59% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.57% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 31 | 0.50% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 34 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 36 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 39 | 0.41% | `java.lang.ref.Reference.get() line: 361` |
| 40 | 0.41% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 42 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 44 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 45 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 49 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 52 | 0.31% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 53 | 0.31% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 54 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 56 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 58 | 0.27% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 59 | 0.27% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7003` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.32% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 2.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 9 | 1.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.55% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 11 | 1.48% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.46% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.43% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.19% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.05% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.03% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.60% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 28 | 0.59% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |

GC pauses: 0, total 0 ms

