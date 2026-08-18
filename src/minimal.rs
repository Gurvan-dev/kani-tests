use softcore_rv64::prelude::*;

pub const ELEM_COUNT_MAX: usize = 1;
pub const ELEM_SIZE: usize = 64;
pub const REG_SIZE: i128 = ELEM_SIZE as i128;
pub const VECTOR_REG_SIZE: i128 = (ELEM_COUNT_MAX * ELEM_SIZE) as i128;

#[derive(Copy, Clone)]
pub enum VecSize {
    S8,
    S16,
    S32,
    S64,
}

pub enum RegIdxVec {
    V0,
    V1,
    V2,
    V3,
}

pub enum RegIdx {
    X0,
    X1,
    X2,
    X3,
}

/* State */

static mut VL: usize = ELEM_COUNT_MAX - 1;
static mut SEW: VecSize = VecSize::S8;
static mut V1: BitDynamic = BitDynamic::zeros(VECTOR_REG_SIZE);
static mut V2: BitDynamic = BitDynamic::zeros(VECTOR_REG_SIZE);
static mut V3: BitDynamic = BitDynamic::zeros(VECTOR_REG_SIZE);

static mut X1: BitStatic::<REG_SIZE> = BitStatic::zeros();
static mut X2: BitStatic::<REG_SIZE> = BitStatic::zeros();
static mut X3: BitStatic::<REG_SIZE> = BitStatic::zeros();

/* Utils */

fn sew() -> usize {
    match unsafe { SEW } {
        VecSize::S8 => 1,
        VecSize::S16 => 2,
        VecSize::S32 => 3,
        VecSize::S64 => 4,
    }
}

fn vl() -> usize {
    unsafe { VL }
}

/* Reading and writing to registers without extensions */

pub fn rv(reg: RegIdxVec) -> BitDynamic {
    match reg {
        RegIdxVec::V0 => BitDynamic::zeros(VECTOR_REG_SIZE),
        RegIdxVec::V1 => unsafe { V1 },
        RegIdxVec::V2 => unsafe { V2 },
        RegIdxVec::V3 => unsafe { V3 },
    }
}

pub fn rx(reg: RegIdx) -> BitStatic::<REG_SIZE> {
    match reg {
        RegIdx::X0 => BitStatic::zeros(),
        RegIdx::X1 => unsafe { X1 },
        RegIdx::X2 => unsafe { X2 },
        RegIdx::X3 => unsafe { X3 },
    }
}

pub fn wv(reg: RegIdxVec, v: BitDynamic) {
    assert!(v.len() == VECTOR_REG_SIZE);
    match reg {
        RegIdxVec::V0 => (),
        RegIdxVec::V1 => unsafe { V1 = v },
        RegIdxVec::V2 => unsafe { V2 = v },
        RegIdxVec::V3 => unsafe { V3 = v },
    }
}

pub fn wx(reg: RegIdx, v: BitStatic::<REG_SIZE>) {
    match reg {
        RegIdx::X0 => (),
        RegIdx::X1 => unsafe { X1 = v },
        RegIdx::X2 => unsafe { X2 = v },
        RegIdx::X3 => unsafe { X3 = v },
    }
}

/* Reading and writing to vector registers with extensions */

/* TODO: Not used for now
fn rv_extend(reg: RegIdxVec) -> BoundedVec<BitDynamic, ELEM_COUNT_MAX> {
    let vl = unsafe { VL };
    let reg_val = rv(reg);
    let mut res = BoundedVec::new();
    for i in 0..vl {
        res.push(reg_val.get_subrange(((i + 1) * ELEM_SIZE) as i128, (i * ELEM_SIZE) as i128))
    }
    res
}
*/

fn wv_extend(reg: RegIdxVec, vs: BoundedVec<BitDynamic, ELEM_COUNT_MAX>) {
    let mut res: BitDynamic = BitDynamic::zeros(VECTOR_REG_SIZE);
    let mut size: i128 = 0;
    for i in 0..vs.len() {
        res = res.set_subrange(vs[i], (size + vs[i].len() - 1) as u64, size as u64);
        size += vs[i].len();
    }
    wv(reg, res);
}

