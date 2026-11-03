# JVM hotspots (JFR ExecutionSample)

Samples: 23217

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6413` |
| 2 | 2.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10301` |
| 7 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9699` |
| 8 | 1.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 11 | 1.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.13% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5650` |
| 17 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9717` |
| 19 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6423` |
| 22 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6920` |
| 28 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 33 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4144` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6988` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10213` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9710` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5624` |
| 39 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 41 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9670` |
| 43 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 45 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 46 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.44% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6586` |
| 48 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.43% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 50 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 54 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.40% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 56 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 57 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9673` |
| 58 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9694` |
| 60 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9847` |
| 2 | 4.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.58% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.55% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4364` |
| 6 | 2.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5615` |
| 11 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 12 | 1.32% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.25% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.24% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.89% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7003` |
| 21 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.84% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 24 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6423` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6919` |

GC pauses: 0, total 0 ms

