---
Task ID: 1
Agent: main (Z.ai)
Task: c-entities с нуля: исследование Paper 1.21.8 + базовый бенчмарк-пайплайн на GitHub Actions

Work Log:
- Удалён весь ранее написанный Rust-код (по требованию владельца) — старт с нуля
- Клонированы исходники PaperMC/Paper ветка ver/1.21.8, изучены патчи:
  ServerLevel (tickNonPassenger/EAR), ActivationRange 2.0 (activateEntities O(N)/игрок,
  checkIfActive, иммунитеты), ItemEntity (гравитация 0.04, merge O(N)/тик), Level.tickBlockEntities
- Написаны docs/research/01-03: пайплайн ванили, план бенчмарка, точки инжекта CRUSSTY
- Создан измерительный плагин ce-baseline (Maven, paper-api 1.21.8): chunk tickets lvl 31,
  плавный fill, MSPT через ServerTickEndEvent, JSON-дамп, самовыключение
- Скрипты: fetch-paper.sh (fill API v3), build-plugin.sh (Maven), run-baseline.sh, bot.js (mineflayer),
  jfr_top.py (топ фреймов JFR), make-report.py, summarize.py
- .github/workflows/baseline.yml: матрица 9 прогонов (control/items/mobs/mixed/tiles), публикация в reports/baseline
- Репо создано через API, запушено; фиксы: mkdir для paper.jar, Maven-лейаут + env-перегрузки

Stage Summary:
- Репо: PLANETA9091/c-entities, main, ветка чистая
- Базовый профиль ванили будет в reports/baseline/SUMMARY.md после зелёного прогона
- Инжект-цели для модуля: ServerLevel.tickNonPassenger, ActivationRange.activateEntities/checkIfActive,
  Level.tickBlockEntities

---
Task ID: 2
Agent: main (Z.ai)
Task: Анализ baseline-результатов (9 прогонов) + findings doc + старт фазы модуля

Work Log:
- Проверил состояние: старый Rust-код удалён ранее; harness собран; workflow отработал (run 39150947858 success, 9 сценариев)
- Скачал все 9 артефактов, разобрал REPORT.md/baseline.json/hotspots.md
- Скрипты jfr_callers.py / jfr_phases.py: разрешены цепочки вызовов из rec.jfr (JDK21 jfr установлен в ~/tools)
- Ключевые находки: AABB.intersects 25.4% (mobs-25k) / 42.7% (mixed-150k) CPU; 100% через ChunkEntitySlices.getEntities; 73% = Level.getPushableEntities ← LivingEntity.pushEntities; item merge-скан каждый тик (док 01)
- Квадратичный закон: mixed N^2.07, mobs N^1.96 по плотности; предметы ~1.05 мкс/энт, зомби 10-19.6 мкс/энт, тайлы 0.28-0.40 мкс
- Починен summarize.py (рекурсивный glob), заполнен reports/baseline/SUMMARY.md (9 строк)
- Написан docs/research/04-baseline-findings.md: числа, квадратичный закон, ranked levers L1-L6
- Git: конфликт паблиш-коммитов разрешён (remote уже имел отчёты), запушено (75b/789... успешный push)

Stage Summary:
- BASELINE ЗАФИКСИРОВАН: control 0.16ms; items-100k 65.8ms; mobs-25k 488.9ms; mixed-150k 909.7ms; tiles-100k 27.5ms
- Главная мишень L1: Rust spatial index для entity->entity запросов (push + merge) — убирает квадратику и 30-45% CPU
- Следующий шаг: docs 05 (дизайн модуля) + Cargo workspace (centities-core: точный XZ-grid индекс + тесты; centities-module: cplug-sdk weave)

---
Task ID: 3
Agent: main (Z.ai)
Task: centities-core v0.1 — ядро точного индекса (Rust)

Work Log:
- Workspace + crates/centities-core: XZ hash-grid (cell 64), single-home по центру AABB, cell-relative f32 (точность на ±30M блоков), open-addressed CellMap/IdMap с backward-shift, oversize-лист, no unsafe, no deps
- Тесты: property exactness против brute-force (20k ops), boundary-кластеры, remove-storm, oversize переходы, Far Lands, buffer reuse; инвариант-чекер (doc(hidden))
- Отловлены и исправлены баги: rechain без return после head-fix; ошибочный free-list (swap-remove сразу занимает слот); КЛЮЧЕВОЙ: cell_key(world0,0,0)==0 конфликтовал с sentinel пустого слота — bias 2^20, ключ теперь никогда 0
- CI: .github/workflows/module.yml (fmt/clippy/test debug+release)
- Запушено: 98367d7

Stage Summary:
- centities-core: 11 тестов зелёных локально
- Дальше: crates/centities-module на cplug-sdk (weave getEntities → CeHooks шим → JNI ceQuery/ceUpsert/ceRemove), e2e, A/B benchmark MODULE=on/off

---
Task ID: 4
Agent: main (Z.ai)
Task: centities-module v0.1 — CRUSSTY-модуль (weave + JNI) на базе данных baseline

