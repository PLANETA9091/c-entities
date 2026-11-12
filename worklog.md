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

## Task 32b — L112 wave4b: division-free nextInt(bound) (2026-11-09)

- goals.rs::next_int_bound: runtime `bits % bound` (nextInt(10)×3 +
  nextInt(60) на моба на чётную ветку ≈ 37k+ div/тик) → точный ceil-magic
  (Hacker's Delight 10-9): INV = ceil(2^64/bound) = u64::MAX/bound+1,
  q = (bits·INV)>>64 через u128, val = bits − q·bound. Доказательство в доке:
  e = INV·b − 2^64 < bound; bits·e < 2^62 < 2^64 ⇒ q_est == floor(bits/bound)
  ТОЧНО. bound=1/2 не доходят до магии (pow2-арм), overflow невозможен.
- Мемо: per-thread 4-slot [(bound, INV)] через Cell<(i32,u64)> (get — 16Б,
  НЕ take/set 40Б×2 — первая версия копировала весь массив на каждый draw и
  подозрительно просела); после прогрева ноль div, hit ≈ 2 get + mulhi.
- ГЕЙТЫ: sweep-тест ~83M проверок (границы 0/q·b/2^30/2^31−1, края
  ванильной зоны отбраковки 2^31−(2^31 mod bound), страт-свипы ×598
  bound, фат-свипы реальных 10/60/20/7/999999) — 0.14s в release; тест
  observation-free: замкнутый референс с plain % на 2000 seed × 5 bound.
- ЧЕСТНЫЙ A/B интерливом двух бинарников (A,B,A,B,A,B без пересборок):
  magic 4.56/4.03/4.03 vs plain % 4.57/4.00/4.02 — дельта НИЖЕ шумового
  пола этой коробки (±5%; первый «A/B» 4.19 vs 4.37 был артефактом
  дрейфа окна — урок: сравнивать только интерливом готовых бинарей).
  Оставлено: mulhi ≤ div по определению, точность пришита свипом,
  наблюдаемое поведение нулевое.
- chasing 3036 бит-в-бит на всех армах; workspace зелёный (release+debug).

## Task 33 — L113 wave6: the fold (no-park resolutions) + phantom-start fix (2026-11-09)

- ДИРЕКТИВА: «парарельность это не ускорение на самом деле. ускорение это
  реально писать код жосткий математический» — незакоммиченный WAVE5
  (scoped-threads по чанкам) ОТБРОШЕН (exec.rs откат, ledger-фикс тени
  метра остался: 25k locked RMW + 25k store/тик на несемплированных мобах
  → один relaxed load).
- СТРУКТУРА: floor-парки (park → executor резолвит → resume) стоили по
  полному раунду каждый: 46-словный repack + ре-вход (ctx rebuild + resume
  lock rebuild). Профиль staged-requests при 25k мобах ≈ 10k раунд-трипов
  на тик (find_target 1-в-10 на каждый Nearest ×2 non-player, LOS, stroll,
  RLA start, cleanup cc). Резолвер теперь объявляет FoldCtx
  (Resolver::fold_ctx) — и ai_floor_ext решает ЧИСТЫЕ v0-предикаты НА МЕСТЕ:
  find_target (alive&&!spectator по упакованному снапшоту, гейт
  find_target_fold — контракт FlatWorld иной и паркует), LOS all-pass (общий
  los_apply для фолда и resume-армы — один источник), stroll-цель (внешний
  стрим: splitmix16/LCG-16 флейвор; продвинутый сид возвращается через EXT
  хвост; действия STROLL_SET(pend) → lock_only GOAL_START(nav=pend) — тот же
  порядок resolution→start), RLA start body (DRAW-EXACT javap: nextDouble +
  nextInt(20) на СОБСТВЕННОМ LCG пола — та же позиция стрима, rel/state через
  ACT_RLA_START), cleanup cc (v0-ответ из пака + перенос target/approach).
  FoldCtx::off() (JNI-путь, фикстуры, твин) воспроизводит parked-семантику
  слово-в-слово.
- БАГ (pre-existing, фолд его вымыл): LOS-парк писал список кандидатов в
  out[8..] = СЛОТ ДЕЙСТВИЯ 0. Действие того же раунда (cc stop) + LOS-парк →
  apply_actions читал кандидатов как ACT_GOAL_START p0=0 — ФАНТОМНЫЙ старт
  HurtBy на каждом затронутом моб-тике, затем FB_LOCK_COLLIDE на rebuild.
  CI-паритет не видел: JNI-сценарии не цепляют cc-stop → LOS-park в одном
  раунде (HurtBy требует бой); нативный бенч цеплял постоянно. Фикс: нативные
  кандидаты через EXT-хвост за областью действий (FoldCtx::ext_cands),
  JNI-провод не тронут (776-словный буфер Java падает в legacy-арм). Мир:
  chasing 10292 → 12592 — фантом молча давил ~2300 корректных захватов.
- ЛОВУШКИ ИЗМЕРЕНИЯ (съели час бисекта, три stash-цикла): (1) отладочные
  env::var_os-зонды, забытые в hot path, стоили +1.1ms/тик (environ lock +
  линейный поиск × сотни тысяч вызовов); (2) per-call Lcg-мемо — регресс
  (холодные слоты перекупывали INV-деление на каждый вызов пола; откат к
  thread-local таблице, теперь CONST-init — без lazy-init ветки wave4b);
  (3) absolute-числа коробки дрейфуют ±0.3ms — честные дельты только
  same-binary A/B (CE_NO_FOLD-крюк в бенче) и candfix-база (HEAD + фикс
  коллизии, тот же chasing-мир): 100k 15.72 → 14.73 (-6.3%).
- АРИФМЕТИКА: открытая travel-цепь (friction/drag ~10 flops + 2 cast на
  моб-тик) свернута в const OG_* (const-fn float IEEE-точно); пин-тест
  пришивает биты + факт округления: 1-(1-0.6f32) возвращает РОВНО 0.6f32,
  гейт скорости `> 0.6` — арм 0.23.
- РЕЗУЛЬТАТ: 25k p50 4.00 → 3.74 (-6.5% vs wave4; -16% против pre-wave6 в
  том же окне; 100k 17.06 → 14.73, -13.7%). reslv-фаза 9% → 0% (парки
  исчезли), floor потяжелел на работу резолвера (ожидаемо). ВСЕ 97 тестов
  зелёные: твин folded-vs-parked 400 тиков бит-в-бит (fnv по всему стору),
  FlatWorld-твин, стрим-пины 5000 seed, travel-консты, все старые
  паритетные гейты. doc 45. Pack-провод НЕ менялся (фикстуры целы).
- Wave7-кандидаты: REQ_CANUSE_VANILLA-фолд (нужен target-index в контексте),
  переразметка resident pack после падения раунд-рейта (~40%), cc-фолд с
  обновлением approach в раунде.

---
Task ID: 24
Agent: main (Z.ai)
Task: WAVE15 — F-kill: параллельный executor без серийных фрагментов (док 55)

Work Log:
- Фазовая проба tick_parallel (PHASE_PROBE + #[ignore] phase_probe, 25k × 200): players 0.000 | pre 0.252 | pass1 1.763 | walk 0.082 | pass2 0.060 | tail 0.000, drawers/tick 157 (0.6% популяции) — серийные фрагменты ≈ 0.39ms = фит F=0.44.
- УМЕРЛИ: alive_idx (серийный O(n) скан), results_buf, repair_buf, assign_buf, drawer_list, серийный walk (статс-контракт + pre-assignment), пар-dispatch счётчика живых (диспатч дороже скана — измерено).
- ВЗЯЛИ: ранговый префикс (fast path free.is_empty() = ширины chunk'ов, tombstone-ветка — скан), пер-chunk статс-регионы через safe split_at_mut (каждое статс-слово пишется РОВНО ОДИН раз — мемсеты умерли), chunk-локальный drawer-леджер в WorkerBufs, O(chunks) stream-префикс (base_stream по префиксу ничьих draws — бит-точная цепочка: alive-порядок = возрастание слотов), pass 2 serial <1024 / chunk-параллель ≥1024.
- Дыра покрытия ЗАКРЫТА: pass2_serial_max крюк + трёхрукий твин parallel_pass2_chunked_matches_serial_executor (4096 мобов, ~68 drawers/тик на 4 chunk'ах): serial / par(serial-repair) / par(chunked-repair) — бит-в-бит 400 тиков.
- W14c-вердикт: gate-пролог (убить pass 2 целиком) = реплей всего floor-движка (гейт внутри goal-update конвейера) — дрейф-риск ради 40µs. Клон подтверждён 4-й раз (undo-log / журнал / prefetch / gate-prologue — все мертвы).
- ЧИСЛА (same-window A/B, 25k × 400, 2 vCPU): PAR 1.26–1.41 vs SER 1.79–2.02 → −26…−32% (4 окна; wave14 был −17%); 100k: 7.27 → 4.87 (−33%); фит T1 2.39–2.42 / T2 1.26–1.29 → F ≈ 0.14, W ≈ 2.27; par(4) ≈ 0.71ms = −91.6% (линия −90% берётся на 4 реальных ядрах), par(8) ≈ 0.42 = −95%; 2 ядра = −85% от ванилы 8.41.
- 106 Rust-тестов зелёных (модульный suite — CI-side). Коммиты c4d11ff (wave15), 700737f (15b), b098e0a (15c) — запушены.

Stage Summary:
- Параллельный executor: серийные фрагменты 0.44 → 0.14ms; −85% на 2 vCPU, −91.6% на 4 ядрах (проекция), −95% на 8.
- Док 55 (docs/research/55-wave15-f-kill.md). Следующие рычаги: travel бит-заперт (atan2/sqrt — vanilla-модель), AI floor выжат волнами 5–13; остаток W — eval+клон+travel ≈ 2.3ms.

---
Task ID: 25
Agent: main (Z.ai)
Task: N2-M0 — wholesale JNI-провод мобов: один вызов на тик, Java = COPY-слой (док 56)

Work Log:
- СВЕЖИЕ ЧИСЛА post-wave15c (2 vCPU, 25k×400): PAR 1.27/1.18/1.16 vs SER 1.76/1.75/1.75 (−28…−34%; к ваниле 8.41 = −85…−86.2%); 100k: 4.80 vs 6.91; T1 2.19/T2 1.26 → F≈0.33, W≈1.86; par(4) ≈ 0.53–0.80 = −90.5…−93.7%. ДЕКОМПОЗИЦИЯ W сходится байт-в-байт: клон+протокол 0.25 + AI-флор 0.56 + travel 0.45 = 1.26. SoA-гипотеза УМЕРЛА: L3=504МБ — рабочий сет 2×6.2МБ резидентен, pass1 compute-bound. Бенч-форма исчерпана → рычаг = продакшн-фаза.
- РАЗРЫВ: centities-native НЕ имел ни одного JNI-экспорта; единственный прод-провод мобов — пермобный L22a-флор (инт-массивные раунды на моба — форма, которую бенч похоронил). Док 42 §8: без N2 прод-клейм запрещён. → N2-M0.
- ПРОТОКОЛ (batch.rs, centities-native, JNI-free): MobBatch над proven NativeTick (параллельный arm включён) — init/add(eid,seed,x,y,z)/remove/set_players/tick(gt,ids,pos)→Ok(n)|Err(needed); read-back: ids[2i]=eid, ids[2i+1]=nav<<32|no_action, pos[4i]=x,y,z,yaw; truncation = Err(pop) без записи. ГЕЙТ: twin batch_matches_direct_executor — 64 зомби × 400 тиков, статс-стрим PartialEq КАЖДЫЙ тик, read-back == полям стора КАЖДЫЙ тик, FNV (22 слова + все goal-рекорды) равен в конце. +3 теста (add/remove/re-add, truncation, players-echo). NativeTick получил last_stats().
- JNI-КЛЕЙ (mobcore.rs, centities-module): 6 нативов на HOOKS_CLASS (Init(J)V/Reset()V/Add(IJDDDI)I/Remove(J)I/Players([D[I)V/Batch(I[J[D)I); thread-local executor + scratch, ноль аллокаций в steady-state; тик происходит ВСЕГДА, read-back может отстать на тик при скачке популяции (возврат −pop).
- JAVA (CeHooks, гейт CE_NATIVE_MOB=1, DEFAULT OFF — шиппед-конфиг не тронут): arm через ТОТ ЖЕ serverAiStep EarlyReturnGuard + ЛЕГКИЙ initNativeMobHandles (zombie/yRot/deltaMovement/noActionTime/SHARED_RANDOM с ceSrOriginal-unwrap); sight = первый маршрут guard'а (ДО L22a-гейтов): zombie+aware → add при первом взгляде + skip тела; boundary = itemTickFlush (граница тика уровня, до n==0-выхода): push игроков → ОДИН nativeMobBatch → аплаер (noActionTime/yRot Unsafe same-value skip, Motion→0 (общий immutable Vec3), fallDistance→0, setPos при изменении) + removal по guard-saw.
- ЧЕТЫРЕ ЖИВЫХ БАГА, УБИТЫХ НА СМОКЕ (живой Paper 26.3 + агент, консоль-summon): (1) thread-local раскол arm/тик — executor создавался на init-потоке, серверный видел None, add молча −1 → lazy-init из atomic seed; (2) кинематическое перетягивание: ванильная гравитация на старой дельте × снап-позиции аплаера = накопленный fall_distance, 5 dmg/s, смерть за 3с (NBT: OnGround=0, Motion=−0.304, fd=0.464) → аплаер владеет кинематикой (Motion→0/fd→0/pos); (3) кросс-левел-убийца: itemTickFlush плетётся на КАЖДЫЙ уровень (3 вызова/тик), глобальный saw-set давал churn 1276 add/1275 remove за 2551 тик → saw-set per-level (m.level()==level); (4) removal-driver: read-back diff НИКОГДА не видит Java-смерть (стор водил мёртвый объект) → драйвер = guard-saw-set.
- ЖИВОЙ ПРУФ: «armed the WHOLESALE native mob executor (N2-M0)» + n=1 ads=1 rem=0 за ~1400 тиков + jpos == нативной позиции БИТ-В-БИТ на каждом полле + Health 20.0 стабильно + stroll-походка (walk→arrive→pause→walk). sum: summon → guard-skip → JNI-add → нативный тик (параллельный executor!) → read-back → аплаер — ЗАМКНУТО.
- НАЙДЕНО (пре-существующее, НЕ N2): @e[type=zombie] пуст с модулем при живом тикающем зомби (ваниль находит; untyped @e работает) — подозреваемые: Level.getEntities/getEntitiesOfClass rewrites item-индекса; один SIGILL в CRUSSTY class_file_load_hook на самом первом boot'е (worldgen-late thread), не воспроизвёлся в трёх следующих — флейт песочницы.
- FUSED FILL: первый A/B провода @25k (CE_WIRE-кноб: тот же мир через MobBatch) дал +0.39ms — fill перечитывал весь стор (6.8МБ) вторым проходом. Фикс: NativeTick::tick_wire_fused — read-back пишется ВНУТРИ pass1/pass2 (тот же alive-rank k, что у статс-регионов; слова drawer'ов перезаписывает repair; регионы едут в zip как OWNED per-chunk Option — shared &mut не Sync для par-замыканий). Стало +0.06–0.07ms (5.7× дешевле, ~5% стены); serial-формы падают в fallback-fill батча. Твин 24/24 не задет.
- ТЕСТЫ: 116 зелёных (native 24 + module 14 + swarm-sim 22 + core 49 + swarm-core 7, +1 ignored phase_probe); natives_parity-страж дополнен 6 дескрипторами. Коммит на main.

Stage Summary:
- N2-M0 LAND: протокол (JNI-free, twin-гейт) + JNI-клей + Java copy-layer (гейт CE_NATIVE_MOB, default OFF). Один JNI-переход на уровень-тик вместо пермобных раундов. Живой сервер: замкнутая петля без churn, бит-точный синк, выживание.
- Следующее: M1 (aiStep-weave — полный скип Java-тела), M2 (e2e-стенд под гейтами: перф-ноги с ботом + JFR), M3 (N3 event-outbox: урон/звуки/дропы), N4 (транскрипты на границе executor'а).

---
Task ID: 8
Agent: main (Z.ai)
Task: N2-M1/M1.5/M1.6/M1.7 — the wholesale wire closes the Java body; the first production A/B

Work Log:
- M1 (2a5e60b, doc 57): the aiStep weave — a chained EarlyReturnGuard at the head of Mob.aiStep; the whole per-mob Java body (LivingEntity.aiStep: travel/gravity/sensors/serverAiStep + Mob.aiStep: burnUndead/looting) = one static call. javap-verified checkDespawn is OUTSIDE aiStep (despawn survives). Live: the route marker, skip lockstep, jpos bit-exact.
- The 2000-mob live smoke caught the activation-flip churn (the saw-set proxy: an inactive mob never feeds the set, the removal diff evicted the whole population — ads+2000/rem+2000 once). Fix (6c89799): the removal driver reads the Java object state (isRemoved/isAlive) at the boundary — activation-independent, same-tick death detection. Post-fix: ads=2008 rem=0 for the whole soak.
- M1.6 (2a1727a): the wire fastpath — NM_BYID -> the primitive NmMap (linear probing, Knuth 6.4 R deletion, zero boxing); the saw machinery dies (dead code); the liveness check folds into the apply loop (int scratch + drain, zero map walks in the steady state). JFR: nativeMobStep self 6.4%->0.0%, boundary self 6.7%->1.9%.
- M1.5 (2a1727a): the tick weave — a THIRD guard at Mob.tick: the native mob's WHOLE Java tick dies (baseTick fire/air/lava/tickCount, pushEntities, updateControlFlags, swing); the applier advances tickCount per mob per boundary; the first-sight classification moved to the tick route. The sight refuses !isAlive() (a dying mob would otherwise be re-classified — a corpse the vanilla dead-branch can never remove). Death path re-verified live (/kill -> rem=1 same boundary, survivors untouched).
- M1.5b (c593617): the zombie-family weave — Zombie OVERRIDES tick() (the drowning-conversion tracker ran 6.0% of the server thread for natives!); the guard now rides Zombie.tick (+ ZombieVillager.tick; Drowned/Husk inherit). Per-class woven state in PRECOMPUTED, deferred verifier weaves.
- M1.7 (0ea9d1e): the liveness fast read — !isAlive() walked getHealth() -> the FULL SynchedEntityData chain (health lives in the data watcher in 26.3), 15.7% of the server thread at 10k. Now U.getBoolean(dead) || isRemoved() — two plain loads; a missed handle keeps the wire off.
- The first PRODUCTION A/B (7caf5ef): scripts/run-baseline.sh end-to-end locally (2 vCPU, mineflayer bot via ViaVersion, 10000 fill zombies, 300-tick measure): vanilla 174.7 ms/tick p50 -> the native wire 49.6 = -71.6% p50, TPS 5.7 -> 20.0; wire state n=10000 ads=10000 rem=0 across the run. CI: the mobs-25k-native matrix row (matrix.EXTRA splices CE_NATIVE_MOB=1 into the paired mod leg) — the dedicated 4-vCPU runners + the back-to-back A/B are the -90% arbiter.
- The @e[type=zombie] selector flag (doc 56): NOT reproducible with the plugin present (typed + untyped selectors both return the entities) — the earlier repro lacked the plugin's chunk tickets; documented, the CI arbiter note stands.

Stage Summary:
- Six commits on main: 2a5e60b, 6c89799, 2a1727a, 7caf5ef, c593617, 0ea9d1e — all pushed. Module suite 5/5 green throughout; CI parity/e2e paths byte-identical (every new weave is gated on CE_NATIVE_MOB).
- The executor line stands: par(4) 0.53-0.80 ms (-90.5...-93.7%) in-bench; production 10k with a player: -71.6% p50 local (noisy box, directional).
- Next: the mobs-25k-native CI leg (the -90% arbiter), M3 (the N3 event outbox: damage/sounds/drops — returns environmental death), the tracker's remaining dirty scan + the applier setPos path if the 25k numbers demand more.

---
Task ID: 9
Agent: main (Z.ai)
Task: N2-M1.8b/M2 — the activation gate, the dispatcher floor, the three live-boot wire fixes; the e2e CI repair

Work Log:
- The e2e CI had been red since aad3509: the N2-M0 commit widened the L22a declined marker with the CE_NATIVE_MOB leg and the closing-paren grep never matched again. Fixed the pattern (prefix-anchored), verified ALL 25 static e2e markers against the current sources with a checker script (scripts/check_e2e_markers.py). e2e green again on 62ad1fe.
- The local 10k leg after M1.7 booted ARMED with the wire DEAD (mob=0/0/0): THREE silent boot bugs killed on forensics (doc 58): (1) M1.7 looked up Entity.dead — the field is LivingEntity.dead in 26.3, the probe declined and the whole wire stayed vanilla; (2) the M1.5b zombie-family weave chained the burnUndead probe onto Zombie bytes (the method is Mob-private) and PATCHES_FAIL++ed the arm gate to death on every CE_NATIVE_MOB=1 boot; (3) the boot-dir mapped jar without a sibling libraries/ starved javac (CE_MAPPED_JAR override).
- M1.8b was shipped DEAD: the op-4 receiver heuristic emitted ALOAD 1 into the STATIC checkIfActive frame — VerifyError, silent retransform rejection, no counter. CRUSSTY cplug-sdk fixed (statics pass local 0); after the rebuild the activation gate arms and the getHealth data-watcher walk (~2.6% @10k) dies.
- M2a: the L7 itemDispatch guard now suppresses the WHOLE dispatcher floor for in-store natives (profiler lambda alloc, commonTick, checkIfActive, the dead tick() virtual, inactiveTick). Passengers route vanilla; checkDespawn stays in the walk lambda (despawn exact). Live: dmob=5.86M @10k.
- M2b: the applier read-back compare reads Entity.position -> Vec3.x/y/z directly (one ref + three plain doubles vs the getX/getY/getZ virtual trio, ~1.9% @10k), handle-gated.
- Infra: the sandbox pty gave jline a live terminal — the console redisplay loop ate ~6% of the JFR; the bench server's stdin now rides /dev/null (both legs).
- The paired local A/B @10k+bot: vanilla 199.9 p50 -> native 51.9 = -74.0% p50 (the box is ±40% noisy; the CI 4-vCPU mobs-25k-native legs are the -90% arbiter). All 116 tests green on the module line; the wire steady (n=10000 ads=10000 rem=0, jpos bit-exact).

Stage Summary:
- Three commits: 62ad1fe (the e2e marker repair), c8e9162 (the M1.8b/M2 wave), 38fa93e (the bench stdin). CRUSSTY: ce-m2-static-fix (the op-4 static fix + the wave's working state).
- The wire's live-boot path is now forensics-hardened: every silent catch logs, the probe failures name the throwable, the counters (dmob) prove the weaves engage.
- Next: the CI arbiter verdicts (mobs-25k-native), M3 (the N3 event outbox — environmental death), the GC leg only if the CI numbers demand it.

---
Task ID: 10
Agent: main (Z.ai)
Task: N2-M2c — the checkDespawn residency skip (the paper-config walk dies)

Work Log:
- The 10k JFR priced the checkDespawn chain at ~3% of the server thread:
  despawnRanges Map.get per mob per tick (the HashMap.get leaf 1.9%),
  the paperConfig() field walk, findNearbyPlayer, getDifficulty/
  isPersistenceRequired. For an in-store native every line is dead weight.
- The 5th op-4 guard on the Mob blob: checkDespawn()V <- nativeMobTracked
  (Entity)Z — the PURE residency probe (no classification; the sight stays
  pinned to the tick route; receiver Mob assignable to Entity — the same
  link-safety shape as the Zombie.tick weave).
- DELTA (honesty ledger): a resident native no longer hard-despawns — the
  paper despawn ranges return with the N3 event outbox. The wire's removal
  driver covers death/discard exactly; the bench population is
  persistenceRequired anyway.
- Live 10k leg: checkDespawn/findNearbyPlayer/despawn leaves GONE (1 sample
  of 5399, was ~3%); wire n=10000 ads=10000 rem=0, jpos bit-exact, dmob
  live. Commit 01e4b54, pushed.
- Also this session: the M1.8a zombie-family arm-time retransform fix (the
  guard never reached the JVM when the zombie classes loaded pre-arm —
  the deferred verifier path is dead for them because
  precompute_zombie_family REMOVES the stash; the m10k-native8 profile
  showed ConversionTracker.tick still riding Zombie.tick — the guard is
  now visible landing in the arm retransform loop, "patched zombie-family
  tick guard" x2). The op-4 STATIC receiver fix (local 0 for static
  methods) went in via the CRUSSTY sdk commit d7ae117.

Next: the tracker gate residual (ttlCachedPath ~1.3%), the applier's setPos
for the walking population, the mobs-25k-native CI leg (the -90% arbiter).

---
Task ID: 11
Agent: main (Z.ai)
Task: N2-M2c verification + the paired 10k production A/B at the M2c build

Work Log:
- Paired 10k legs (same window fill300/warm60/measure300, same box hour):
  vanilla 283.4 p50 / 313.5 mean vs native M2c 47.9 p50 / 60.9 mean =
  -83.1% p50 (the first production A/B was -71.6%). Wire n=10000
  ads=10000 rem=0, dmob=4.97M, jpos bit-exact; module suite 14 green.
- Post-M2c profile: the mob-attributable server-thread leaves sum to ~7%
  of all samples (the walk 1.52, the applier's primitive reads ~3, the
  tracker gate 0.92, setPosRaw 0.49, the tracker dirty scan 0.48) — the
  Java-side mob floor is CLOSED; the rest is harness + shared server
  floor. doc 59 (59-n2m2c-checkdespawn-skip.md).
- Dead-end ledger: the M1.8b activation weave shipped dead (the op-4
  static ALOAD bug — fixed in the sdk d7ae117 and made redundant by the
  M2a dispatcher floor); the M1.8a zombie-family arm-time retransform fix
  (the deferred path is dead for pre-arm classes — the stash is removed by
  the precompute); the local 25k vanilla leg exceeds the sandbox call
  ceiling (the CI 25k leg stays the arbiter).
- Next: M3 (the N3 event outbox), the 25k CI pair, the tracker gate
  residual.

---
Task ID: 12
Agent: main (Z.ai)
Task: M2d — the bench console dies (--noconsole); the paired 10k pair reads -87.4% p50

Work Log:
- The rootfs hit 100% (each bench server dir pins ~250MB) — that was the
  source of the tool-call timeouts late in the session. Purged bench/*/server
  (results kept, the runner recreates them): 5.0GB -> 559MB, the disk 54%.
- The M2d launcher fix: `--noconsole` for the bench server (the jline
  PumpThread spun on the /dev/null EOF — ~1.5% of samples). javap-verified
  the flag in org.bukkit.craftbukkit.Main (26.3). Both legs share it.
- The re-run pair (same window, same hour): vanilla 169.7 p50 / 188.4 mean
  vs native 21.4 p50 / 33.4 mean = **-87.4% p50** at 10k player-present
  (the wire n=10000 ads=10000 rem=0, dmob live, jpos bit-exact).
- doc 59 addendum. The -90% arbiter stays the mobs-25k-native CI leg; the
  executor line (par(4) -90.5...-93.7%, doc 55) and the production 10k
  pair now both sit at the line's doorstep on a 2-shared-vCPU box.

---
Task ID: 13
Agent: main (Z.ai)
Task: the vanilla-order regression gate + M2e (the walk fast route) + M2f (the applier trim)

Work Log:
- Doc 36b gate: the versioned parity-order re-sort (upsert_ord /
  ensure_cell_order) shipped in the N2 wave train WITHOUT its own
  regression tests. tests/order_parity.rs adds 6 tests against an
  independent vanilla-order model — cell-internal insertion independence
  (+ the sy-desc/seq-asc adversary), chunkZ-major walk across six chunks
  (seqs assigned chunkZ-ASCENDING on purpose), re-append/section-move
  reorder + same-stamp no-churn, oversize global merge, 12k-op removal
  storm (ghost ids fail loudly), world separation. 6/6 green first run;
  42 core tests green. Commit db1a432.
- N2-M2e (doc 60, commit 4294755): the L13/L17 walk mirror fast-routed
  native ITEMS straight into the flush; a classified MOB still paid
  consumer.accept -> the vanilla lambda (isRemoved read, the M2c-killed
  checkDespawn entry, tickNonPassenger) -> the M2a guard chain — every
  frame inside already-dead code, invoked per mob per tick. The M2e
  branch routes the mob with the SAME decision chain (nativeMobTick)
  one level up; a decline takes the exact vanilla path; mobRoute flips
  whole-hog on disarm (no tick-list membership touched, zero
  bookkeeping); -Dcentities.mobWalkRaw=true refuses. Live m10k-m2e:
  walk=6.29M == skip=6.29M, dmob=0 (the M2a dispatcher counter FROZEN —
  the dispatcher is never entered), n=10000 rem=0, jpos bit-exact.
- Paired local A/B (same box hour, fill300/warm60/measure300, 2 shared
  vCPU): vanilla 151.1 p50 / 161.2 mean vs native M2e 19.4 / 30.5 =
  -87.1% p50 (the M2d build read 21.4 on the same protocol — the route
  is the ~2ms/tick of dead frames).
- N2-M2f (commit 9942a43): the boundary applier's per-mob reads were the
  top server leaf (getDouble 3.4% + getBoolean 1.5%, all inside
  nativeMobBoundary). The Motion zero check now reads the FIELD
  REFERENCE (1 read vs getObject + 3 Vec3 loads — any plugin
  setDeltaMovement allocates a fresh Vec3, identity-differs, re-store
  fires; Vec3 immutable); the M2b position read-back's loads are LAZY
  inside the unchanged predicate (NaN guard short-circuits, a mover
  whose x differs never pays y/z). m10k-m2f: p50 19.6 / mean 27.9 (the
  mean carries the trim), walk=6.38M, dmob=0, jpos bit-exact, GC pauses
  0 for the whole leg. The remaining getDouble leaf is the safepoint-
  biased attribution of the whole applier loop (setPos + the volatile
  fence inside the same body) — load-bearing.
- walkMobNative counter demoted to a PLAIN long (the increment and the
  poll read ride the same tick thread — a volatile store per mob per
  tick was a fence in the hot route for nothing).
- Sandbox provisioning rebuilt from scratch (the recycle wiped /tmp):
  rustup, JDK 25.0.5 (javac v69), paperclip 218 extraction (the pinned
  build, sha256 verified), maven 3.9.9, ViaVersion 5.12.2 + ViaBackwards,
  mineflayer. The local leg protocol: XMS=512M (XMS 2G > XMX 1536M died
  the JVM silently once), RUN_TIMEOUT=300, ~9 min/leg inside the 600s
  tool ceiling.
- CI: the mobs-25k-native arbiter dispatched on 9942a43 (M2e+M2f);
  the full-matrix baselines queued behind it.

Next: the arbiter verdict, M3 (the N3 event outbox — the despawn design
needs a player-present non-persistent scenario to be honest: the parity
harness has no player, checkDespawn declines early, so the harness CANNOT
see despawn), the tracker gate residual (ttlCachedPath ~1.7% — mature
L59-L72 territory, byte-exactness-sensitive, LOW priority).

---
Task ID: 14
Agent: main (Z.ai)
Task: the despawn pure port (N3-N1 phase 1) + the CI matrix rides the wire

Work Log:
- doc 61 (d1103b3): the exact 26.3 checkDespawn semantics javap'd from
  the pinned jar — the ellipsoid/cylinder shapes, the persistent gates,
  the noActionTime>600 + 1/800 soft roll, the in-soft-range RESET (the
  ELSE branch, NOT persistence-gated), the config table push design,
  the virtual class-cache, the SHARED_RANDOM draw slot (the draw COUNT
  per mob is computable pre-draw -> the wave13/14 prefix assignment
  survives), and the honest blocker: NO existing scenario can see a
  native despawn (parity has no player; the bench is persistent) —
  a new player-present non-persistent scenario is the gate.
- The pure port (c04cd63): crates/centities-native/src/despawn.rs —
  DespawnRange::should_despawn byte-exact both shapes, DespawnCfg::
  decide() the full tail, the draw stays with the caller. 6 geometry
  tests (every == vs > edge, the ratio stretch, the else-reset for
  persistent mobs too). 29/29 native tests green. Phase 2 = the config
  push + the executor slot + the outbox (doc 61 §"The native pieces").
- CI matrix: the spread-100k / mixed-150k rows carried a zombie
  fraction (20% / 10%) that ticked VANILLA in every previous mod leg —
  the committed L-era report read spread -2% / mixed +46% (the weave
  overhead on vanilla mobs drowned the item wins). Both rows now pass
  CE_NATIVE_MOB=1 (19b8d7f) — the production configuration; the
  populations are persistenceRequired so the M2c ledger is invisible
  by construction.
- CI verdicts so far: e2e SUCCESS on e61307f (M2e+M2f — the walk route
  + the applier trim under the natives soak), module SUCCESS; parity
  in flight; the mobs-25k-native arbiter queued/in-flight on two
  baselines (the dispatch + the full matrix).

---
Task ID: 15
Agent: main (Z.ai)
Task: N3-N1 phase-2a live (the despawn cfg push) + the verification leg

Work Log:
- phase-2a (84550f2): pushDespawnCfgOnce rides the FIRST boundary per
  level (WeakHashMap-guarded, one attempt, handle-gated) — reads the
  PUBLIC spawning.despawnRanges map + despawnRangeShape, the four
  PRIVATE DespawnRange precomputations via Unsafe offsets, resolves v
  through the IntOr.Default record accessor; one nativeDespawnCfg
  (II[D[I)Z crossing lands the table in the mobcore thread-local. A
  NaN/stride/refuse leaves DESPAWN_CFG_LIVE=false and the M2c skip
  stays (the SAFE default). natives_parity pins the descriptor.
- Live m10k-n3cfg leg: "N3-N1: despawn cfg live (shape=ellipsoid
  present=255)" — the REAL Paper 26.3 config crossed: ALL 8 categories
  present, the default shape is ELLIPSOID (ground truth for phase-2b's
  tests; the cylinder tests stay as the alternative shape's gate). The
  push ran once per level (3 worlds x the boundary driver) — phase-2b
  needs the per-level cfg state (noted in doc 61 §"the native pieces").
  MSPT p50 19.05 (the best on this box), walk=6.3M, dmob=0, jpos
  bit-exact — the push is free on the hot path (one WeakHashMap probe
  per boundary).
- CI: e2e SUCCESS on e61307f (M2e+M2f) and c04cd63 (the despawn port);
  module SUCCESS both; parity + the mobs-25k-native arbiter in flight.

Next: phase-2b — the executor slot (the decision rides the pack pass),
the RNG walk-order draw prefix, the discard outbox (nativeMobDiscard ->
mob.discard(DESPAWN) at the boundary), the per-level cfg state, and the
player-present non-persistent bench scenario (doc 61's honest gate).

---
Task ID: 16
Agent: main (Z.ai)
Task: N3-N1 phase-2b LIVE (the despawn decision) + phase-2c (the honest gate) — a864230, 32e0c96

Work Log:
- Doc 61b written BEFORE the code: the ground truth re-pinned from the
  paper-26.3-249 mapped jar (Mob.checkDespawn bc 0-309; the hard branch
  FALLS THROUGH to the soft check; the roll runs IFF soft-hit &&
  !persistent && noAction>600 BEFORE removeWhenFarAway; EntitySelector
  lambda$static$7 = !isSpectator && isAlive && affectsSpawning — the
  affectsSpawning field is PUBLIC; getNearestPlayer = strict-< 3D^2,
  first-in-list wins ties, radius -1 = unlimited; MobCategory ordinals
  MONSTER=0..MISC=7; removeWhenFarAway base = TRUE; the TriState is
  public).
- The executor slot: the decision rides re_eval_mob BEFORE the AI (the
  vanilla lambda$tick$0 head) in BOTH arms (serial walk + wave15 pass 1)
  — per-mob pure => bit-identical by construction. The roll draws from
  the mob's OWN mirror inside decide (the doc 61 §4 walk-order note
  re-framed: under the shipped per-mob-mirror semantics the order is
  trivially per-mob; bound 800 is even => exactly one LCG step — the
  wave15 parallel shape untouched, NO new drawer class).
- The outbox became the CAUSE COLUMN on the read-back rows (ids[2i+1]
  bits 48..56; nav_state moved to 32..48 — the M0 wire never read it):
  zero extra JNI crossings; the applier discards(DESPAWN) + feeds the
  removal driver; finalize_despawns tombstones AFTER the fill (fused:
  inside tick_wire_fused; serial: after the batch's own fill — which
  ALSO fixed a LATENT DOUBLE TICK: the old serial fallback re-ran
  exec.tick after tick_wire_fused had already walked).
- The kind bits ride nativeMobAdd (category + persistence||passenger||
  leash + removeWhenFarAway(0.0) once + the public TriState resolution);
  the Difficulty ordinal rides the batch call PER TICK (nativeMobBatch
  (II[J[D)I) — /difficulty is runtime-exact; the affects-spawning bit
  (bit 1 of NM_PF) computed per player per boundary.
- Tests: the despawn strata twin (hard/soft-roll/missed-roll/persistent/
  no-affects/in-soft-reset — the rows asserted on the read-back + the
  whole-store dumps equal after the tombstones), the no-player gate, the
  draw-order proof (the roll lands before the AI draws — pinned vs the
  decision-OFF arm), the peaceful-branch unit set; the batch twins
  serialize on the EXEC_TWIN_LOCK discipline. THE WHOLE WORKSPACE GREEN
  (incl. the 25k bench: p50 1.48ms, census 3745==3745 — the decision OFF
  in the bench costs zero).
- Caught while wiring: the MobAI despawn word must ride the twins' dumps;
  add() keeps MobAI::new defaults (kind==0 is NOT "the defaults" — a real
  zombie's kind carries rwf|peaceful).

Stage Summary:
- Push: a864230 (phase-2b) + 32e0c96 (phase-2c) on main.
- The bench honest gate: ab-paired mobs-despawn-10k (DESPAWN=1:
  removeWhenFarAway(true), non-persistent, AREA=700, 600-tick measure,
  CE_NATIVE_MOB=1 on the ON leg) — the identical spawn layout (rnd seed
  42) collapses to the same strata in both arms; the gate asserts the
  end-population delta <= 5% on top of the MSPT ratio.
- Next: the per-level executor split (the M0 one-executor ledger: 3
  levels boot on the bench (no allow-nether=false!) — itemTickFlush is
  woven per Level => the boundary fires 3x per server tick and the ONE
  executor ticks the whole store 3x; the saw-set hides it from the walk
  counter, the populations self-apply, but the mob AI advances 3x per
  server tick on ANY real server) — the worklog's per-level cfg state
  item, now proven structural.

---
Task ID: 17
Agent: main (Z.ai)
Task: N3-N1 phase-2d live (the per-level executor split) + the dsp poll ride

Work Log:
- Phase-2d (44d84ea): THE PER-LEVEL EXECUTOR SPLIT — itemTickFlush is
  woven PER LEVEL, so the ONE executor ticked the whole mob store 3x per
  server tick (3 worlds boot: overworld + nether + the_end; no
  allow-nether=false in the bench). The split gives each level its own
  executor slot + per-level cfg state; the saw-set hid it from the walk
  counter, the populations self-applied, but the mob AI advanced 3x per
  server tick on any real server. Closed by construction: the per-level
  wire drops the redundant crossings AND the 3x tick.
- The dsp counter rides the N2 poll line (dc3e150): the phase-2b LIVE
  smoke on a bot-present boot — 9 native hard-despawns fired (dsp=9),
  the survivors stayed, jpos bit-exact, the per-level cfg printed per
  level (3 worlds). The despawn decision is LIVE on the real server.
- Sandbox re-verified on dc3e150 (2 vCPU): the whole workspace green
  (17 suites), the 25k bench p50 1.26 ms/tick (−85.0% vs the 8.41
  bench origin), shapes even_z0 54.5% / even_z1 45.5% / gen 0.
- CI forensics: no baseline-results commit after Nov 8 — the publish
  fix (HEAD:main) + the watchdog raise (RUN_TIMEOUT 3600, job timeout
  330) were already ON main; the Nov-8 SUMMARY's mobs-25k-mod row (656
  ms) is the plain no-knob leg (the mob wire needs CE_NATIVE_MOB=1 —
  the mobs-25k-native row exists since 7caf5ef); the blackout is the
  pre-L104 mod sides dying to the 45-min job timeout + the 36.7-min
  per-side watchdog (run 341's signature), both fixed on main. The
  next push re-arms the full-matrix arbiter.
- Local paired items-25k A/B (26.3, this sandbox, same window):
  vanilla p50 20.26 / mod 10.87 (−46.4%, mean 29.6 → 16.6). The item
  wire is the weakest leg (CI −56% at 100k). The measure-window JFR:
  the module cluster ~17% (tracker 2.8 + tickListForEach 2.7 + the
  flush protocol 3.6 + rowQuery 4.0 + dispatch glue 1.2), the native
  item tick itself invisible (the mirror carries the state; L8.2's
  compact batch already side-packs only the cd entries). The L37f rest
  arm is OFF by design (the 26.3 %4 exception) — rest=0 in the leg.
  The next item wave rides the proven shapes: native merge-cadence
  candidates in the compact read-back (rowQuery dies for the cadence
  scans) + the native-side pre-decided dispatch skip.

Stage Summary:
- Push: 44d84ea + dc3e150 already on main; this entry re-arms CI.
- The mob story: −87.4% paired @10k (CI), 1.26 ms/tick @25k in-bench
  (2 vCPU), the despawn decision live, the per-level split structural.
- The item story: −46% paired @25k local; the wire is stateful (L8.2),
  the read-back compact (L79), the floor invisible; the fat is the
  Java-side cadence scans + tracker pass.
- Next: the item wave (the doc first), then the CI arbiter reads the
  mob rows again (mobs-25k-native, spread, mixed ride the wire).
