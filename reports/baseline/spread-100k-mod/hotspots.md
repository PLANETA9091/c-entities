# JVM hotspots (JFR ExecutionSample)

Samples: 31346

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6497` |
| 3 | 3.61% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 4 | 3.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 6 | 2.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.88% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 10 | 1.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.21% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 12 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4376` |
| 15 | 1.02% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.89% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 18 | 0.88% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.86% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.85% | `java.lang.String.hashCode() line: 2433` |
| 22 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 24 | 0.79% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.78% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.77% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.76% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 30 | 0.76% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.75% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 33 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.66% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 35 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 36 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 39 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 40 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 42 | 0.55% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 44 | 0.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 45 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4664` |
| 46 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 49 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.42% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4410` |
| 52 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 53 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.40% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 55 | 0.39% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4365` |
| 56 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2105` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 3.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4731` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 11 | 1.70% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 14 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 15 | 1.31% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 1.30% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4003` |
| 17 | 1.17% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 20 | 1.14% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 1.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 1.05% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 1.01% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.89% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 29 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms

