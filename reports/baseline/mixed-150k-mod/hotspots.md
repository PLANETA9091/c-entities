# JVM hotspots (JFR ExecutionSample)

Samples: 28959

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10380` |
| 3 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.48% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 7 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9778` |
| 12 | 1.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.75% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10292` |
| 20 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 22 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.61% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 27 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.55% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9752` |
| 32 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9749` |
| 35 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.46% | `java.util.EnumMap$EntrySet.iterator() line: 475` |
| 37 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6486` |
| 38 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 39 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 41 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.41% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 43 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 44 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 45 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9796` |
| 49 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 51 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 53 | 0.36% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 54 | 0.36% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 55 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.35% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 58 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10188` |
| 60 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9926` |
| 5 | 2.56% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.48% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 7 | 2.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.03% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 17 | 1.03% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.83% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 25 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 29 | 0.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

