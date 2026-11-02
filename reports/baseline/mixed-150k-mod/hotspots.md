# JVM hotspots (JFR ExecutionSample)

Samples: 29345

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9949` |
| 4 | 2.72% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 5 | 2.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 8 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4769` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9433` |
| 14 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 23 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9861` |
| 26 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 28 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4054` |
| 33 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 35 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 36 | 0.44% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 37 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9404` |
| 38 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 39 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.42% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 42 | 0.42% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 43 | 0.41% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 44 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9407` |
| 48 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 49 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.34% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9521` |
| 5 | 2.89% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9452` |
| 7 | 2.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.19% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.87% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.22% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 16 | 0.93% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9511` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7177` |
| 22 | 0.73% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 23 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 27 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3960` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |

GC pauses: 0, total 0 ms

