# JVM hotspots (JFR ExecutionSample)

Samples: 31286

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 3 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9257` |
| 6 | 2.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 7 | 2.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 10 | 1.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.92% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.84% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.77% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 26 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 28 | 0.72% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.61% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.58% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 36 | 0.57% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 37 | 0.53% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 38 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 39 | 0.52% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 41 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 42 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 46 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 49 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4342` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9202` |
| 53 | 0.38% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7169` |
| 54 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 55 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3996` |
| 56 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 58 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 2.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9112` |
| 5 | 2.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.21% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.76% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.59% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5440` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9104` |
| 17 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 19 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.99% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.97% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.85% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 30 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms

