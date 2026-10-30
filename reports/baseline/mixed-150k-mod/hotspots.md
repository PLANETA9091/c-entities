# JVM hotspots (JFR ExecutionSample)

Samples: 33217

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 3 | 3.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.05% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 5 | 3.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 6 | 2.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 8 | 1.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 10 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.63% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8782` |
| 26 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 28 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 34 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8657` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 39 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 40 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 41 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 42 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3988` |
| 44 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8339` |
| 45 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.36% | `java.lang.ref.Reference.get() line: 361` |
| 48 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 51 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 53 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.31% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 58 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 59 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 4.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.16% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 5 | 3.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 7 | 2.40% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.76% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 10 | 1.60% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.16% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.01% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 19 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2170` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.71% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6626` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.62% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

