# Как тикают энтити в Paper 1.21.8 — факты из исходников

Источник: реальные исходники `PaperMC/Paper` ветка `ver/1.21.8`
(патчи поверх декомпилированной ваниллы 1.21.8, mojang-маппинги).
Ниже — только то, что подтверждено чтением кода, без домыслов.

## 1. Цикл тика мира (ServerLevel.tick)

Каждый тик сервер последовательно (один поток!):

1. Погода, время, спавнер (natural spawning), чанки (light, random ticks).
2. **`ActivationRange.activateEntities(level)`** — Paper-патч, см. §3.
3. **`entityTickList.forEach(entity -> ...)`** — итерация по **ВСЕМ** энтити
   загруженных чанков (не только рядом с игроками!):
   ```java
   this.entityTickList.forEach(entity -> {
       if (!entity.isRemoved()) {
           if (tickRateManager... /* ванильные tick-rate проверки */) {
               this.tickNonPassenger(entity);
           }
       }
   });
   ```
4. Блок-энтити: `tickBlockEntities()` — см. §5.

## 2. tickNonPassenger — что происходит на ОДНОГО энтити за тик

```java
public void tickNonPassenger(Entity entity) {
    entity.setOldPosAndRot();
    entity.tickCount++;
    profilerFiller.push(() -> ENTITY_TYPE.getKey(entity.getType()).toString()); // профайлер — не бесплатно
    final boolean isActive = ActivationRange.checkIfActive(entity); // Paper EAR 2
    if (isActive) {
        entity.tick();          // полный ванильный тик
        entity.postTick();      // CraftBukkit (события и хвостовая логика)
    } else {
        entity.inactiveTick();  // дешёвый альтернативный тик
    }
    // + рекурсивный tickPassenger для пассажиров
}
```

Ключевое: **виртуальный вызов + профайлер-пуш на каждый энтити каждый тик**,
даже если энтити потом отбрасывается по активации.

## 3. Entity Activation Range 2.0 (Paper)

### activateEntities — скан активации
Каждый тик, для КАЖДОГО игрока:
```java
final List<Entity> entities = world.getEntities((Entity) null, ActivationRange.maxBB, e -> true);
for (final Entity entity : entities) ActivationRange.activateEntity(entity);
```
Это **O(N_энтити) на игрока на тик** — полная выборка мира через AABB-запрос.
При 100k+ энтити сам «активатор» уже заметная нагрузка (проверим бенчмарком).

Постоянно-активные типы (defaultActivationState = true): Player, EyeOfEnder,
ThrowableProjectile, EnderDragon, WitherBoss, AbstractHurtingProjectile,
LightningBolt, PrimedTnt, FallingBlock, Minecart, Boat, EndCrystal,
FireworkRocket, ThrownTrident. Маркеры — по конфигу `entities.markers.tick`.

### checkIfActive — решение «тикануть или нет»
- FireworkRocket и ItemEntity каждый 4-й тик (`(tickCount + id) % 4 == 0`) —
  **всегда активны** (гравитация предметов не выключается никогда).
- Новые энтити (< 200 тиков), порталы, поводок у игрока — активны.
- `entity.activatedTick >= currentTick` → активен.
- Неактивные: каждые 20 тиков проверка «иммунитетов» (`checkEntityImmunities`):
  огонь, вода/плавание, `!onGround`, горизонтальная скорость > 1e-5,
  стрела не в земле, лестница, прыжок, эффекты, `mob.getTarget() != null`,
  грудной ребёнок/любовь, подожжённый крипер и т.д. Иммунитет выдаёт
  окно активности на N тиков.
- Иначе — `inactiveTick()`: у мобов пропускаются goals/navigation/travel,
  у предметов почти всё.

### Вывод по EAR
EAR — это **уже готовый тировый планировщик Paper**, но:
1. Сканирование активации O(N) на игрока.
2. `checkIfActive` — тяжёлая ветвистая проверка на каждом энтити.
3. Иммунитеты пересматриваются вслепую каждые 20 тиков.
4. Физика предметов всё равно тикает (каждый 4-й тик обязателен).

## 4. Физика конкретных типов

