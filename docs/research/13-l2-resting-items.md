# L2 — static fast-path for resting items: design

Follows docs 04 (lever L2) and 12 (4b.2a results). Owner mandate unchanged:
never slower than vanilla, outputs byte-identical.

## Where a resting item's tick cost actually goes

Decompiled ItemEntity.tick (Paper 1.21.8, mojang-mapped), server side, for
an item resting on the ground:

1. stack-empty → discard; base Entity.tick (tickCount++, misc)
2. pickupDelay decrement
3. xo/yo/zo = pos (oldPos stores — VALUES UNCHANGED for resting)
4. water/lava check → fluid state queries; else applyGravity (vy -= 0.04)
5. `noPhysics = noCollision(bb.deflate(0.002))` — a block-collision scan
   EVERY tick
6. moveTowardsClosestSpace (push out of blocks)
7. move(SELF, delta) — the full collision move
8. applyEffectsFromBlocks
9. friction: getBlockPosBelowThatAffectsMyMovement → getBlockState →
   getFriction — block lookups EVERY tick; delta *= friction
10. onGround landing damping (vy multiply)
11. block-position change check (xo/yo/zo floors vs current) → gameEvent
12. isMergable → mergeWithNeighbours — the merge scan (fast-pathed by the
    index/rows already)
13. age++ → despawn at 6000 (OBSERVABLE — must stay exact)
14. updateInWaterStateAndDoFluidPushing / hasImpulse bookkeeping

For a resting item, steps 4–11 are physics over an UNCHANGED world state:
gravity gets cancelled by the same collision every tick, friction and
noCollision re-derive constants from blocks that did not change, oldPos
writes store identical values. Doc 04 measured the item tick at ~1.05 µs;
the skippable pipeline tax is 35–40 % of it, the collision/move family is
most of the rest.

## The wake-up contract (exactness)

The fast path may skip 4–11 ONLY while the world cannot have changed under
the item. Any change that could move the item, hurt it, merge differently,
or affect fluids must flip it back to full ticking BEFORE the vanilla tick
would observe the difference:

- **block edits** — woven PREFIX probe on `Level.setBlock(BlockPos,State,int[,int])`
  and `destroyBlock/removeBlock` (all funnel into ServerLevel.setBlock —
  verify call graph first): wake every resting item whose box (expanded by
  1 block) touches the edited block. The index's cells are 16×16 XZ; the
  wake registry is per-cell bitsets on the Java side, keyed like the row
  table (a resting item registers its cell at fast-path entry, deregisters
  on disturbance/wake).
- **fluid state** — flowing water/lava spreads through block ticks →
  setBlock → covered by the block-edit wake.
- **explosions / pistons / block ticks** — all mutate through setBlock →
  covered.
- **merges** — the merge scan still runs every tick (step 12 stays), so
  merge behavior is byte-identical by construction. A merge that absorbs
  this item discards it (vanilla path); a count change keeps it resting.
- **pickup** — playerTouch is driven by the PLAYER's tick, not
  ItemEntity.tick — unaffected. pickupDelay keeps decrementing (stay
  inside the fast path: it is pure arithmetic).
- **despawn** — age++ and the 6000 check stay in the fast path (pure
  arithmetic, observable — never skipped).
- **chunk events** — unload/reload of the item's chunk drops the resting
  state (fast path must not persist across a chunk boundary: hook the
  EntityLookup remove probe — any removeEntity already deregisters).
- **damage / portal / /tp / plugin teleport** — all funnel through
  setPos/setBoundingBox → probeBox already fires → classify the item
  non-resting (a position change is by definition a disturbance).

## Resting predicate (all must hold at fast-path entry)

- onGround() && !isInWater() && !isInLava()
- deltaMovement ≈ 0 (|dx|,|dz| < 1e-6, |dy| <= 1e-7 — gravity's remainder)
- pickupDelay ≤ 0 (a decrementing delay does not need physics, but the
  item is still "settling"; keep it simple: full tick)
- age < 5900 (despawn imminent → full tick, vanilla flashes)
- isMergable() (merging items stay on the scan path anyway)
- the item's support block matches the one recorded when it settled
  (one getBlockState on ENTRY only)

The resting flag lives on the entity via a weave-local side table
(ARENA-adjacent id→flags array, single-homed like the row ids) — no Entity
field is added (no constructor interference).

## Weave points

1. `ItemEntity.tick` — REPLACE body (like Level.getEntities): a bridge
   `CeHooks.itemTick(ItemEntity)` that runs either the fast path or a
   faithful mirror of the vanilla body. The mirror must be byte-exact
   (decompiled-body re-implementation, verified by the e2e differential
   harness). Risk: ItemEntity.tick body is ~2× larger than getEntities —
   the mirror needs constant re-verification against the shipped jar
   (same discipline as the getEntities mirror).
   ALTERNATIVE (safer, weaker): prefix probe `probeItemTick(Entity) → boolean skip`
   — the weave returns early ONLY when skip==true (exception-table/stack
   constraints of an early-return prefix: the method returns void — a
   RETURN instruction is stack-neutral, so insert_if_return is feasible).
   The vanilla body still exists for the non-skip path — no mirror drift,
   ever. PREFER the prefix-return design.
2. `Level.setBlock...` — prefix probe (blockPos) → wake cell bitsets.
3. Existing `EntityLookup.removeEntity` probe — deregisters.

## Expected effect

- items-100k (post-merge ~63k resting items): the per-item tick drops from
  ~1.05 µs toward the merge-scan + counter floor (~0.15–0.3 µs). Vanilla
  65.8 ms/tick → ~20–30 ms. items-100k is the merge-dominated scenario, so
  the scan (already indexed) dominates what remains — the L3 hopper/tile
  lever and further merge batching are the next constants after this.
- mixed-150k: 70 % items → proportional cut of the items' share.
- The wake registry itself must stay O(edits) — block edits are rare vs
  item counts (bench: zero edits mid-measure).

## Staging

1. L2a: prefix-return weave into ItemEntity.tick + resting predicate +
   wake registry wired to setBlock probes. Gate: the row path's EMA tier
   is not enough here — itemTick is UNCONDITIONAL (every item ticks), so
   the safety valve is a measured fallback: if wake storms (wakes per tick
   > threshold) the fast path disables globally and vanilla runs.
2. CI: ab-paired items-100k must move materially; mixed/items gates stay.
