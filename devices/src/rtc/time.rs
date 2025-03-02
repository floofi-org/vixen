use std::time::Duration;
use std::ops::{Add, Sub};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    secs: u32,
    nanos: u32,
}

#[derive(Debug, Default)]
pub struct Timer {
    start: Time,
    duration: Time,
}

impl Time {
    const NANOS_PER_SEC: u32 = 1_000_000_000;

    #[must_use]
    pub fn new(secs: u32, nanos: u32) -> Self {
        Self {
            secs,
            nanos,
        }
    }

    #[inline]
    #[must_use]
    pub fn secs(&self) -> u32 {
        self.secs
    }

    #[inline]
    #[must_use]
    pub fn nanos(&self) -> u32 {
        self.nanos
    }

    pub fn add(&mut self, duration: Duration) {
        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();

        self.add_secs(secs as u32);
        self.add_nanos(nanos);
    }

    #[inline]
    pub fn add_secs(&mut self, secs: u32) {
        self.secs = self.secs.wrapping_add(secs);
    }

    #[inline]
    pub fn sub_secs(&mut self, secs: u32) {
        self.secs = self.secs.wrapping_sub(secs);
    }

    pub fn add_nanos(&mut self, nanos: u32) {
        let secs = nanos / Self::NANOS_PER_SEC;
        let nanos = nanos % Self::NANOS_PER_SEC;

        self.secs = self.secs.wrapping_add(secs);
        self.nanos += nanos;

        if self.nanos >= Self::NANOS_PER_SEC {
            self.nanos -= Self::NANOS_PER_SEC;
            self.secs = self.secs.wrapping_add(1);
        }
    }

    pub fn sub_nanos(&mut self, nanos: u32) {
        let secs = nanos / Self::NANOS_PER_SEC;
        let nanos = nanos % Self::NANOS_PER_SEC;

        self.secs = self.secs.wrapping_sub(secs);

        if self.nanos > nanos {
            self.nanos -= nanos;
        } else {
            self.nanos = Self::NANOS_PER_SEC - self.nanos.abs_diff(nanos);
            self.secs = self.secs.wrapping_sub(1);
        }
    }

    #[inline]
    pub fn set_secs(&mut self, secs: u32) {
        self.secs = secs;
    }

    #[inline]
    pub fn set_nanos(&mut self, nanos: u32) {
        self.nanos = nanos;
    }
}

impl Timer {
    pub fn new(start: Time) -> Self {
        Self {
            start,
            duration: Time::default(),
        }
    }

    #[inline]
    pub fn adjust_secs(&mut self, secs: u32) {
        self.duration.set_secs(secs);
    }

    #[inline]
    pub fn adjust_nanos(&mut self, nanos: u32) {
        self.duration.set_nanos(nanos);
    }

    pub fn remaining(&self, current_time: &Time) -> Time {
        self.end() - current_time
    }

    pub fn expired(&self, current_time: &Time) -> bool {
        *current_time > self.end()
    }

    fn end(&self) -> Time {
        self.start.clone() + &self.duration
    }
}

impl Add<&Self> for Time {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.add_secs(rhs.secs);
        self.add_nanos(rhs.nanos);

        self
    }
}

impl Sub<&Self> for Time {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self.sub_secs(rhs.secs);
        self.sub_nanos(rhs.nanos);

        self
    }
}
