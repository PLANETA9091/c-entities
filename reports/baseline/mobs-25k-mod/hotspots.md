# JVM hotspots (JFR ExecutionSample)

Samples: 15723

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 5 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.72% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.71% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.44% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 10 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.05% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 14 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 0.90% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 0.89% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 19 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 22 | 0.79% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 23 | 0.78% | `java.lang.ref.Reference.get() line: 361` |
| 24 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 25 | 0.73% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2865` |
| 26 | 0.73% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 27 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 29 | 0.66% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 30 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 31 | 0.63% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6692` |
| 32 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 33 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 34 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 36 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.60% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5539` |
| 38 | 0.60% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 39 | 0.59% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7670` |
| 40 | 0.58% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 42 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 44 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.54% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 46 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.50% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.47% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 51 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 52 | 0.46% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2802` |
| 53 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 56 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 57 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.41% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4796` |
| 59 | 0.40% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6225` |
| 60 | 0.39% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 2 | 3.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.97% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 6 | 1.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.45% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 8 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 9 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7475` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.05% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 16 | 1.02% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 17 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.97% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 19 | 0.96% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6650` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 21 | 0.89% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.89% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.89% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 25 | 0.82% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 27 | 0.66% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$FastEntryIterator.next() line: 766` |

GC pauses: 0, total 0 ms

