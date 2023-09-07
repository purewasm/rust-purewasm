pub struct Event<P, R, S> {
    wam_id: Vec<u8>,
    payload: P,
    result: R,
    state: S,
}

