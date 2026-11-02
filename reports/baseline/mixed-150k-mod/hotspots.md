# JVM hotspots (JFR ExecutionSample)

Samples: 30832

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.53% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 3 | 3.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9257` |
| 6 | 3.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 9 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 10 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 12 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.19% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.96% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 21 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.81% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.77% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 36 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 37 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.42% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 40 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3996` |
| 42 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9202` |
| 44 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 45 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4342` |
| 49 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 51 | 0.33% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8759` |
| 52 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3869` |
| 54 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 55 | 0.31% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 57 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.55% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9112` |
| 7 | 2.17% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.11% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 2.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 2.02% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.66% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.50% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9104` |
| 15 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.19% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 18 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5440` |
| 19 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 1.01% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 21 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.91% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7046` |
| 23 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 29 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |

GC pauses: 0, total 0 ms

