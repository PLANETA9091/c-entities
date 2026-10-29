# JVM hotspots (JFR ExecutionSample)

Samples: 32217

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.52% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 3 | 3.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 4 | 2.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 5 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 9 | 1.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 10 | 1.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 14 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 1.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.97% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.88% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.84% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.77% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.73% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.66% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 35 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 36 | 0.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8328` |
| 40 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 42 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2336` |
| 45 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8203` |
| 48 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 50 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3255` |
| 52 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3601` |
| 56 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2317` |
| 57 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6295` |
| 59 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8238` |
| 6 | 2.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.40% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.98% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.85% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.69% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1809` |
| 12 | 1.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.45% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8230` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 17 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 22 | 0.91% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.85% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

