# JVM hotspots (JFR ExecutionSample)

Samples: 15612

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.67% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1581` |
| 2 | 3.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3320` |
| 3 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3321` |
| 5 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.11% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.08% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.95% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 12 | 0.85% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9442` |
| 14 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 16 | 0.78% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6213` |
| 17 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.74% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9095` |
| 20 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9520` |
| 21 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 22 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1582` |
| 24 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9273` |
| 26 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6150` |
| 29 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3084` |
| 31 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 33 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 35 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.44% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4804` |
| 37 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.42% | `java.util.ArrayList.add(Object) line: 497` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10065` |
| 40 | 0.41% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 41 | 0.37% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 42 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 43 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.34% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 46 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3267` |
| 49 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3303` |
| 50 | 0.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 52 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14471` |
| 55 | 0.28% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 14652` |
| 56 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 58 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.39% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 6.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3084` |
| 3 | 1.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9535` |
| 6 | 1.58% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.37% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.22% | `java.util.ArrayList.add(Object) line: 497` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9520` |
| 12 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9441` |
| 16 | 0.86% | `java.util.HashMap.get(Object) line: 565` |
| 17 | 0.82% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9611` |
| 20 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6460` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3389` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.58% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 26 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.51% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |

GC pauses: 0, total 0 ms

