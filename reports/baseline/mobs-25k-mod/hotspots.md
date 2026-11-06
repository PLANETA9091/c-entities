# JVM hotspots (JFR ExecutionSample)

Samples: 13921

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 7.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.58% | `net.minecraft.world.entity.Entity.moonrise$getSectionY() line: 453` |
| 4 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 7 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.22% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 10 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10491` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11127` |
| 13 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 15 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.84% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1293` |
| 23 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 24 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 27 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4933` |
| 29 | 0.54% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 30 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3706` |
| 31 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 32 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.49% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 36 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7056` |
| 37 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 39 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4870` |
| 41 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8026` |
| 44 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2448` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 47 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 11220` |
| 53 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 57 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11039` |
| 59 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7234` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 2 | 7.45% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 4.58% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2306` |
| 4 | 2.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2800` |
| 7 | 1.64% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 8 | 1.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.38% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10674` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 12 | 1.23% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 13 | 1.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7496` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 18 | 1.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2741` |
| 19 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 21 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 26 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.65% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 29 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7402` |
| 30 | 0.64% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

