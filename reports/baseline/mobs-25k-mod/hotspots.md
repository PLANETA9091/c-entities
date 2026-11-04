# JVM hotspots (JFR ExecutionSample)

Samples: 13632

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 7.75% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.48% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.66% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 5 | 2.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.63% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 9 | 1.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 10 | 1.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 0.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 12 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.81% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 14 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10075` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 18 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10677` |
| 23 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 26 | 0.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 27 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.47% | `java.util.ArrayList.elementData(int) line: 412` |
| 29 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10223` |
| 31 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 32 | 0.45% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10770` |
| 33 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.44% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 35 | 0.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 36 | 0.40% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1580` |
| 37 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3985` |
| 42 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.33% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 45 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 47 | 0.29% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 48 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 49 | 0.29% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 50 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 52 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 53 | 0.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 54 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 55 | 0.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4832` |
| 56 | 0.26% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7858` |
| 57 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 58 | 0.25% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7092` |
| 59 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.24% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4769` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 7.75% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 5.10% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 3.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 5 | 2.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 2.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.64% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 8 | 1.64% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 9 | 1.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 10 | 1.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 12 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10223` |
| 15 | 0.87% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 17 | 0.84% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 20 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 21 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 22 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 23 | 0.65% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7247` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1534` |
| 30 | 0.56% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

