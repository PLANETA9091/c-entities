# JVM hotspots (JFR ExecutionSample)

Samples: 31506

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.95% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9334` |
| 3 | 3.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6152` |
| 7 | 2.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9107` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5998` |
| 10 | 1.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5416` |
| 14 | 1.30% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.98% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.81% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 111` |
| 23 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.75% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 26 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9152` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 33 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 36 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 41 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4355` |
| 43 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9279` |
| 48 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 49 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 52 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 53 | 0.37% | `java.lang.ref.Reference.get() line: 361` |
| 54 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.34% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4009` |
| 57 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7182` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6165` |
| 60 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9189` |
| 6 | 2.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.25% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.13% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.95% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.76% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.32% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9181` |
| 16 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2082` |
| 22 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.81% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |
| 25 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 29 | 0.72% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.71% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

