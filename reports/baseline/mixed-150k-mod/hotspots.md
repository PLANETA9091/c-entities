# JVM hotspots (JFR ExecutionSample)

Samples: 33134

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 4 | 3.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 5 | 3.35% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 3.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 11 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 13 | 1.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 16 | 1.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.75% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 31 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 32 | 0.61% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 37 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 40 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 47 | 0.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 51 | 0.34% | `java.lang.ref.Reference.get() line: 361` |
| 52 | 0.34% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.32% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 54 | 0.32% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 55 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 59 | 0.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 6 | 2.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 7 | 2.17% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.08% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 2.07% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 2.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.95% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 13 | 1.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 14 | 1.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.25% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 16 | 1.24% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.17% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 21 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.89% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 23 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.87% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6718` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 28 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.67% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

