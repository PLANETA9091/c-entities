# JVM hotspots (JFR ExecutionSample)

Samples: 31611

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 3 | 3.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 4 | 3.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 13 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.99% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 22 | 0.96% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.75% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8922` |
| 28 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 32 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8797` |
| 39 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 40 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 41 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 42 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 44 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 50 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 54 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 56 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 57 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.33% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 6 | 2.12% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.01% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 1.99% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.93% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.91% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.81% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 13 | 1.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 15 | 1.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 16 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 18 | 1.09% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 0.99% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.84% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6766` |
| 26 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 27 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

