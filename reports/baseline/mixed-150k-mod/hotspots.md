# JVM hotspots (JFR ExecutionSample)

Samples: 21666

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.tickEffects(ServerLevel, LivingEntity)` |
| 2 | 5.68% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 4 | 4.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 5 | 3.81% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9532` |
| 6 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 9 | 2.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 10 | 2.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 11 | 2.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 18 | 1.10% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.01% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 21 | 0.96% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9477` |
| 23 | 0.88% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.68% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.66% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9373` |
| 27 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9166` |
| 29 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 30 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.50% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 32 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 33 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 34 | 0.48% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 35 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 37 | 0.39% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 38 | 0.39% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1199` |
| 39 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3797` |
| 41 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 42 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1429` |
| 43 | 0.31% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 44 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 45 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.30% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7062` |
| 47 | 0.29% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3973` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 49 | 0.28% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.28% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 52 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 53 | 0.24% | `java.lang.AbstractStringBuilder.append(String) line: 650` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8786` |
| 55 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.21% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 59 | 0.21% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.21% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.67% | `dev.planeta9091.centities.CeHooks.enchTickEffects$(ServerLevel, LivingEntity) line: 9335` |
| 2 | 5.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 3.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 3.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 8 | 3.01% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 2.53% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 2.45% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 2.34% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 2.28% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 2.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 15 | 1.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.62% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9195` |
| 18 | 1.09% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3926` |
| 20 | 1.01% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9220` |
| 22 | 1.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 23 | 0.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 24 | 0.91% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.87% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7073` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates()` |
| 27 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 28 | 0.60% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1173` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |

GC pauses: 0, total 0 ms

