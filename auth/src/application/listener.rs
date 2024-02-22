use super::port::*;
use super::signal::Signal;
use forgen::*;

pub struct RecomputeUser(pub Signal);

impl<R> Listener<R> for RecomputeUser
where
    R: EventStore + SignalBus + UserRepository,
{
    fn listen(&self, runtime: &R) {
        if let Signal::EventsEmitted(events, metadata) = &self.0 {
            if let Some(user_id) = &metadata.user_id {
                let mut user = UserRepository::find_by_user_id(runtime, user_id)
                    .unwrap()
                    .unwrap_or_default();
                user.apply_all(events);
                UserRepository::save(runtime, &user).unwrap();
                SignalBus::publish(runtime, Signal::UserProjected(metadata.clone()));
            }
        }
    }
}
