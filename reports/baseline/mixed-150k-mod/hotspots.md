# JVM hotspots (JFR ExecutionSample)

Samples: 35634

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 3 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8832` |
| 4 | 3.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5727` |
| 6 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4936` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 9 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5582` |
| 11 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.70% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8777` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8652` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8334` |
| 30 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.54% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 33 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 34 | 0.51% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3637` |
| 35 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 39 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.41% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 45 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.35% | `java.lang.ref.Reference.get() line: 361` |
| 49 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 51 | 0.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 52 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 54 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3983` |
| 57 | 0.30% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.29% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 60 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.86% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.08% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.62% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 8 | 1.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8679` |
| 11 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.44% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5037` |
| 19 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2170` |

GC pauses: 0, total 0 ms

