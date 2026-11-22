# 67 — B22: the kernel-owned fast rows (the verdict wire pays for itself)

Date: 2026-11-22 (the -90%-everywhere push continues, session 29).
Inputs: the B21 verdict (addendum 6 — the live consult regressed +12ms
spread ON from the prep pass cost; the wire ships dormant behind
CE_WVERD=1), the fresh profile walls on 7d78d271 (the post-L190c/B21fix
code): spread tickListForEach ~5.0%, itemTickFlush ~5.1%, Unsafe ~6.3%,
ttlCachedPath ~3.8%, appendDispatch ~1.4%; items: the moonrise churn ~13%
(both-leg, ratio-dead), SynchedEntityData.get ~3.0%, the walk ~3.5%,
itemTickFlush ~4.3%.

## The honest arithmetic (why B22 and not the walk diet)

The verdict walk alone is a wash: the killed reads (~0.5ms/tick spread)
compete with the dieted prep (~1ms/tick at the 100k scale). The wire
becomes PROFITABLE only when the verdict cohort stops PAYING the row
plumbing entirely:

- the walk's cohort arm (IB_IDS check + IB_FLAGS/IB_I/IB_CD/IB_ITEM/
  IB_ATI writes — the appendDispatch block for the cohort misses),
- the flush's loop A per-row reads (IB_IDS/IB_ITEM/IB_FLAGS/RM_BIT),
- the row content's Java-side state reads.

The fresh walls price this at ~2.5-3.5ms spread ON + ~2ms items ON —
the single largest ON-leg-only family left after the L190c diets.

## The surgery (what moves, what stays)

1. THE ROW PLAN AT THE PREP. The kernel remembers the last prep's
   verdict array + the roster (ItemWorld state — no extra crossing).
   The walk consult stays as shipped (CE_WVERD=1).
2. THE WALK'S FAST ARM (VERD=1/2): BATCH_POS/BATCH_GEN stamps STAY
   (the L119 positional-drain contract is load-bearing: the fast rows'
   mid-walk box changes must hold until vanilla's loop reaches them —
   pos = the batch index), ibN++ consumes the batch index, and the row
   identity rides TWO sequential writes: FAST_ROW_IDS[k] = eid,
   FAST_ROW_REFS[k] = e. The IB_* packing (the appendDispatch block)
   is SKIPPED for the verdict cohort.
3. THE FLUSH INPUT (nativeItemTick): the kernel builds the MERGED batch
   in walk order — for each roster slot in order: VERD!=0 -> the row
   from its own store (the L7 dispatch-owned state IS the truth: x/y/z/
   dx/dy/dz/age/pickup_delay/tick_count/despawn_rate/fall_dist/
   on_ground + the verdict's inactive flag); VERD==0 -> the next
   IB_IDS row (the chain rows keep the Java packing). The id check
   (the chain row's id == the roster id) validates the interleave —
   a mismatch falls back wholesale to the exact IB_-only batch (the
   verdict cohort re-arms next tick).
4. THE READ-BACK GAINS THE ROW KIND: one bit per row (B21's verdict
   kind) lets the applier route: the fast row -> the entity ref from
   FAST_ROW_REFS[fastK++] (ARENA[id & AMASK] validated by the L174
   belt discipline); the chain row -> IB_ITEM[i] exactly as today.
5. LOOP A DIETS: the fast rows read the id/ref from the fast-row
   arrays (sequential) instead of IB_IDS/IB_ITEM (packed); the flags
   ride the read-back bits; the stamp/quiet decisions read the same
   od/oi/bits data. The resync rows are chain rows (IB_ITEM present)
   — the RS_IDX/ATI passes are untouched.
6. THE PD-DIRTY CHANNEL: the pickupDelay is STORE-owned for the fast
   rows, so a probed pd write must reach the kernel BEFORE the row is
   built: the PD_DIRTY belt drains (id, pd) pairs at the prep (the
   existing belt grows a second lane; the pd-dirty owner set is
   unchanged).

## The gates (what must stay bit-exact)

- The batch membership set is UNCHANGED (every item that ticks today
  rides the batch); only the DATA SOURCE moves (the store vs the
  Java-packed IB_*) for the verdict cohort.
- The L119 drain contract: the BATCH_POS/BATCH_GEN stamps stay for the
  verdict cohort (the pos = the batch index = the kernel's row index).
- The row ORDER = the walk order = the TL order (the same discipline
  the TLO dual-hash proves every dump tick).
- The despawn/merge/land events return through the compact rows as
  today; the absorption body's order relative to the tail writes is
  the twin-asserted order.
- The interleave-mismatch fallback: any id drift between the kernel's
  row plan and the Java-packed chain rows disables the verdict cohort
  for the tick (the exact IB_-only batch) — the chain never degrades.
- The twins: the existing item suites + a new B22 twin (the verdict
  cohort's rows vs the full-packing rows over a 25k-shaped storm —
  the dump-equal discipline).

## The expected effect

The verdict cohort's walk body drops to the BATCH stamps + 2 sequential
writes (~8ns/visit); the flush's loop A reads sequential fast-row
arrays; the appendDispatch block leaves the thread for the cohort. The
wire's prep cost (~1ms dieted) pays for itself at ~2x margin. The
named walls it eats: the walk's remaining dispatch body (~1ms), the
flush loop A's fast-row share (~1.5ms), the appendDispatch frame
(~0.8ms). The -90% line for spread (55.18 -> 47.64) still needs the
ttl arc (~2.1ms) and the Unsafe cluster diet (~1-2ms) — the campaign
continues after B22.
