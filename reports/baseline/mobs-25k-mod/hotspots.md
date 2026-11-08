# JVM hotspots (JFR ExecutionSample)

Samples: 11060

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 2.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 2.15% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 5 | 2.15% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 6 | 2.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5129` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10841` |
| 12 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 15 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5066` |
| 19 | 0.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8250` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7458` |
| 23 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7280` |
| 25 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.71% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.71% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 30 | 0.69% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.67% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 32 | 0.65% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 33 | 0.59% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 34 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 36 | 0.55% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 37 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11430` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 39 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 40 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 41 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7307` |
| 44 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 45 | 0.37% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 46 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 48 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.35% | `java.lang.String.hashCode() line: 2430` |
| 52 | 0.33% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 53 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 11611` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8248` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2731` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 60 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.07% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7720` |
| 2 | 2.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 2.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 4 | 2.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 5 | 2.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 6 | 2.15% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 7 | 2.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 10 | 1.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 13 | 1.25% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 15 | 1.22% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 16 | 1.20% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 17 | 1.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.11% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7626` |
| 19 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7796` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2599` |
| 21 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 23 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.77% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.77% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3595` |
| 27 | 0.72% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.69% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 29 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.66% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms

