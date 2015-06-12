use std::ops::{Add,Sub, Mul, Div, Shl, Shr};

#[derive(Copy, Clone)]
pub struct Fixed32 {
    val: i32,
    f: u8,
    m: u8,
    msig: i32
}

impl Fixed32 {
    pub fn new(qf: u8) -> Fixed32 {
        Fixed32 {
            val: 0,
            f: qf,
            m: 32-qf,
            msig: 1 << qf
        }
    }

    pub fn setf(&mut self, value: f32) {
        let x: f32;
        let y: f32;

        if value >= 1.0 {
            x = value.floor();
            y = value%x;
        } else {
            x = 0.0;
            y = value;
        }
        self.val = ( (x as i32) << self.f) | ((self.msig as f32 *y) as i32);

    }

    pub fn get_qf(self) -> u8 {
        self.f
    }

    pub fn max_int(self) -> i32 {
        let mut g = 0;
        for i in 0..self.m-1 {
            g |= 1 << i;
        }

        g
    }

    pub fn geti(self) -> i32 {
        self.val
    }

    pub fn scaling(self) -> f32 {
        1.0/(2.0f32.powi(self.f as i32))
    }
}

impl Add for Fixed32 {
    type Output = Fixed32;

    fn add(self, other: Fixed32) -> Fixed32 {
        Fixed32 {
            val: self.val + other.geti(),
            f: self.f,
            m: self.m,
            msig: 1 << self.f
        }
    }
}

impl Sub for Fixed32 {
    type Output = Fixed32;

    fn sub(self, other: Fixed32) -> Fixed32 {
        Fixed32 {
            val: self.val - other.geti(),
            f: self.f,
            m: self.m,
            msig: 1 << self.f
        }
    }
}

impl Mul for Fixed32 {
    type Output = Fixed32;

    fn mul(self, other: Fixed32) -> Fixed32 {

        let shr = (self.f+other.get_qf()) - self.f;

        let nv: i64 = (self.val as i64 * other.geti() as i64) >> shr;

        Fixed32 {
            val: nv as i32,
            f: self.f,
            m: self.m,
            msig: 1 << self.f
        }
    }
}

impl Div for Fixed32 {
    type Output = Fixed32;

    fn div(self, other: Fixed32) -> Fixed32 {

        let nv: i64 =  ((self.val as i64) << self.f) / other.geti() as i64;

        Fixed32 {
            val: nv as i32,
            f: self.f,
            m: self.m,
            msig: 1 << self.f
        }
    }
}

impl Shl<u32> for Fixed32 {
    type Output = Fixed32;

    fn shl(self, value: u32) -> Fixed32 {

        Fixed32 {
            val: self.val << value,
            f: self.f,
            m: self.m,
            msig: 1 << self.f
        }
    }
}

impl Shr<u32> for Fixed32 {
    type Output = Fixed32;

    fn shr(self, value: u32) -> Fixed32 {

        Fixed32 {
            val: self.val >> value,
            f: self.f,
            m: self.m,
            msig: 1 << self.f
        }
    }
}

pub fn qf_res(res: f32) -> u32 {
    (1.0/res).log(2.0).ceil() as u32
}

pub fn qf_scaling(qf: u32) -> f32 {

    1.0/(2.0f32.powi(qf as i32))
}
