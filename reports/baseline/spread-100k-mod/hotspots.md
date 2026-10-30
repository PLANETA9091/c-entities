# JVM hotspots (JFR ExecutionSample)

Samples: 45791

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.24% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 3 | 2.74% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 4 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 7 | 2.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 9 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 12 | 1.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.02% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.94% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 20 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 23 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.79% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 25 | 0.76% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 27 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.64% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 32 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.62% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 35 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2450` |
| 36 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.54% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8578` |
| 38 | 0.54% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 39 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 40 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.51% | `java.lang.ref.Reference.get() line: 361` |
| 42 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 43 | 0.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 44 | 0.49% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 45 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 47 | 0.45% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 48 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 49 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 51 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 54 | 0.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 55 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 58 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 60 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 3 | 3.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.78% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.68% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 11 | 1.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.59% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.44% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 16 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 17 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 1.11% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1921` |
| 20 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.97% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.94% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 23 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2446` |
| 24 | 0.83% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.75% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 28 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.70% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

