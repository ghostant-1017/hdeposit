mod el_fee;
mod withdrawals;
mod cl_reward;

pub enum TaskState {
    ELFee,
    Withdrawal,
}