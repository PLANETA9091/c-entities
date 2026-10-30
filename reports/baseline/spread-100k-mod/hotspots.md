# JVM hotspots (JFR ExecutionSample)

Samples: 34684

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 2 | 2.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 5 | 2.74% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.57% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 7 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 11 | 1.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 15 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.93% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 22 | 0.92% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 25 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 30 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.71% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.70% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 33 | 0.68% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 34 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 35 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 36 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 37 | 0.61% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.59% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 39 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.54% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 42 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 47 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3773` |
| 52 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8567` |
| 55 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 56 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 57 | 0.39% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 58 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8442` |
| 59 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6534` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8477` |
| 7 | 2.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.87% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.46% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8494` |
| 14 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 16 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.06% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.93% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |
| 20 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 22 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2435` |
| 23 | 0.83% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 28 | 0.77% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 29 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |

GC pauses: 0, total 0 ms