Work Log:
- Изучен cplug-sdk: hooks::register_bytes (ClassFileLoadHook), asm::replace_body (ASM9, COMPUTE_FRAMES, op1/op2), weave.rs (insert_call_at_start без аргументов), classes/jni_util (with_attached, define_class, register_natives), cplug-abi (CPluginApi v3, claim-ключи)
- Декомпилирован Paper 1.21.8 (merged jar): Level.getEntities(Entity,AABB,Predicate) = 4 строки (moonrise$getEntityLookup().getEntities + PlatformHooks.addToGetEntities) — точное зеркальное fallback возможно
- CeHooks.java (мост): getEntities (fast path + vanilla-mirror fallback + kill-switch), probeAdd/probeRemove/probePos/probeBox (префикс-зонды), setActive, нативы с AABB-даблами в аргументах (ноль JNI field reads)
- patch.rs: свой билдер префикс-хуков С АРГУМЕНТАМИ (aload/lload/dload + invokestatic, stack-neutral), фиксы: StackMapTable (первый кадр +delta), exception table, LNT/LVT, отказ на switch/wide; тесты на РЕАЛЬНОМ Level.class (структура + двойной патч)
- lib.rs: cplugin_init → byte-хуки на 3 класса → on_kernel_ready(ServerLevel) → define CeHooks в загрузчик Level + RegisterNatives (claim) → retransform_class → ARM (CeHooks.ACTIVE=true) только если все патчи легли
- Баги по пути: aload 0x2a vs 0x19 (поймано тестом), attr_len_pos offset, Option<Option<Vec>>, CP count, zero-key cellmap (core) — всё зелёное
- CI: module.yml — core tests + module job (paperclip boot → PAPER_JAR → cargo test + release cdylib + bundle artifact)
- Запушено: 0671504 (+ 9368379 gitignore чистка зипов)

Stage Summary:
- Модуль v0.1 собран: 5 weave-точек, kill-switch на каждом уровне, fallback = точное зеркало ванили
- Тесты: core 11 + module 2 — зелёные; сборка cdylib успешна локально
- Следующий шаг: e2e на CRUSSTY (модуль в modules/, Paper 1.21.8 boot, проверка ARM + дифференциальный тест зеркало==ванилла), затем A/B бенчмарк MODULE=on/off в baseline-матрице

---
Task ID: 5
Agent: main (Z.ai)
Task: Парity A/B — анализ провала (run 40529103630) + L37 order-exact фикс

Work Log:
- Разобрал логи упавшего parity: mobs leg — 14 type mismatches (ZOMBIE↔CHICKEN, chicken-jockey от finalizeSpawn RNG), 8 лишних entity в on-leg; items — позиции бит-в-бит, vx/vy расходятся (1e-14/0.04), 143/95 разных merge-выживших
- Корень (задокументирован в самом коде!): (a) merge fast-path «candidate ORDER may differ from vanilla» — tryToMerge держит ПЕРВОГО кандидата в ванильном порядке скана; (b) push kernel выдавал кандидатов в entity-id порядке + maxEntityCollisions cap оставляет первых N ПО ПОРЯДКУ — бродящие зомби переаппендятся в секции, id-порядок ≠ ванильному
- Декомпилировал EntityLookup/ChunkEntitySlices/ActivationRange/ServerLevel/ItemEntity/Entity (26.3): точный порядок = regions Z,X → chunks Z,X → sections Y → insertion seq (BasicEntityList add=append, remove=arraycopy, moveEntity re-append); tickCount++ живёт в commonTick (каждый тик!); ItemEntity: %4 исключение активации + 1-из-20 temporary window; L21 rest inline path — мёртвый код (gravity ломает fixed-point)
- L36 idle guards (nav/move/look) верифицированы против декомпиляции — точны
- Реализован L37: ORD_SEQ (probeAdd + НОВЫЙ probeMoveSection weave на EntityLookup.moveEntity — обязателен для ARM), arm-backfill через moonrise$getEntityLookup().getEntities(full-box) (= ванильный порядок), ordSortVanilla на fastQuery+rowQuery, push kernel: cell walk chunkZ-major + sort_cell_by_keys по (chunkZ, chunkX, sectionY, seq), sy/seq через батч (B_SY/B_SEQ → nativeFlush)
- Снапшоты parity: добавлены tc (tickCount) / at (activatedTick) per entity; parity-compare.py --dump → компактный артефакт дивергенции (upload при failure)
- Пойманы 2 бага при написании: JNI-дескриптор nativeFlush (11→12 групп), Rust precedence `>> 4 + bias` (скобки)

Stage Summary:
- Push: 072dd43 на main; CI (module/parity/ab-paired/e2e/baseline) запущен
- След.: парсить parity-dumps при новом прогоне; аудит L15 GoalSelector floor; item core vy-фаза по данным dumps; цель — parity PASS затем гнать скорость к -90%

---
Task ID: 6
Agent: main (Z.ai)
Task: CI-итерации L37 — три багфикса по правилу «ошибки сразу»

