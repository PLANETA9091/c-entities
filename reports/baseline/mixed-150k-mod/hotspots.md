# JVM hotspots (JFR ExecutionSample)

Samples: 36394

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.23% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 3 | 3.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6031` |
| 5 | 2.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5295` |
| 7 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8955` |
| 8 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5877` |
| 10 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 16 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 21 | 0.85% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9094` |
| 25 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.51% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 36 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8969` |
| 38 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3888` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 43 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 44 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 50 | 0.34% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 51 | 0.33% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 52 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 53 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.30% | `java.lang.String.equals(Object) line: 1926` |
| 57 | 0.30% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 58 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4234` |
| 60 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.81% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9004` |
| 6 | 2.59% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.73% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8996` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5332` |
| 12 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 17 | 1.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.90% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 22 | 0.85% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.80% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.66% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.65% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

