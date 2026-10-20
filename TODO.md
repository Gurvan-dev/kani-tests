# TODO

- Try to find out what goes from instant for basic memset to 2min on a `sb`
  memset
- It seems like both memset9 and memset10 take just as long to work, could
  either be bitvectors or just taking the global pointer?
- Try crate rustbitvec
- Claim: Support for intrinsics for x86
  + Some instrinsics support, seems incomplete.
  + https://github.com/diffblue/cbmc/issues/7718
  + [Seems like some commit tried to add this?](https://github.com/tautschnig/cbmc/commit/f83878527594e28ae420cf0012edd5cce2bc4fa1)
- cbmc exposes bitvector, what about kani?
  + seemsl like it might have some support for [std::simd](https://doc.rust-lang.org/std/simd/index.html)?
- explosion dans cbmc ou solver?
- voir ce qui est difficile dans vérifier les pointeurs
- voir diff entre memset17 et memset18