Work Log:
- Прогон 072dd43: e2e/module упали (a) тесты pushcore звали старый upsert (9 vs 11 аргументов) — адаптированы; (b) natives_parity — захардкоженная таблица дескрипторов не знала о sy/seq — обновлена
- Прогон 7cd27ba: e2e «module not armed» + parity «FATAL: module did not ARM» — ordBackfillWorlds звал getEntities с full-world box (±30M) → ~3.5 млрд итераций регионов → поток ARM завис навсегда. Фикс: per-REGION боксы из Bukkit getLoadedChunks (внутри региона порядок = ванильный, меж-секционный порядок из живых moonrise-полей)
- Прогон 2d2959b: e2e ✓ module ✓; parity — все 8 ног прошли, но гейт exit 2 и НОЛЬ parity-файлов: мой диагностика-код брал getMethod("getHandle") у org.bukkit.entity.Entity (интерфейс!) — NoSuchMethodException молча убивал каждый снапшот; ls без файлов под pipefail → exit 2 без объяснений. Фиксы: bind от runtime-класса первой сущности; гейт толерантен к отсутствию файлов и ПЕЧАТАЕТ каких снапшотов нет
- Аудиты в ожидании: L13 tickList raw walk — точен; L15 fullPassTick — условия/порядок/canUse-последним — точны; L22a mobAiFloor — RNG round-trip структурно верен (ThreadUnsafeRandom = одно 48-бит LCG-слово, aiPack перечитывает seed)

Stage Summary:
- f46d0ae в полёте (parity + baseline); e2e/module стабильно зелёные на 2d2959b
- След.: разобрать parity-dumps артефакт (tc/at диагностика покажет точку расхождения), затем добить скорость до -90%

---
Task ID: 7
Agent: main (Z.ai)
Task: L37 parity-охота — 6 корней найдено инструментом trace/orderCheck, 5 исправлено

Work Log:
- Bisect-матрица через dispatch knobs (CE_ITEM_TICK/CE_MOB_AI/CE_MERGE/push/ai_floor):/items PASS при item_tick=0+merge=0 → item core; мобы расходятся даже со всем ванильным
- ORDER-CHECK (-Dcentities.orderCheck=true): fastQuery сверяется с зеркалом поэлементно → items n_van=0 n_fast=1-2 — SET-дивергенция! КОРЕНЬ: EntityIndex.query — MARGIN-СУПЕРМНОЖЕСТВО (расширение max_half_span+MARGIN), fastQuery доверял индексу без пер-кандидатной AABB-проверки → merge с предметами за радиусом + push вне бокса. ФИКС: точная AABB-проверка (marginExtra счётчик). После фикса: 0 ORDER-MISMATCH во всех ногах
- Пер-тиковый PARITY_TRACE (hex-строки, tc/at с первого тика): items id 2 t=342 — vanilla move-ветка, модуль — rest-режим (счётчики без физики!): fixed-point РЕАЛЕН на %4-тиках (коллизия обнуляет гравитацию) → REST отключён (L37f). Ранее: Y_DRAG 0.98 (f64) → 0.98f32 as f64 (ULP за тик)
- mobs id 1 t=207/162: crammed push (d0<0.01) — ядро молча пропускало силу+2 RNG-дроа → двухпроходное ядро (detect-проход без мутаций → crammed пара маршрутизирует весь тик в ваниллу). Плюс jockey-флипы сократились 14→6
- Пойманы и исправлены попутно: GitHub falsy-"0" в inputs, natives_parity хардкод-таблица, full-world box backfill (ARM завис), JNI-дескриптор, тесты под новые контракты

Stage Summary:
- Паритет: items vy-фаза исправлена (остатки 1e-15→1e-42), мобы 6 флипов; осталось: 3-тиковый lag store.tick_count vs JAVA у 12 предметов (фаза %4 инвертируется), goal-start draws в L22a (yaw-старт у мобов t=162-204)
- Скорость (2d2959b): items -80.4%, spread -67.3%, mixed -42.2%, mobs -54.1% (порядок-фикс УСКОРИЛ)
- След.: ① tick_count lag — трейс стора (sTC в trace), ② L22a goal-start — сверить RandomLookAround/LookAtPlayer draws с goals.rs, ③ после зелёного parity — гнать -90%

