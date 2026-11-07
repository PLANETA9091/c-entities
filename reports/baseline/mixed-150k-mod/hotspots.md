# JVM hotspots (JFR ExecutionSample)

Samples: 29393

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 3.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 2.57% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 2.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 1.25% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 15 | 1.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 17 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 19 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 21 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 23 | 0.87% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.83% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 27 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 28 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.73% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 32 | 0.70% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 33 | 0.69% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 35 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.64% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 38 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 39 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 40 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2799` |
| 41 | 0.50% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 42 | 0.50% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 43 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11403` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 46 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 49 | 0.40% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 50 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 51 | 0.39% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 11291` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 54 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8215` |
| 55 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 57 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 5 | 2.57% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 6 | 2.38% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 7 | 2.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.10% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 11 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 13 | 1.30% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 15 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 1.25% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |
| 17 | 1.25% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnaligned(MemorySessionImpl, Object, long, boolean) line: 3849` |
| 18 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 19 | 1.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 20 | 1.00% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 21 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.94% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 24 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 25 | 0.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 26 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 27 | 0.84% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 28 | 0.80% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 30 | 0.80% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |

GC pauses: 0, total 0 ms

