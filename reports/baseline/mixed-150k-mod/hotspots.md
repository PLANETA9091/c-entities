# JVM hotspots (JFR ExecutionSample)

Samples: 34401

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 3 | 3.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 4 | 3.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9267` |
| 7 | 2.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5995` |
| 10 | 1.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 13 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 17 | 0.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.83% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.66% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 31 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9212` |
| 34 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9087` |
| 36 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.42% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 41 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 42 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 46 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 54 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 55 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 56 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 58 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.92% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9122` |
| 7 | 2.18% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.97% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9139` |
| 13 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9114` |
| 14 | 1.45% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 16 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.86% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 23 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.66% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |

GC pauses: 0, total 0 ms

