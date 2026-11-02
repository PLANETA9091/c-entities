# JVM hotspots (JFR ExecutionSample)

Samples: 19856

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.tickEffects(ServerLevel, LivingEntity)` |
| 2 | 3.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 4 | 3.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 5 | 3.13% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9532` |
| 6 | 2.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.75% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 9 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 12 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.24% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 17 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 19 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 20 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.87% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 24 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.70% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 27 | 0.64% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.63% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.61% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 33 | 0.56% | `java.util.HashMap.getNode(Object) line: 586` |
| 34 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.46% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 37 | 0.45% | `java.lang.AbstractStringBuilder.append(String) line: 650` |
| 38 | 0.44% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 40 | 0.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 41 | 0.38% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 42 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 43 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9166` |
| 44 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9477` |
| 45 | 0.33% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 46 | 0.32% | `java.lang.AbstractStringBuilder.ensureCapacityNewCoder(byte[], byte, int, int, byte) line: 287` |
| 47 | 0.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3797` |
| 49 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 50 | 0.30% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3973` |
| 51 | 0.29% | `java.lang.String.getBytes(byte[], int, byte) line: 4799` |
| 52 | 0.28% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 53 | 0.28% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 54 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 55 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.24% | `java.lang.String.length() line: 1583` |
| 59 | 0.24% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1199` |
| 60 | 0.24% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.31% | `dev.planeta9091.centities.CeHooks.enchTickEffects$(ServerLevel, LivingEntity) line: 9335` |
| 2 | 5.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 7 | 2.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.92% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.67% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.63% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 16 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.98% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3926` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9195` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 21 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2086` |
| 24 | 0.79% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 25 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 26 | 0.72% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 27 | 0.72% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7073` |
| 28 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.tick() line: 3481` |
| 30 | 0.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |

GC pauses: 0, total 0 ms

