# JVM hotspots (JFR ExecutionSample)

Samples: 34543

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8699` |
| 3 | 3.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8840` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5735` |
| 5 | 3.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4944` |
| 8 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8646` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5590` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 12 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.05% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.83% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8785` |
| 26 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8660` |
| 29 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 31 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 33 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3645` |
| 38 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.39% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 45 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8793` |
| 49 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.33% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 51 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 52 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 54 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 55 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3991` |
| 56 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.30% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 60 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.56% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8695` |
| 7 | 2.47% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.48% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5045` |
| 15 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.16% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2173` |
| 18 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 1.03% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.95% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.69% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6629` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

