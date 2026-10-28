# JVM hotspots (JFR ExecutionSample)

Samples: 15470

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 5 | 2.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.99% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.70% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.65% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 10 | 1.40% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 12 | 1.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 15 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 18 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.86% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 20 | 0.85% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.83% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 23 | 0.80% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 27 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7003` |
| 29 | 0.67% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3094` |
| 31 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 32 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 35 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 37 | 0.52% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 38 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.48% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.48% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 44 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.45% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 46 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.43% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6536` |
| 48 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.42% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 51 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 52 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 53 | 0.39% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 5107` |
| 54 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.37% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 56 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.37% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3247` |
| 58 | 0.36% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.36% | `net.minecraft.world.entity.Entity.computeSpeed() line: 966` |
| 60 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 4 | 2.17% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 6 | 1.65% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 9 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.41% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 11 | 1.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6961` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 20 | 0.86% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 21 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 23 | 0.83% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.80% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6792` |
| 26 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.64% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 29 | 0.60% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

