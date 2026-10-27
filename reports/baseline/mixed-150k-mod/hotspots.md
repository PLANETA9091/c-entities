# JVM hotspots (JFR ExecutionSample)

Samples: 31771

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 3 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 5 | 1.75% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 6 | 1.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 12 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.80% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |
| 22 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 23 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.68% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 26 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 29 | 0.62% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 32 | 0.58% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 33 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 36 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 40 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.50% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 43 | 0.48% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 45 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.47% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.44% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 49 | 0.44% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.44% | `java.lang.ref.Reference.get() line: 361` |
| 51 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 53 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.40% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 55 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 57 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 58 | 0.37% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 653` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2947` |
| 60 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 3 | 3.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 4 | 2.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 5 | 1.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.64% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 10 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7557` |
| 12 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.18% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.05% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 16 | 1.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5498` |
| 18 | 0.88% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 0.83% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.81% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.79% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 0.79% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.69% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |

GC pauses: 0, total 0 ms

