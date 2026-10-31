# JVM hotspots (JFR ExecutionSample)

Samples: 13899

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 2.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 9 | 1.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 11 | 1.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.50% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 13 | 1.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 15 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.23% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 17 | 1.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 18 | 1.17% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 20 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 23 | 0.94% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 24 | 0.87% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.82% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4017` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.78% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4190` |
| 30 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 32 | 0.63% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 33 | 0.59% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.55% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 37 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 38 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8996` |
| 41 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.43% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 49` |
| 44 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 45 | 0.42% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 46 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4325` |
| 51 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 52 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4170` |
| 55 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 57 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.29% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 60 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 2 | 2.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 3 | 2.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 5 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 9 | 1.89% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 11 | 1.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 12 | 1.50% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 16 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 1.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 18 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 19 | 1.19% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4324` |
| 20 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 1.11% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 1.01% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3376` |
| 23 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 25 | 0.91% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 26 | 0.87% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.82% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6837` |
| 28 | 0.81% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 29 | 0.81% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 30 | 0.80% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4326` |

GC pauses: 0, total 0 ms

