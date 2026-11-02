# JVM hotspots (JFR ExecutionSample)

Samples: 31738

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.77% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10182` |
| 3 | 3.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6305` |
| 4 | 2.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5537` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9580` |
| 7 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10094` |
| 19 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9551` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 31 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 32 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 33 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.49% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6318` |
| 36 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6478` |
| 38 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9554` |
| 39 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 40 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 41 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.40% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 43 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10102` |
| 47 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9990` |
| 50 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 53 | 0.35% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 54 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 55 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.33% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5542` |
| 60 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9728` |
| 4 | 3.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.19% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.58% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4292` |
| 7 | 2.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.58% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.45% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.18% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 16 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.95% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6895` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7298` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6315` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 28 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.61% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3977` |

GC pauses: 0, total 0 ms

