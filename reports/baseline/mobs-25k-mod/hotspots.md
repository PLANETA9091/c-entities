# JVM hotspots (JFR ExecutionSample)

Samples: 11762

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 2 | 3.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.58% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 5 | 2.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 7 | 1.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 10 | 1.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 12 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 15 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7590` |
| 18 | 1.12% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 19 | 0.96% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4506` |
| 20 | 0.95% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 25 | 0.74% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 26 | 0.73% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 27 | 0.72% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 28 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.63% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 31 | 0.62% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 36 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 39 | 0.52% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 40 | 0.51% | `java.lang.String.hashCode() line: 2430` |
| 41 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 43 | 0.47% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 44 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 45 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.39% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6673` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7588` |
| 50 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 51 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 54 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 55 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 58 | 0.30% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 59 | 0.29% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6647` |
| 60 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 4.08% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 3 | 3.22% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 4 | 3.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 5 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 2.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 9 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 1.54% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 14 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.33% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 16 | 1.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7136` |
| 17 | 1.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6979` |
| 18 | 1.12% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 19 | 0.99% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7586` |
| 20 | 0.95% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 21 | 0.89% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.89% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 23 | 0.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.78% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 26 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 27 | 0.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