---
ВОССТАНОВИТЕЛЬНЫЙ БЛОК #2 (записан в Task 22, 2026-11-08 ~07:50+08:00): второй откат снапшота песочницы — worklog снова обрезан по Task 7 (15118 байт, старая эпоха L22a). Tasks 8–21 восстановлены компактно ниже; с этого момента канонический worklog зеркалится в GitHub-ветку `journal` (переживает рециклинги; локальная копия = кэш).
- Tasks 8–17 (CI-инфра и swarm): watchdog RUN_TIMEOUT 3600; rerun по ветке; noise ±5.5 пп (items-контроль — СЛИШКОМ узко для mobs/mixed, см. Task 21); L103 goals-draws; L104 bigbox-refuse (3d1a076, merged e1fc0db: запросы >1024 строк отказываются от fast path в точный ванильный body); swarm-sim v0.1/v0.2 (6949272, 4720fcc): native full-tick executor, WALK 5.24 ms/tick (x9.5 запас), FARM 27.9 (x1.8), RAM 1.7 ГБ @ 1e9 зомби; Rust standalone в ~/tools/rust185 (фоновые install не выживают — ставить синхронно).
- Task 18 (жатва 42396574161 "353", L104b): вердикт — регрессий нет, spread dAB −8.5 пп реален; локально FARM 19.81 (x2.5), WALK 4.85 (x10.3); publish non-fast-forward → publish-fix b6cfa96 (`git push origin HEAD:main`); прицел L106: query-семейство ~34.5% self-time живых мобов (getBoundingBox 11.11 + fastQuery 10.01 + rowQuery 8.63 + ord* 4.65 в 58857-сэмпловом профиле).
- Task 19 (цикл 04:00, L106 @ l106-query 12a0767): query-family pack — структурный ordSkip (Rust-скан chunkZ-major = ванильный порядок по построению; ordKey+prescan снесены), view hoist (сеть vBlew), counter locals, inline intersects; CE_L106=0; diag l106=<ordStructSkip>/<mViewBlew>/<on>; отложено L107: f64 row-боксы в строках + Rust-скан строк, 214 дрейф-кейсов freshness.
- Task 20 (жатва паритета L104b 42409887277): SUCCESS, PARITY-A-B-GATE: PASS, 12 гейтов bit-identical, снапшоты T389/T639/final (f4ae8d8b…, 8a14a2f2…, 66bec95a…); PushChecks 452519, DispChecks 4686128; паритет L106 42410762954 тоже SUCCESS.
- Task 21 (жатва baseline L106 42410764735 + ПУШ): L106 diag — ordStructSkip 21.4M–44.8M, mViewBlew=0, on=1; профиль spread-мод: ordSortVanilla/ordKey/isViewAfter ИСЧЕЗЛИ (≥2.7 пп удалено), getBoundingBox топ-1 10.84%; исправление данных: «mixed +45.8» Task 18 = ошибка экстракции, реальные ноги 353 mixed = 859.24/589.23 = −31.4%; межрановая волатильность ног ±35–44% (mobs 580.9→353.3→527.5 vanilla), dAB-дельты между ранами внутри этого шума; ПУШ: merge-batch @ 789b78c = main(1373619+fe741a6) + merge 031549b, тесты 81/0, окно чистое, FF-пуш подтверждён; подняты 4 рана merged-main.

---
Task ID: 22
Agent: main (Z.ai)
Task: жатва CI merged-main @ 789b78c (cron 513336): паритет-гейт + третий full-matrix datapoint

Work Log:
- Все 4 рана merged-main @ 789b78c SUCCESS (attempt 1): parity 42420738476, baseline 42420738528, module 42420738518, e2e 42420738486.
- ПАРИТЕТ MERGED-MAIN: PARITY-A-B-GATE: PASS, 12 гейтов bit-identical — ТРЕТИЙ PASS стека (L104b standalone → L106 standalone → merged). Interaction-гипотеза (ordSkip×push-ядро) опровергнута: merged-дерево бит-в-бит.
- Baseline merged (8 артефактов): dAB внутрирановое — items −53.2%, mobs +33.5%, spread +32.5%, mixed +35.7%. Триплан ранов (353 → L106 → merged): items −59.4/−57.2/−53.2, mobs +24.6/+32.0/+33.5, spread +31.9/+37.0/+32.5, mixed −31.4(аномалия)/+36.2/+35.7.
- Профиль spread-мод @ 789b78c: getBoundingBox 10.64% (топ-1), fastQuery 7.43, rowQuery 3.12, Level.getEntitiesOfClass 2.56, SynchedEntityData.get 2.40, vtabHash 2.36, vtabGet 2.36, CeHooks.getEntitiesOfClass 2.29; ord* ОТСУТСТВУЮТ (структурное удаление стабильно между ранами). Профиль идентичен L106-рану (10.84/7.44/3.26) — воспроизводимо.
- Стабильные сигналы (код-атрибутируемые): паритет ×3, ord*=0 ×2 рана. Нестабильные: dAB-дельты между ранами (ноги ходят ±35–44%); дисциплина вердиктов — профиль + абс внутри рана, dAB только внутрирановый.
- Worklog-зеркало: создана ветка journal на GitHub (worklog.md), обновляется после каждой записи — канон против откатов снапшота.

Stage Summary:
- main @ a2e66e0 = 789b78c (полный стек L104+L104b+L106+publish-fix) + results merged-baseline; паритет гейта merged-main PASS.
- Оставшийся налог живых мобов (spread-мод профиль): query-семейство ~30% self-time — getBoundingBox 10.64 + fastQuery 7.43 + rowQuery 3.12 + getEntitiesOfClass 4.85 + vtab* 4.72 + SynchedEntityData 2.40.
- СЛЕДУЮЩИЙ ФРОНТ L107: f64-боксы строк + Rust-скан строк (отказ deref getBoundingBox на отбраковке кандидатов; survivors only → Java), freshness-штампы против 214 дрейф-кейсов; далее vtab*-локали и SynchedEntityData.get (читается где? — локализовать в L107-JFR). Цель −90% на живых мобах остаётся главной.

---
Task ID: 23
Agent: main (Z.ai)
Task: L107 имплементация — exact-f64 row boxes + Rust fresh filter, ветка l107-rowboxes @ 20e6f6f

Work Log:
- Прицел (789b78c профиль): Entity.getBoundingBox 10.64% топ-1 — L37d live-box re-check deref'ит КАЖДОГО кандидата, а отбрасывает post-L60 только 0.00003% (marginExtra 214/662M, ран 352). Причина: индекс хранит f32 cell-relative боксы; точный f64 бокс УЖЕ едет в каждом batch-entry (B_BOX) — его просто не хранили.
- РЕШЕНИЕ: (1) EntityIndex: slot_bb ([f64;6] absolute) + slot_tick (штамп зонда) на каждом upsert_ord; swap-remove вместе со слотом; clear(). (2) query() получает tick и на свежих слотах (штамп == тик запроса) гонит ТОЧНЫЙ 6-сравнительный AABB.intersects как стабильную in-place компакцию ПОСЛЕ oversize-merge: точный промах не доходит до Java вообще (deref умер), хит уходит с FRESH_BIT (1<<40) — Java скипает re-check (ванильный predicate остаётся). Stale-слоты/oversize/tick<=0/push-kernel query_cell не тронуты. (3) JNI: queryNative/queryNative2/nativeFlush несут per-entry B_TICK + query tick; off-thread nativeBoxDirect = TICK_UNKNOWN. (4) Java fastQuery: rawId/cid сплит, FRESH_BIT-скип для свежих; vActive-окна (viewBox) — полный legacy; rowQuery не тронут (L107b surface).
- Контракт корректности: свежий штамп = зонд записал live box, flush-merged apply лёг до скана, однопоточный тик-цикл не мутирует между flush и сканом → точный тест = то же решение, что принял бы ванильный trim. Паритет-рефери; ORDER_CHECK цел (компакция сохраняет порядок — аргумент L106).
- Knob CE_L107=0 возвращает всё (query tick=0); diag l107=<fresh>/<legacy>/<on> на hooks@.
- Локально: cargo test core+swarm 82/0 — новый fresh_filter_exact_decides_this_tick_only (fresh/stale/re-probe/slot-reuse); javac (JDK21 из /tmp/my-project/work) — 58 ошибок, ВСЕ в известных чужих зонах (fastutil/paper-internals), зоны правок чисты.
- CI: parity 42430389700 + baseline 42430395165 задиспатчены на l107-rowboxes (shipped config, полный матрикс); TTL-harvest cron 513542.
- Попутный фикс: тест exactness/simd_parity/examples переведены на query(4-арг); TICK_UNKNOWN импорт (ветка компилируется).

Stage Summary:
- Ожидание: getBoundingBox 10.64% → вниз на долю stale-кандидатов; l107Fresh крупный (в плотных стопках большинство кандидатов зондируется текущим тиком), l107Legacy малый; mobs/spread/mixed внутрирановый dAB улучшается выше шума.
- Дальше: при PASS обоих гейтов — мерж-пачка в main (чистое окно); L107b = f64-боксы в published rows (rowQuery-путь, ~3.12% + его доля getBoundingBox); затем vtab*/SynchedEntityData.get (4.72%/2.40%).

## Task 24 — L107 harvest: вердикт FAIL, main откачен к a2e66e0 (2026-11-08)

CI-пара на l107-rowboxes (20e6f6f): parity 42430389700 = SUCCESS (четвёртый бит-в-бит PASS
стека — fresh-контракт корректен), baseline 42430395165 = SUCCESS по джобам, но вердикт по
профилю и внутрирановому dAB — **FAIL**:

- Внутрирановый dAB (mod vs vanilla, шум машины сокращается): items −48.9%, spread **+99.4%**,
  mobs **+157.1%**, mixed **+123.0%** — против референса 42410764735 (items −57.2/mobs +32.0/
  spread +37.0/mixed +36.2). Регрессия далеко за пределами межрановой волатильности ±35–44%.
- Профиль spread-мод: getBoundingBox 10.72% (было 10.64%) — ЦЕЛЬ НЕ ДОСТИГНУТА; fresh-путь
  покрыл лишь 15.9M из 657M кандидатов (2.4%), legacy 641.5M — стейл-строки не зондируются
  текущим тиком, перепроверка getBoundingBox осталась у 97.6% кандидатов.
- Root-cause слой (по инструментам рана): нативный fast-query. fastNanos (полное покрытие):
  spread 468.8s→1005.9s (+114%), mobs 187.8s→1031.2s (+449%), mixed →2157.9s. qMs/resMs —
  сэмплированные 1-из-1024: натив spread 12.2→40.1 мкс/вызов (×3.3), mobs 385→1757 мкс/вызов
  (×4.6) при ПЛОСКОЙ нагрузке (cand ±1.8%, box ±4%, flush ±3%, mirror ±6%) — рост чистой
  цены вызова, не объёма работы. Vanilla-ноги того же рана: ставки JFR-сэмплов ±3% — машина
  чиста. JFR ExecutionSample не видит роста (fastQuery 10.82% top-1 при 40%−runnable) —
  поток в JNI (thread_in_native не сэмплируется), потому профиль и выглядел «здоровым».
- Локальная изоляция (bench 100k spread-like): фильтр query() = +108–145 нс/запрос, upsert
  с slot_bb/slot_tick = ±0% — сам Rust-код диффа микроскопичен. Расхождение с CI ×20–40 —
  кэш-холодные случайные достпупы idmap.find+slot_tick+slot_bb на каждого кандидата (моя
  бенч-таблица была тёплой) объясняет spread (×2–3), но НЕ mobs (×4.6, out/call упал 300→223);
  честная гипотеза для следующего цикла: дропы exact-miss (не считаются ни одним счётчиком!)
  раздувают pre-filter out у pile-запросов + interaction с bigbox--refuse-сканами 28.2M
  кандидатов/тик. Диагностический пробел: счётчик raw-out и дропов в Rust-фильтре.
- Действия: main @ 8bffddd содержал f7155ef (publish уложил) → revert 7b00297 (крейты
  бит-в-бит = a2e66e0, reports-коммит сохранён), локально cargo test 81/0. Пуш в main —
  чистое окно; автотриггер baseline на main валидирует откат.
- Выводы в стек: (1) fresh-контракт «штамп == тик запроса» корректен (паритет), но
  бесполезен по покрытию — стейл-строки (не зондируемые в этом тике) дают 97–98% кандидатов;
  (2) фильтр ПОСЛЕ скана на каждом кандидате через хэш-таблицу — архитектурная ошибка
  стоимости; решает только фильтрация ВНУТРИ SIMD-скана (cell_box уже горяч в кэше) или
  f64-боксы прямо в cell SoA (L107b-дизайн пересмотреть: exact-боксы в scan_cells);
  (3) JNI-вызовы не видны JFR — только hooks@-таймеры (fastMs/qMs) дают истину; keep в
  дисциплине вердиктов: fastMs/qMs × workload-счётчики обязательны к каждой жатве.
- Дальше: L107b (exact-f64 в SIMD-скане, без per-candidate хэшей) — прицел: getBoundingBox
  10.64% + fastQuery-хвост; затем vtab*-локали (4.72%), SynchedEntityData.get (2.40%);
  dormancy far-field под F1–F6 остаётся терминальной целью −90% на живых мобах.

## Task 29 — L110: the native pivot (Horizon study → zero-Java entity executor) (2026-11-08)

- ДИРЕКТИВА ВЛАДЕЛЬЦА: перестройка всей архитектуры ускорения — изучить Horizon
  (github.com/horizon-rust/horizon, Minecraft 26.3 в Rust), перенять портирование
  энтити на Rust: НОЛЬ джавы в энтити, затем профайлер по RUST-функциям.
- Horizon изучен (клон horizon-study, MIT; ~600 дифференциальных харнессов):
  AoS Vec<Mob> ~3KB + FxHashMap id→slot + per-tick Seen-снапшот + 4.0-block XZ
  SeenGrid (candidates = superset); плоский Attrs вместо AttributeMap; O(1)
  BlockFacts-строки; одиночный тик в ванильном порядке; события отложены в
  outbox, дрейн раз в тик; встроенный профайлер (sample 1-in-17).
- НОВЫЙ КРЕЙТ centities-native (ветка l110-native, мерж @ 4f01f54):
  MobStore (персистентный AoS-регистр в ванильном порядке + tombstone-компакт),
  SeenGrid (порт Horizon, superset-гейт в тесте), WorldRes (targeting/lookat из
  РЕАЛЬНОГО снапшота игроков O(P); BlockFacts-шов v0=OpenGround — N1 подложит
  секции), NativeTick (grid→pack→approach→AI-цикл→travel→компакт, один вызов
  на тик — JNI-трафик с per-mob-per-round схлопывается до нуля),
  профайлер фаз (ai/travel/grid/apply, 1-in-17, atomics).
- swarm-sim::Resolver: +2 опциональных предиката (lookat_query/find_target,
  default false — семантика симуляции не тронута); floor сам выбирает цель
  (ACT_REF_SET_PLAYER → target+approach) — цепочка таргет-джейла работает
  end-to-end: 3080/25000 мобов в chase в bench.
- BENCH: 25k зомби + игрок, весь тик в Rust: p50 8.79 / p90 9.48 / p99 12.28
  ms/tick; фазы ai 6.10 / travel 2.12 / grid 2.06 / apply 0.11. Референс:
  модуль ~242–260, ваниль ~494 (doc 41) — ~28×/56× архитектурный запас.
  Бит-в-бит детерминизм по seed (тест). 9/9 тестов крейта; гейт workspace:
  16 наборов зелёных (core/native/swarm-core/swarm-sim).
- ПРОФАЙЛЕР: scripts/rust_perf_top.py (perf cycles:u → TOP-40 RUST-символов
  по self-time, фоллбек в фазовый леджер) + workflow native-profile.yml
  (summary + artifact). Первый диспетч 42489377148. С этого момента каждое
  оптимизационное решение цитирует RUST-функцию (урок L107 про слепую JNI-зону
  больше не применим — исполнитель Rust end-to-end).
