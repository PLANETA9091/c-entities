# JVM hotspots (JFR ExecutionSample)

Samples: 30597

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6311` |
| 2 | 3.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.24% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10199` |
| 5 | 2.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5543` |
| 6 | 2.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9597` |
| 8 | 1.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10111` |
| 20 | 0.72% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 21 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6818` |
| 29 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9571` |
| 31 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.47% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 35 | 0.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6886` |
| 36 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 38 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9568` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.38% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10007` |
| 45 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9608` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 48 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 51 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.35% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6662` |
| 56 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9745` |
| 3 | 3.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.06% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 7 | 2.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.95% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.82% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.38% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.24% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.19% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.11% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6901` |
| 19 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.86% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6321` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7304` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6886` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.64% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

