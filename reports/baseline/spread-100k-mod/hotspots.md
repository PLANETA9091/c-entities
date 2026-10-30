# JVM hotspots (JFR ExecutionSample)

Samples: 38553

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.28% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 4 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 6 | 2.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8423` |
| 7 | 2.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 9 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 14 | 1.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 17 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.04% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.94% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.78% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 31 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.63% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.60% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 34 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.53% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 42 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8243` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8368` |
| 47 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 48 | 0.42% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 49 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3635` |
| 51 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 52 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.39% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3289` |
| 57 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 2.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8278` |
| 5 | 2.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.06% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.54% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.52% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.48% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.15% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4631` |
| 18 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 1.02% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 21 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8270` |
| 22 | 0.94% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1831` |
| 26 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.79% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

