# JVM hotspots (JFR ExecutionSample)

Samples: 36817

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.36% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 4 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 10 | 1.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.87% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.84% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.pushChunkWait(ServerLevel, int, int) line: 923` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 27 | 0.65% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 28 | 0.64% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8922` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 31 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 34 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8797` |
| 41 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 46 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 50 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 52 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 54 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 55 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 59 | 0.27% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.27% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.09% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 5 | 3.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 8 | 2.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 1.97% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.41% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 13 | 1.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.05% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 17 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.87% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.85% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 22 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.82% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 122` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.69% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |

GC pauses: 0, total 0 ms

