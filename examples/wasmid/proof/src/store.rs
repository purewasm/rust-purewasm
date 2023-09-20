pub trait IdStore {
    fn get_event(&self, id: DigestId) -> Option<PersistedIdEvent>;
    fn get_context(&self, id: DigestId) -> Option<DigestId>;
    fn add_event(&mut self, id: DigestId, event: PersistedIdEvent);
}