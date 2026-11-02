# JVM hotspots (JFR ExecutionSample)

Samples: 28738

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9395` |
| 2 | 3.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 4 | 3.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9898` |
| 6 | 2.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5472` |
| 7 | 2.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.11% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.79% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9843` |
| 22 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.66% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 24 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 26 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.putAll(Map) line: 226` |
| 28 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.52% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 32 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9411` |
| 33 | 0.49% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 34 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6254` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 37 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 39 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4052` |
| 40 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.41% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 45 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4398` |
| 50 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 51 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 52 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7284` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9739` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9390` |
| 60 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9504` |
| 2 | 5.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.06% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.05% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9494` |
| 10 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.12% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.10% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.01% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 15 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 0.89% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7160` |
| 20 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.72% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 25 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 28 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 30 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 125` |

GC pauses: 0, total 0 ms

