use std::time::{Duration, Instant, SystemTime};

use vixen::BusDevice;
use vixen::devices::errors::{BusError, BusResult};

mod time;

pub use time::{Time, Timer};

#[derive(Debug)]
pub struct RealTimeClock {
    time: Time,
    timer: Option<Timer>,
    last_tick_time: Instant,
}

#[allow(clippy::cast_possible_truncation, reason = "Wrapping is intentional as Vixen is 32-bit")]
impl RealTimeClock {
    #[must_use]
    pub fn new(time: Time) -> Self {
        Self {
            time,
            timer: None,
            last_tick_time: Instant::now(),
        }
    }

    #[must_use]
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();

        let secs = duration.as_secs() as u32;
        let nanos = duration.subsec_nanos();
        let time = Time::new(secs, nanos);

        Self {
            time,
            timer: None,
            last_tick_time: Instant::now(),
        }
    }

    #[inline]
    #[must_use]
    pub fn secs(&self) -> u32 {
        self.time.secs()
    }

    #[inline]
    #[must_use]
    pub fn nanos(&self) -> u32 {
        self.time.nanos()
    }

    #[inline]
    pub fn add(&mut self, duration: Duration) {
        self.time.add(duration);
    }

    #[inline]
    pub fn add_secs(&mut self, secs: u32) {
        self.time.add_secs(secs);
    }

    #[inline]
    pub fn add_nanos(&mut self, nanos: u32) {
        self.time.add_nanos(nanos);
    }

    #[inline]
    pub fn set_secs(&mut self, secs: u32) {
        self.time.set_secs(secs);
    }

    #[inline]
    pub fn set_nanos(&mut self, nanos: u32) {
        self.time.set_nanos(nanos);
    }

    #[must_use]
    pub fn timer_secs(&self) -> u32 {
        self.timer.as_ref()
            .map(|t| t.remaining(&self.time))
            .map(|t| t.secs())
            .unwrap_or_default()
    }

    #[must_use]
    pub fn timer_nanos(&self) -> u32 {
        self.timer.as_ref()
            .map(|t| t.remaining(&self.time))
            .map(|t| t.nanos())
            .unwrap_or_default()
    }

    pub fn adjust_timer_secs(&mut self, secs: u32) {
        self.get_timer()
            .adjust_secs(secs);
    }

    pub fn adjust_timer_nanos(&mut self, nanos: u32) {
        self.get_timer()
            .adjust_nanos(nanos);
    }

    pub fn has_timer(&self) -> bool {
        self.timer.is_some()
    }

    pub fn clear_timer(&mut self) {
        self.timer = None;
    }

    fn get_timer(&mut self) -> &mut Timer {
        self.timer.get_or_insert_with(|| Timer::new(self.time.clone()))
    }

    fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now - self.last_tick_time;
        self.last_tick_time = now;

        // Y2K38 says haiii
        self.add(elapsed);
    }

    fn update_timer(&mut self) -> bool {
        let trigger = self.timer.as_ref().is_some_and(|t| t.expired(&self.time));

        if trigger {
            self.clear_timer();
            true
        } else {
            false
        }
    }
}

impl BusDevice for RealTimeClock {
    fn get_port_count(&self) -> u32 {
        5
    }

    fn get_base_address(&self) -> u32 {
        0x0400_020C
    }

    fn read_port(&mut self, index: u32) -> BusResult<u32> {
        match index {
            0 => Ok(self.secs()),
            1 => Ok(self.nanos()),
            2 => Ok(self.timer_secs()),
            3 => Ok(self.timer_nanos()),
            4 => Ok(u32::from(self.has_timer())),
            _ => Err(BusError::PortOutOfRange),
        }
    }

    fn write_port(&mut self, index: u32, data: u32) -> BusResult<()> {
        match index {
            0 => self.set_secs(data),
            1 => self.set_nanos(data),
            2 => self.adjust_timer_secs(data),
            3 => self.adjust_timer_nanos(data),
            4 => self.clear_timer(),
            _ => return Err(BusError::PortOutOfRange),
        }

        Ok(())
    }

    fn tick(&mut self) -> BusResult<()> {
        self.update();

        if self.update_timer() {
            return Err(BusError::DeviceEvent)
        }

        Ok(())
    }
}
