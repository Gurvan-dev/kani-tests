use softcore_rv64::prelude::*;

pub const ELEM_COUNT_MAX: usize = 4;
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

static mut X1: BitDynamic = BitDynamic::zeros(REG_SIZE);
static mut X2: BitDynamic = BitDynamic::zeros(REG_SIZE);
static mut X3: BitDynamic = BitDynamic::zeros(REG_SIZE);

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

pub fn rx(reg: RegIdx) -> BitDynamic {
    match reg {
        RegIdx::X0 => BitDynamic::zeros(REG_SIZE),
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

pub fn wx(reg: RegIdx, v: BitDynamic) {
    assert!(v.len() == REG_SIZE);
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
    let mut res = BitDynamic::new(VECTOR_REG_SIZE, 0);
    let mut size: i128 = 0;
    for i in 0..vs.len() {
        res = res.set_subrange(vs[i], (size + vs[i].len() - 1) as u64, size as u64);
        size += vs[i].len();
    }
    wv(reg, res);
}

/* Instructions */

pub fn xmove(regidx: RegIdx, value: BitDynamic) {
    assert!(value.len() == REG_SIZE);
    wx(regidx, value);
}

/* Vector Instructions */

pub fn vset(count: usize, sew: VecSize) -> usize {
    unsafe {
        VL = count.min(ELEM_COUNT_MAX - 1);
        SEW = sew;
        VL
    }
}

pub fn vxmove(regidx: RegIdxVec, value: BitDynamic) {
    let vl = vl();
    let sew = sew();
    assert!(value.len() == 8 * sew as i128);
    assert!(vl < ELEM_COUNT_MAX);
    let mut elements: BoundedVec<BitDynamic, ELEM_COUNT_MAX> = BoundedVec::new();
    for _ in 0..vl {
        elements.push(value);
    }
    wv_extend(regidx, elements);
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