- МЕХАНИКА: workflow_dispatch требует workflow на default-ветке → мерж L110 в
  main в чистом окне (аддитивен: шиппед-конфиг не тронут; rebase поверх
  publisher-коммита 366ee31; пуш 4f01f54). Автопачка main @ 4f01f54:
  parity 42489346664 / baseline 42489346666 / module 42489346842 / e2e
  42489346854 — в работе.
- ПАРИТЕТ+BASELINE НА ШИППЕД-КОНФИГ (жатва Task 28-валидации): parity
  42481280688 PASS (7-й бит-в-бит) + baseline 42481280704 SUCCESS — конфигурация
  main (L104+L104b+L106+L109) полностью валидирована ДО мержа L110.
- СЛЕДУЮЩИЕ ФАЗЫ (doc 42 §6): N1 BlockFacts из секций (отмена OpenGround);
  N2 мост L111 CE_NATIVE_MOB=1 (Java = COPY-слой: add/remove diff + игроки +
  подписки секций, один JNI-вызов nativeMobBatch на тик, обратное применение —
  Java-тела нативных мобов не стартуют вовсе); N3 event-outbox (звуки/дропы/
  деспавн); N4 parity-транскрипты на границе исполнителя.

## Task 30 — L110 N1: SectionFacts (реальный BlockFacts) + profiler в CI зелёный (2026-11-08)

- facts.rs: SectionFacts реализует шов BlockFacts ПОВЕРХ секционного кэша
  item-пайплайна (centities_core::sections::SectionCache): path-type-0
  контракт — коллайдер земли (блок-ОТНОСИТЕЛЬНЫЙ y1 + база блока) достаёт
  плоскость стояния, ноги/голова без коллайдеров, unloaded-визиты отказ.
  10/10 тестов крейта (плоский мир + стена; на топе стены стоять ВАЛИДНО —
  тест это фиксирует). Java-мост уже собирает эти секции для предметов —
  мобовый тик переиспользует ту же grid: НОЛЬ JNI в цикле фактов.
- native-profile CI: два урока — (1) workflow_dispatch требует workflow на
  default-ветке (мерж L110 в main), (2) workspace-манифест требует
  ../CRUSSTY (клон pinned c315a1b — как в module.yml/e2e.yml). Третий запуск
  42490323137 SUCCESS: артефакт native-profile-42490323137 (на раннере без
  linux-tools — ledger-фоллбек: p50 9.18 ms/tick @ 25k, фазы ai 6.82/travel
  1.83/grid 1.48; на раннере с тулзами perf-путь работает — парсер исправлен
  после первого live-прогона).
- Валидационная пачка main @ e61e688 в работе (parity/baseline/module/e2e);
  e2e @ 4f01f54 уже SUCCESS. Жатва — крон/следующий тик.

## Task 31 — L112 wave2 arithmetic: ноль аллокаций, инкрементальный грид, честные часы (2026-11-09)

- Директива «ещё жоще с аритметиками»: 8 ударов по нативному хот-пути
  (exec/mob/store/profile), все bit-identical. Бенч 25k×400: p50 8.06 →
  4.9–5.1 ms/tick (−38%), p99 11.4 → 10.5, chasing 3036 == 3036.
  Док 43: docs/research/43-l112-wave2-arithmetic.md.
- Часы: PhaseGuard платил Instant::now за КАЖДЫЙ scope (≈2 ms/tick
  накладных внутри замера) → Option<Instant>. Грид: full-rebuild (25k
  hash-вставок/тик) → инкрементальный ре-бакетинг (Horizon N1), 1.81 →
  0.75 ms; strength reduction div_euclid(4.0) → *0.25 (bit-exact, свип-тест).
  Пак: zero-alloc pack_into + shared scratch; out-буфер: 9-словный сброс
  вместо 3KB memset (дисциплина записи пола верифицирована; гейт
  reused_out_buffer_matches_fresh_zero — близнецы, грязный буфер,
  400 тиков, полное побитовое сравнение).
- Суб-леджер swarm_sim::ledger: pack/floor/reslv/acts, 1-из-289 моб-тиков,
  проценты (первая версия завышала p99 до 22 мс — тень метра; починено).
  Сплит: floor 42% | pack 28.9% | acts 20.4% | reslv 8.7% — карта волны 3
  (delta-pack раундов 2+; арифметический рерайт walk без смены in→out).
- Workspace-тесты зелёные (включая 10-минутный soak parity-крейта).

## Task 31b — L112 wave3: delta-pack (scratch поддерживается, не пересобирается) (2026-11-09)

- apply_actions/request-arm зеркалят КАЖДУЮ мутацию в packed-слова
  (running, states, LOOKAT-слово через общий packed_lookat_state,
  no_action, nav_state, mf). Раунды 2+ пишут ~11 слов вместо 134.
