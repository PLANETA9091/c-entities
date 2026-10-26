# JVM hotspots (JFR ExecutionSample)

Samples: 17326

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 4 | 1.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.25% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.18% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 8 | 1.17% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 9 | 1.15% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 15 | 1.06% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 17 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 18 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 20 | 0.87% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 21 | 0.83% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 22 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.81% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 24 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 27 | 0.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 28 | 0.68% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 29 | 0.66% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4409` |
| 31 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.61% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 34 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 35 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 36 | 0.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 37 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 38 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 39 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 40 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.51% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 42 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 44 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4364` |
| 46 | 0.47% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 47 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 48 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.45% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 50 | 0.44% | `java.lang.String.hashCode() line: 2433` |
| 51 | 0.44% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6522` |
| 52 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 55 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2559` |
| 59 | 0.33% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 60 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 3.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.11% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4730` |
| 5 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 1.81% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 7 | 1.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4662` |
| 9 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.28% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 14 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 15 | 1.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 16 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 1.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4506` |
| 20 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 22 | 0.85% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.84% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.73% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 27 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.68% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

