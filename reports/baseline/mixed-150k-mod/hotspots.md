# JVM hotspots (JFR ExecutionSample)

Samples: 33724

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.68% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 3 | 3.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 4 | 3.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 9 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 13 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 16 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.74% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 26 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9202` |
| 30 | 0.58% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 31 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 35 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3996` |
| 41 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 42 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 45 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 50 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 52 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 53 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4342` |
| 56 | 0.31% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.36% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 5 | 2.66% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9112` |
| 7 | 1.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.57% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9104` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5440` |
| 13 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 18 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 1.01% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.62% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |

GC pauses: 0, total 0 ms

