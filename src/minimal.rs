use softcore_rv64::prelude::*;

pub const ELEM_COUNT_MAX: usize = 4;
pub const ELEM_SIZE: usize = 8;
pub const VECTOR_REG_SIZE: i128 = (ELEM_COUNT_MAX * ELEM_SIZE) as i128;

pub enum RegIdx {
    V0,
    V1,
    V2,
    V3,
}

/* State */

static mut ELEM_COUNT_CURRENT: usize = ELEM_COUNT_MAX - 1;
static mut V1: BitDynamic = BitDynamic::zeros(VECTOR_REG_SIZE);
static mut V2: BitDynamic = BitDynamic::zeros(VECTOR_REG_SIZE);
static mut V3: BitDynamic = BitDynamic::zeros(VECTOR_REG_SIZE);

/* Reading and writing to vector registers without extensions */

fn rv(reg: RegIdx) -> BitDynamic {
    match reg {
        RegIdx::V0 => BitDynamic::zeros(VECTOR_REG_SIZE),
        RegIdx::V1 => unsafe { V1 },
        RegIdx::V2 => unsafe { V2 },
        RegIdx::V3 => unsafe { V3 },
    }
}

fn wv(reg: RegIdx, v: BitDynamic) {
    match reg {
        RegIdx::V0 => (),
        RegIdx::V1 => unsafe { V1 = v },
        RegIdx::V2 => unsafe { V2 = v },
        RegIdx::V3 => unsafe { V3 = v },
    }
}

/* Reading and writing to vector registers with extensions */

fn vectobit(v: BoundedVec<BitDynamic, ELEM_COUNT_MAX>) -> BitDynamic {
    let mut res = BitDynamic::new(VECTOR_REG_SIZE, 0);
    for i in 0..v.len() {
        assert!(v[i].len() == ELEM_SIZE as i128);
        res = res.set_subrange(v[i], ((i + 1) * ELEM_SIZE - 1) as u64, (i * ELEM_SIZE) as u64); // TODO: Should not be `i` here
    }
    res
}

fn rv_extend(reg: RegIdx) -> BoundedVec<BitDynamic, ELEM_COUNT_MAX> {
    let vl = unsafe { ELEM_COUNT_CURRENT };
    let reg_val = rv(reg);
    let mut res = BoundedVec::new();
    for i in 0..vl {
        res.push(reg_val.get_subrange(((i + 1) * ELEM_SIZE) as i128, (i * ELEM_SIZE) as i128))
    }
    res
}

fn wv_extend(reg: RegIdx, v: BoundedVec<BitDynamic, ELEM_COUNT_MAX>) {
    wv(reg, vectobit(v))
}

/* Instructions */

pub fn vset(i: usize) -> usize {
    unsafe {
        ELEM_COUNT_CURRENT = i.min(ELEM_COUNT_MAX - 1);
        ELEM_COUNT_CURRENT
    }
}

pub fn memmove(regidx: RegIdx, value: BitDynamic) {
    let vl = unsafe { ELEM_COUNT_CURRENT };
    assert!(value.len() == ELEM_SIZE as i128);
    assert!(vl < ELEM_COUNT_MAX);
    let mut elements: BoundedVec<BitDynamic, ELEM_COUNT_MAX> = BoundedVec::new();
    for _ in 0..vl {
        elements.push(value);
    }
    wv_extend(regidx, elements);
}

pub fn memload8(addr: *mut u8, regidx: RegIdx) {
    let vl = unsafe { ELEM_COUNT_CURRENT };
    let mut elements: BoundedVec<BitDynamic, ELEM_COUNT_MAX> = BoundedVec::new();

    for i in 0..vl {
        let val = unsafe { core::ptr::read(addr.wrapping_add(i)) };
        elements.push(bvd(8, val as u64));
    }

    wv_extend(regidx, elements);
}

pub fn memstore8(addr: BitDynamic, regidx: RegIdx) {
    let vl = unsafe { ELEM_COUNT_CURRENT };
    let reg_val = rv(regidx);
    let addr = addr.unsigned() as usize;
    for i in 0..vl {
        let addr = core::ptr::with_exposed_provenance_mut::<u8>(addr.wrapping_add(i));
        unsafe {
            *addr = reg_val.get_subrange(((i + 1) * 8) as i128, (i * 8) as i128).to_raw_le()[0];
        }
    }
}