/* Instructions */

pub fn xmove(regidx: RegIdx, value: BitStatic<REG_SIZE>) {
    wx(regidx, value);
}

/* Vector Instructions */

pub fn vset(count: usize, sew: VecSize) -> usize {
    unsafe {
        VL = count.min(ELEM_COUNT_MAX);
        SEW = sew;
        VL
    }
}

pub fn vxmove(regidx: RegIdxVec, value: BitDynamic) {
    let vl = vl();
    let sew = sew();
    assert!(value.len() == 8 * sew as i128);
    assert!(vl <= ELEM_COUNT_MAX);
    let mut elements: BoundedVec<BitDynamic, ELEM_COUNT_MAX> = BoundedVec::new();
    for _ in 0..vl {
        elements.push(value);
    }
    wv_extend(regidx, elements);
}

pub fn vadd(rd: RegIdxVec, r1: RegIdxVec, r2: RegIdxVec) {
    /* TODO */
}

pub fn load(addr: *mut u64, reg: RegIdx) {
    let val = unsafe { core::ptr::read(addr) };
    wx(reg, bv(val));
}

pub fn vload8(addr: *mut u8, regidx: RegIdxVec) {
    let vl = vl();
    let mut elements: BoundedVec<BitDynamic, ELEM_COUNT_MAX> = BoundedVec::new();

    for i in 0..vl {
        let val = unsafe { core::ptr::read(addr.wrapping_add(i)) };
        elements.push(bvd(8, val as u64));
    }

    wv_extend(regidx, elements);
}

pub fn store(addr: RegIdx, value: RegIdx) {
    let addr = core::ptr::with_exposed_provenance_mut::<u64>(rx(addr).unsigned() as usize);
    let value = rx(value).unsigned();
    unsafe {
        *addr = value as u64;
    }
}

pub fn vstore8(addr: RegIdx, regidx: RegIdxVec) {
    let vl = vl();
    let sew = sew();
    let reg_val = rv(regidx);
    let addr = rx(addr).unsigned() as usize;
    for i in 0..vl {
        let addr = core::ptr::with_exposed_provenance_mut::<u8>(addr.wrapping_add(i * sew));
        let value = reg_val.get_subrange(((i + 1) * 8 * sew) as i128, (i * 8 * sew) as i128).to_raw_le()[0];
        unsafe {
            *addr = value;
        }
    }
}

// Implementation ----------------------------------------------------------------------------------

pub fn memset1(mut input: &mut [u8], value: u8) {
    let mut len = input.len();
    while 0 < len {
        let vl = vset(len, VecSize::S8);
        assert!(0 < vl);
        xmove(RegIdx::X1, bv(input.as_mut_ptr() as u64));
        vxmove(RegIdxVec::V1, bvd(8, value as u64));
        vstore8(RegIdx::X1, RegIdxVec::V1);
        input = &mut input[vl..];
        len -= vl;
    }
}

pub fn memset2(mut input: &mut [u64], value: u64) {
    let mut len = input.len();
    while 0 < len {
        xmove(RegIdx::X1, bv(input.as_mut_ptr() as u64));
        xmove(RegIdx::X2, bv(value));
        store(RegIdx::X1, RegIdx::X2);
        input = &mut input[1..];
        len -= 1;
    }
}

// Verification ------------------------------------------------------------------------------------

#[test]
fn kani_minimal_memset() {
    const LEN: usize = 32;
    let fill_value: u8 = 1;
    let mut inp: [u8; LEN] = [0; LEN];

    memset(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value, "{} != {} at indice {}", inp[i], fill_value, i);
    }
}


#[cfg(kani)]
#[kani::proof]
fn kani_minimal_memset1() {
    const MEMSET_CHECK_LEN: usize = 32;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; MEMSET_CHECK_LEN] = kani::any();

    memset1(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_minimal_memset2() {
    const MEMSET_CHECK_LEN: usize = 32;
    let fill_value: u64 = kani::any();
    let mut inp: [u64; MEMSET_CHECK_LEN] = kani::any();

    memset2(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}
