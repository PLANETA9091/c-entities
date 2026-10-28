# JVM hotspots (JFR ExecutionSample)

Samples: 24868

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 3 | 2.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7770` |
| 4 | 2.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 7 | 2.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4544` |
| 12 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.97% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 20 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.84% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4689` |
| 25 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1851` |
| 28 | 0.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5212` |
| 30 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.58% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.58% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.56% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 35 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4867` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7715` |
| 38 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2578` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 41 | 0.48% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 42 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5155` |
| 44 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.43% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 47 | 0.43% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 48 | 0.42% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 49 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 653` |
| 51 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2676` |
| 54 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 55 | 0.37% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 56 | 0.36% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4691` |
| 58 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.35% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7625` |
| 3 | 2.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 4 | 2.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.92% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.68% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1393` |
| 12 | 1.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.29% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5222` |
| 14 | 1.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4017` |
| 19 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.99% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5583` |
| 21 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.88% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4544` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 26 | 0.82% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 27 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5154` |
| 28 | 0.76% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 29 | 0.76% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4691` |

GC pauses: 0, total 0 ms

