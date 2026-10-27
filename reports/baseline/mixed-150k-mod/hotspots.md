# JVM hotspots (JFR ExecutionSample)

Samples: 38032

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 3.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 4 | 2.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 2.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 7 | 1.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 11 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 13 | 1.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.32% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 15 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.88% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 22 | 0.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 25 | 0.78% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 26 | 0.75% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 33 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 36 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 37 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 40 | 0.47% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 43 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 46 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.44% | `java.lang.ref.Reference.get() line: 361` |
| 49 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 50 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 52 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 55 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2409` |
| 58 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 60 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 3 | 3.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 6 | 2.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 7 | 1.94% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.87% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 1.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.72% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.41% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.37% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 15 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 17 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 20 | 0.90% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.88% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.75% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 28 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

