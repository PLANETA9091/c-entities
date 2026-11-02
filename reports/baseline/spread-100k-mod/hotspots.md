# JVM hotspots (JFR ExecutionSample)

Samples: 25441

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10216` |
| 3 | 2.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6328` |
| 4 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.86% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9614` |
| 10 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 19 | 0.95% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6835` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 0.87% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4797` |
| 23 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.77% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 27 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 29 | 0.72% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.70% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 31 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.66% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 34 | 0.64% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7445` |
| 35 | 0.64% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.63% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.62% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 39 | 0.62% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4444` |
| 41 | 0.53% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 42 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.50% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 44 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.48% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 49 | 0.45% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 51 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 52 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.41% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10024` |
| 55 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9625` |
| 58 | 0.39% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6679` |
| 59 | 0.38% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6501` |
| 60 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 5 | 2.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 2.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 2.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.09% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6918` |
| 9 | 1.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 12 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.36% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.30% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 16 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.01% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 22 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 24 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 25 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.76% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 28 | 0.72% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |

GC pauses: 0, total 0 ms

