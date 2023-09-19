use anyhow::{anyhow, Result};

use slot_clock::{SlotClock, SystemTimeSlotClock};

pub const EPOCH_PER_YEAR: u64 = 82125;
pub const SLOT_PER_EPOCH: u64 = 32;
pub const DEPOSIT_AMOUNT: u64 = 32_000_000_000;

pub fn get_current_epoch(clock: &SystemTimeSlotClock) -> Result<u64> {
    let current_epoch = clock.now().ok_or(anyhow!("clock now"))? / SLOT_PER_EPOCH;
    Ok(current_epoch.as_u64())
}

pub fn caculate_arp(
    clock: &SystemTimeSlotClock,
    active_epoch: u64,
    accumulative_protocol_reward: u64,
) -> anyhow::Result<f64> {
    let epoch_range = get_current_epoch(clock)?.saturating_sub(active_epoch);
    let arp: f64 = (accumulative_protocol_reward as f64 / epoch_range as f64
        * EPOCH_PER_YEAR as f64
        / 32_000_000_000.0) as f64;
    Ok(arp)
}