- Debug-проба инварианта (полный pack == scratch, слово-в-слово, каждый
  раунд) поймала 2 реальных дрифта при первом запуске (melee-state запись
  в request-arm; resolver-start'ы) — починены repack_goal_words; теперь
  КАЖДЫЙ debug-тест — гейт дельта-пака.
- stats-буфер реюз: tick возвращает заём (1.2MB/тик больше не аллоцируется).
- Итог сессии: p50 8.06 → 4.73 ms/tick (−41.3%), chasing 3036 бит-в-бит
  на каждом этапе, workspace зелёный. Волна 4 (карта): per-mob резидентный
  пак (~96 статичных слов пишутся один раз при add_mob), арифметика walk
  пола (floor 42% — чистая функция, parity-гейты уже стоят).

## Task 32 — L112 wave4: the resident pack, the armed-atomic gate, the 5-flag lock table (2026-11-09)

- Директива «ещё жоще давай». Бенч 25k×400 (канон 0xC0FFEE): p50 4.94 →
  4.00–4.06 (−19%), p90 5.14 → 4.22, p99 10.67 → 9.55, chasing 3036 ==
  3036 бит-в-бит на каждом шаге. Накопительно от пивота: 8.79 → 4.00 (−54%),
  от старта wave2 8.06 → 4.00 (−50.4%). Док 44: docs/research/44-l112-wave4-resident-pack.md.
- РЕЗИДЕНТНЫЙ ПАК (PackState, один на исполнитель): провода aiPack разрезаны
  на статик-получтипа (fnv-отпечаток таблицы целей — пишется ОДИН раз;
  ~70 слов), тик-получтипа (game_time/pcount/блок игроков — штамп
  (game_time, pcount, fnv-игроков), tfp считается ОДИН раз за тик), и
  ~46 раунд-слов (cursor/mf/id/seed/res/op/pos/goal-живые) — пишутся каждый
  раунд вместо 134. 25k мобов × 88 слов/тик ≈ 2.2M лишних записей/тик убрано;
  wave3 delta-арм поглощён (раунд-слова дешевле зеркал).
- ПРОБА ПОЙМАЛА РЕАЛЬНЫЙ ДРЕЙФ на первом прогоне: park-токен
  (SEL_TARGET=0, PH_CLEANUP=0, gi=0) == 0 — возобновлённый раунд идёт через
  тот же арм, что и свежий, и слова запроса (IN_RES/IN_RES_OP/IN_RES_X)
  обязаны ехать в паке, не нулями. HurtBy (цель №0 таблицы зомби) паркуется
  ровно там в каждой погоне — release бы молча ломал цепочку погони
  (res=0 ⇒ предикат «провален»). Фикс: слова запроса — раунд-слова, пишутся
  из живых (res, req_op, xyz) КАЖДЫЙ раунд. Урок wave3 повторился: проба
  инварианта окупается на первом же заезде.
- ML_EVAL_SWAPS fetch_or был БЕЗУСЛОВНЫМ на каждом can_use — ~275k locked
  RMW/тик на одной кэш-линии при выключенном леджере. Гейт за ML_ON.load:
  один relaxed load (~1нс) вместо locked RMW (~10–20нс) на оценку.
- scan_players → scan_players_into([usize;32] на стеке): ~20M malloc за
  бенч (2 скана на моба на чётный тик) убрано; P ≤ 30 под гейтом
  MF_MANY_PLAYERS — 32 слота жёсткий потолок.
- MAX_LOCK_FLAGS = 5: таблица локов по битам флагов — ванильных ординалов
  ПЯТЬ, классифай-сет использует 0..3; ребилд писал 32 мёртвых слова за
  вызов пола → 10. Биты ≥5 у бегущей цели — collides-fallback (ваниль-модель
  принимает), биты 0..4 не изменились.
- apply_actions: Option<&mut [i32]> → &mut [i32] + ранний выход n==0;
  exec.rs: players_buf реюз (был свежий Vec каждый тик), set_full_pack()
  (гейт-близнец), tfp один раз за тик.
- ТРИ НОВЫХ ГЕЙТА: resident-vs-full близнец на уровне моба (400 тиков,
  побитово каждый тик), переживание переключения мобов на ОДНОМ scratch
  (продакшн-форма), resident-vs-full на уровне исполнителя (64 моба × 400
  тиков, весь store побитово). Debug-проба расширена на КАЖДЫЙ раунд.
  Workspace зелёный (включая 33-сек soak).
- ЛЕДЖЕР-ЧЕСТНОСТЬ: суб-леджер (1-из-289) достиг пола разрешения —
  Instant::now ×2 за сэмпл (~50–100нс) теперь сопоставим с РЕАЛЬНОЙ ценой
  фазы; сплит pack/floor/reslv/acts отсюда наполовину тень метра. Вердикт —
  только wall-clock p50; атрибуция — perf на раннере с linux-tools.
- Волна 5 (кандидаты): SoA-горячие колонки MobStore (тик стримит ~590Б AoS
  ×3-4 прохода ≈ 50–60МБ/тик — travel 2.05ms в основном это), nextInt(bound)
  дивид (50k runtime div/тик, u128 ceil-magic + thread-local кэш), perf-таблица
  с CI-раннера.