### ItemEntity.tick (предметы)
- Гравитация: **0.04**/тик; трение: **0.98** (в воздухе), на земле
  `blockFriction * 0.98`; `move(MoverType.SELF, delta)` → полный
  collision-sweep против блоков каждый тик, когда предмет движется
  или каждый 4-й тик (см. `(!onGround || hSpeed^2 > 1e-5 || (tickCount+id)%4==0)`).
- Вода: плавучесть (восходящее движение).
- **Слияние (merge) — каждый тик у сливаемого предмета:**
  ```java
  getEntitiesOfClass(ItemEntity.class, getBoundingBox().inflate(radius, ..., radius), ...)
  ```
  `radius = spigotConfig.itemMerge` (по умолчанию 0.5).
  Это **AABB-запрос по всем энтити мира на каждый предмет на каждый тик** —
  O(N) поведение без пространственного индекса. При 100k предметов —
  главный кандидат на лавину (подтвердим JFR-профилем).
- Деспавн: `age >= 6000` (spigot despawn-rate).
- PickupDelay 32767 = «никогда не подбирается», и тогда предмет
  не сливается (`isMergable()` false) — скан слияния пропускается.

### LivingEntity (мобы)
Порядок в tick(): baseTick (огонь/лава/пустота) → `aiStep()`
(цели/мозг → input) → `serverAiStep()` → `travel()`
(трение, гравитация 0.08 для живых, moveRelative) → `move()`
→ **AABB-коллизии против блоков** + толкание других энтити
(`getEntities` вокруг бокса, Paper ограничивает
`max-entity-collisions`, по умолчанию 8) → головокружение/эффекты.
Плюс goalSelector/targetSelector тикают каждый тик у активных мобов.

### ExperienceOrb
Ищет игрока в радиусе притяжения каждый тик, гравитация 0.03,
слияние аналогично предметам. EAR даёт ему окно 20 тиков.

## 5. Блок-энтити (Level.tickBlockEntities)

```java
// blockEntityTickers — простой ArrayList на весь уровень
for (this.tileTickPosition = 0; tileTickPosition < blockEntityTickers.size(); tileTickPosition++) {
    TickingBlockEntity tickingBlockEntity = blockEntityTickers.get(tileTickPosition);
    if (tickingBlockEntity.isRemoved()) { toRemove.add(...); }
    else if (runsNormally && this.shouldTickBlocksAt(tickingBlockEntity.getPos())) {
        tickingBlockEntity.tick();   // виртуальный вызов на каждый BE каждый тик
    }
}
```

- **Полная итерация всего списка каждый тик** + чанк-проверка
  `shouldTickBlocksAt` на каждый BE.
- **Печка** (AbstractFurnaceBlockEntity.serverTick): проверка
  `isLit`, `canBurn` (рецепт-проверка через кэшированный
  `RecipeManager.createCheck`), `cookingTimer++` каждый тик при готовке,
  пересадка результата, событие Bukkit.
- **Воронка** (HopperBlockEntity): `tryMoveItems` каждый тик —
  вытягивание из инвентаря сверху, выталкивание вниз/в сторону;
  cooldown 8 тиков после успешного переноса; Paper имеет
  hopper-cooldown конфиг.
- 100k тайл-энтити = 100k виртуальных вызовов + проверок чанка каждый тик.

## 6. Что гипотетически «жрёт» время при 100k+ (проверяем бенчмарком)

| # | Подозреваемый | Где | Почему |
|---|---------------|-----|--------|
| 1 | Merge-сканы предметов | ItemEntity.tick | O(N) AABB-запрос/предмет/тик |
| 2 | Активационный скан | EAR activateEntities | O(N) на игрока/тик |
| 3 | checkIfActive | EAR | ветвистая логика на каждый энтити |
| 4 | Коллизии с блоками | Entity.move | sweep по блокам для движущихся |
| 5 | Виртуальный вызов tick() | tickNonPassenger | dispatch + профайлер-пуш ×N |
| 6 | Толкание энтити | LivingEntity.push | getEntities вокруг бокса |
| 7 | BE-итерация | tickBlockEntities | N виртуальных вызовов + чанк-чек |
| 8 | GC-давление | getEntities/AABB-выборки | списки-аллокации каждый тик |

Каждый пункт превратится в измеряемый факт после бенчмарка (JFR top frames).
