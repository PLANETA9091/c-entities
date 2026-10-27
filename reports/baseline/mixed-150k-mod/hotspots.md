# JVM hotspots (JFR ExecutionSample)

Samples: 30136

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7092` |
| 3 | 2.66% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7281` |
| 4 | 2.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7145` |
| 7 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 8 | 1.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4060` |
| 11 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1433` |
| 14 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 18 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.71% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2112` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1417` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.57% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 33 | 0.56% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 34 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 35 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.48% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 40 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.42% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6179` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4210` |
| 52 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 54 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.35% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 626` |
| 56 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2083` |
| 58 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 60 | 0.31% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 3 | 3.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7141` |
| 4 | 2.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4207` |
| 9 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7153` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1418` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.24% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 18 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7133` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.69% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5099` |
| 29 | 0.58% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 30 | 0.57% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |

GC pauses: 0, total 0 ms

