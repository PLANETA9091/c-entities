# JVM hotspots (JFR ExecutionSample)

Samples: 14178

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.69% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 6 | 1.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 7 | 1.54% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 11 | 1.21% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.09% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 15 | 0.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 16 | 0.90% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2865` |
| 19 | 0.83% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 23 | 0.75% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.75% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2802` |
| 25 | 0.73% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 26 | 0.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7670` |
| 31 | 0.67% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 32 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 37 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 38 | 0.60% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5539` |
| 39 | 0.60% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.59% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 42 | 0.56% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4796` |
| 43 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 44 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.49% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6692` |
| 46 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.45% | `java.lang.String.hashCode() line: 2430` |
| 48 | 0.44% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6225` |
| 49 | 0.43% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2975` |
| 50 | 0.42% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 51 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 52 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 53 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.40% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 4 | 2.09% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 1.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 1.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.20% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 13 | 1.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7475` |
| 15 | 1.10% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 16 | 1.05% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 18 | 0.93% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 19 | 0.90% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 20 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.83% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 22 | 0.83% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 23 | 0.82% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.76% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 29 | 0.71% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 30 | 0.71% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |

GC pauses: 0, total 0 ms

