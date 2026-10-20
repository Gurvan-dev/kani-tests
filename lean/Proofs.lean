import MemsetCore.Funs
open Aeneas Aeneas.Std Result ControlFlow Error

set_option maxHeartbeats 1000000

namespace memset_kani

/-- One iteration when the range is non-empty. -/
@[step]
theorem rv64.memset4_loop.body_cont
    (fill : Std.U8) (s e : Std.Usize) (inp : Slice Std.U8)
    (hlt : s.val < e.val) (hlen : inp.length = e.val) :
    rv64.memset4_loop.body fill { start := s, «end» := e } inp
      ⦃ r => ∃ s' : Std.Usize, ∃ inp' : Slice Std.U8,
          r = cont ({ start := s', «end» := e }, inp') ∧
          s'.val = s.val + 1 ∧
          inp'.length = inp.length ∧
          inp'.val = inp.val.set s.val fill ⦄ := by
  unfold rv64.memset4_loop.body
  step*
  all_goals first
    | (refine ⟨_, _, rfl, ?_, ?_, ?_⟩ <;> simp_all <;> omega)
    | skip

/-- Loop body when the range is exhausted. -/
@[step]
theorem rv64.memset4_loop.body_done
    (fill : Std.U8) (s e : Std.Usize) (inp : Slice Std.U8)
    (hge : ¬ s.val < e.val) :
    rv64.memset4_loop.body fill { start := s, «end» := e } inp
      ⦃ r => r = done inp ⦄ := by
  unfold rv64.memset4_loop.body
  step*

/-- Loop spec, by induction on the remaining iterations. -/
theorem rv64.memset4_loop.spec (fill : Std.U8) :
    ∀ (k : Nat) (s e : Std.Usize) (inp : Slice Std.U8),
      e.val - s.val = k → s.val ≤ e.val → inp.length = e.val →
      (∀ j, j < s.val → inp.val[j]! = fill) →
      rv64.memset4_loop { start := s, «end» := e } inp fill
        ⦃ res => res.length = inp.length ∧
                 ∀ j, j < res.length → res.val[j]! = fill ⦄ := by
  intro k
  induction k with
  | zero =>
    intro s e inp hk hse hlen hinv
    have hge : ¬ s.val < e.val := by omega
    unfold rv64.memset4_loop
    rw [loop]
    step with rv64.memset4_loop.body_done fill s e inp hge
    simp_all
    intros j a
    simp_all [←(hinv j (by omega))]
  | succ k ih =>
    intro s e inp hk hse hlen hinv
    have hlt : s.val < e.val := by omega
    unfold rv64.memset4_loop
    rw [loop]
    step with rv64.memset4_loop.body_cont fill s e inp hlt hlen
      as ⟨r, s', inp', hr, hs', hlen', hval'⟩
    subst hr
    have hinv' : ∀ j, j < s'.val → inp'.val[j]! = fill := by
      intro j hj
      rw [hval']
      by_cases hjs : j = s.val
      · subst hjs
        simp [show s.val < inp.val.length from by
          have : inp.length = inp.val.length := rfl
          omega]
      · have hj' : j < s.val := by omega
        simp [List.getElem!_eq_getElem?_getD, hjs, Ne.symm hjs] at *
        have := hinv j hj'
        simpa [List.getElem!_eq_getElem?_getD] using this
    -- the remaining goal is the recursive call `loop ...`; unfold back to the spec
    have := ih s' e inp' (by omega) (by omega) (by omega) hinv'
    -- `rv64.memset4_loop` was unfolded above, so refold it before using `this`
    simp only [← rv64.memset4_loop] at *   -- may need adjusting
    step with ih s' e inp' (by omega) (by omega) (by omega) hinv' as ⟨res, h1, h2⟩
    exact ⟨by omega, h2⟩

/-- `memset4` never fails and sets every byte to `fill_value`. -/
theorem rv64.memset4.spec (inp : Slice Std.U8) (fill_value : Std.U8) :
    rv64.memset4 inp fill_value
      ⦃ res => res.length = inp.length ∧
               ∀ j, j < res.length → res.val[j]! = fill_value ⦄ := by
  unfold rv64.memset4
  have hlen : (Slice.len inp).val = inp.length := by simp
  step*
  -- if `step*` can't discharge the loop call, use:
  -- step with rv64.memset4_loop.spec fill_value (Slice.len inp).val 0#usize
  --   (Slice.len inp) inp (by simp) (by simp) hlen.symm (by intro j hj; simp at hj)

end memset_kani
