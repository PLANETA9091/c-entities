# JVM hotspots (JFR ExecutionSample)

Samples: 30746

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9149` |
| 2 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 4 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.68% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6031` |
| 7 | 2.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8955` |
| 9 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5877` |
| 11 | 1.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.10% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 19 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5295` |
| 24 | 0.80% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 25 | 0.79% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.74% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.69% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 29 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 30 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.63% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.59% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 36 | 0.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3888` |
| 37 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.54% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 39 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4234` |
| 43 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 45 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7061` |
| 46 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 47 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 48 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 50 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 52 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 53 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 57 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 58 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4171` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 5.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9004` |
| 6 | 2.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 2.04% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.82% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9021` |
| 12 | 1.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 15 | 1.20% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 17 | 1.10% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 18 | 1.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5332` |
| 20 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8996` |
| 21 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.71% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 30 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |

GC pauses: 0, total 0 ms

