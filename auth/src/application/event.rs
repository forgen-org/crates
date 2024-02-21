use crate::domain;
use crate::domain::scalar::UserId;

#[derive(Clone)]
pub enum Event {
    DomainEvent(domain::Event),
    UserProjected(UserId),
}

impl Event {
    pub fn to_domain_events(events: Vec<Event>) -> Vec<domain::Event> {
        events
            .into_iter()
            .filter_map(|event| match event {
                Event::DomainEvent(event) => Some(event),
                _ => None,
            })
            .collect()
    }

    pub fn from_domain_events(events: Vec<domain::Event>) -> Vec<Event> {
        events
            .into_iter()
            .map(|event| Event::DomainEvent(event))
            .collect()
    }
}
