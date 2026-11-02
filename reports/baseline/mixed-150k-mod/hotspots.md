# JVM hotspots (JFR ExecutionSample)

Samples: 30632

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 5 | 3.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 6 | 3.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9267` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5995` |
| 9 | 2.09% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 10 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 14 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 18 | 0.95% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.90% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 21 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.79% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.72% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9087` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9212` |
| 35 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 37 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.40% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 41 | 0.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 42 | 0.39% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 43 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4352` |
| 48 | 0.34% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 49 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.32% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 51 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 52 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.30% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 55 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 57 | 0.30% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7179` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 59 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 60 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9122` |
| 7 | 2.37% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 2.10% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.97% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.93% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.90% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9114` |
| 15 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 16 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.98% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.90% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.78% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7056` |
| 25 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |

GC pauses: 0, total 0 ms

