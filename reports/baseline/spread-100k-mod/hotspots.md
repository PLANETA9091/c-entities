# JVM hotspots (JFR ExecutionSample)

Samples: 34547

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.87% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 3 | 3.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 7 | 2.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 11 | 1.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.18% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 17 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 1.14% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.98% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 4690` |
| 22 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 23 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.87% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.79% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 29 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 33 | 0.66% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.65% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 35 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.62% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 38 | 0.62% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.56% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 44 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 45 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 48 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 52 | 0.45% | `java.lang.ref.Reference.get() line: 361` |
| 53 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 54 | 0.43% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 55 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.43% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 57 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 59 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 60 | 0.36% | `org.spigotmc.WatchdogThread.run() line: 72` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 7 | 2.12% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.80% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.75% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 11 | 1.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.32% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.18% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 1.14% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 22 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 23 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.96% | `dev.planeta9091.centities.CeHooks.navIdle$(PathNavigation) line: 9121` |
| 25 | 0.95% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2657` |
| 28 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms

