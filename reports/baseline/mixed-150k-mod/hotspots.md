# JVM hotspots (JFR ExecutionSample)

Samples: 33980

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 2 | 3.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.38% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9334` |
| 4 | 3.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6152` |
| 5 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5416` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9107` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5998` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.99% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 0.77% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9279` |
| 25 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9152` |
| 27 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.56% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 30 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 31 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 32 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 33 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.46% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 37 | 0.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 38 | 0.45% | `java.lang.String.equals(Object) line: 1926` |
| 39 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 41 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.36% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 44 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 45 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 50 | 0.35% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4009` |
| 52 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 4.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9189` |
| 6 | 2.58% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9181` |
| 8 | 1.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9206` |
| 10 | 1.67% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 14 | 1.31% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7059` |
| 22 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |

GC pauses: 0, total 0 ms

