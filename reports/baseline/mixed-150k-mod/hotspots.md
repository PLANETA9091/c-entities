# JVM hotspots (JFR ExecutionSample)

Samples: 26789

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6203` |
| 5 | 3.28% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9806` |
| 6 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5435` |
| 9 | 1.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9331` |
| 11 | 1.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 1.04% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.88% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 21 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 24 | 0.72% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 25 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9322` |
| 28 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9751` |
| 29 | 0.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 30 | 0.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 32 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 33 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.47% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 35 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4028` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.44% | `java.util.Objects.requireNonNull(Object) line: 219` |
| 39 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 40 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 43 | 0.41% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 44 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4374` |
| 46 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.37% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 48 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 53 | 0.34% | `java.lang.ref.Reference.get() line: 361` |
| 54 | 0.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7233` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9317` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9424` |
| 5 | 3.29% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 3.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9321` |
| 7 | 2.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.50% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.94% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.89% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 1.06% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 15 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9414` |
| 17 | 0.95% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 18 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.84% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7110` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 24 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.75% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6213` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.57% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms

