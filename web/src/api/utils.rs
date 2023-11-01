use anyhow::anyhow;
use slot_clock::{Slot, SlotClock, SystemTimeSlotClock};

pub fn epoch_to_timestamp(clock: &SystemTimeSlotClock, epoch: u64) -> anyhow::Result<u64> {
    // TODO: replace `slots_per_epoch` of
    let slot = Slot::new(epoch * 32);
    let time = clock.start_of(slot).ok_or(anyhow!("start of slot error"))?;
    Ok(time.as_secs())
}
