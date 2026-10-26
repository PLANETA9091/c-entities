# JVM hotspots (JFR ExecutionSample)

Samples: 34297

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.08% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6703` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 5 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 8 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.51% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 12 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6539` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.92% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.88% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 20 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.76% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 24 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.74% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 26 | 0.71% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 28 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2110` |
| 30 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4726` |
| 32 | 0.61% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 33 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 37 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 40 | 0.50% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4415` |
| 41 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 42 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4208` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.43% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 624` |
| 48 | 0.42% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 49 | 0.41% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 50 | 0.41% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 51 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 52 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 53 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 55 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 57 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 59 | 0.33% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 60 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 3 | 3.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 5 | 2.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.82% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 9 | 1.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 10 | 1.55% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 12 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4736` |
| 14 | 1.26% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 16 | 1.17% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 1.11% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 18 | 1.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 22 | 1.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4008` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.84% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4512` |
| 27 | 0.83% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 28 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 29 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.74% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms

