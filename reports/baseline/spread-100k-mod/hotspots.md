# JVM hotspots (JFR ExecutionSample)

Samples: 35964

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.88% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 3 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 5 | 2.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 6 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 9 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 14 | 1.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.17% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.81% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.74% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 28 | 0.72% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.70% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 34 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 36 | 0.58% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 37 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3988` |
| 47 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 50 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8657` |
| 52 | 0.42% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 54 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 56 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2610` |
| 57 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8782` |
| 58 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 60 | 0.36% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 6 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.44% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2594` |
| 11 | 1.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.54% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1985` |
| 15 | 1.17% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 18 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 1.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2611` |
| 22 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 23 | 0.84% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 24 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.79% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.78% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 27 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.77% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms

