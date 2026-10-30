# JVM hotspots (JFR ExecutionSample)

Samples: 34606

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.29% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 3 | 3.34% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8423` |
| 4 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 6 | 2.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 9 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 14 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8368` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8243` |
| 28 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 31 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3289` |
| 33 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 34 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 41 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7925` |
| 43 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 45 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 47 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 49 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1990` |
| 51 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 52 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 56 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 57 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3635` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8278` |
| 6 | 2.97% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.38% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.66% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8270` |
| 12 | 1.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.45% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.23% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 17 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4631` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.81% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 22 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 30 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

