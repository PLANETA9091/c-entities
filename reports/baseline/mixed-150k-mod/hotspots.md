# JVM hotspots (JFR ExecutionSample)

Samples: 38473

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 2 | 3.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5860` |
| 4 | 3.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8965` |
| 6 | 2.88% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5133` |
| 8 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8771` |
| 10 | 1.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5715` |
| 13 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.77% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8910` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8785` |
| 30 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.48% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 39 | 0.45% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 41 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 42 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 44 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 47 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 50 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 55 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 60 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8820` |
| 6 | 2.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.27% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.06% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 2.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.79% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8837` |
| 12 | 1.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8812` |
| 13 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.54% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5170` |
| 16 | 1.17% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2040` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5862` |

GC pauses: 0, total 0 ms

