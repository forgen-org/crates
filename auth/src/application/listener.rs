use super::port::*;
use super::signal::Signal;
use forgen::*;

pub struct RecomputeUser;

impl<R> Listener<R> for RecomputeUser
where
    R: EventStore + UserRepository,
{
    type Signal = Signal;

    fn listen(&self, runtime: &R, signal: &Self::Signal) {
        if let Signal::EventsEmitted(events, metadata) = &signal {
            if let Some(user_id) = &metadata.user_id {
                let mut user = UserRepository::find_by_user_id(runtime, user_id)
                    .unwrap()
                    .unwrap_or_default();
                user.apply_all(events);
                UserRepository::save(runtime, user_id, &user).unwrap();
            }
        }
    }
}
