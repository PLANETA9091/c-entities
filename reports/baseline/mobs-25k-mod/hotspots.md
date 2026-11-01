# JVM hotspots (JFR ExecutionSample)

Samples: 14452

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9080` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8939` |
| 8 | 1.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 0.89% | `java.lang.ref.Reference.get() line: 361` |
| 16 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5962` |
| 18 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4165` |
| 22 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 23 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4255` |
| 25 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8886` |
| 27 | 0.48% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5808` |
| 28 | 0.47% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4102` |
| 30 | 0.45% | `java.lang.Enum.ordinal() line: 128` |
| 31 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.44% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 33 | 0.44% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6992` |
| 34 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.43% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 36 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.40% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 40 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 45 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 46 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 47 | 0.34% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 48 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.33% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 50 | 0.33% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4432` |
| 51 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 56 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9147` |
| 59 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 60 | 0.28% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.94% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 2 | 2.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 1.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.63% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8927` |
| 8 | 1.58% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 11 | 1.16% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 1.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5758` |
| 16 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 21 | 0.91% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 23 | 0.89% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4275` |
| 24 | 0.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 25 | 0.82% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3459` |
| 26 | 0.77% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.75% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4431` |
| 29 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 30 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

