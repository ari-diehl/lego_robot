use ev3dev_lang_rust::Ev3Result;

pub trait ManualControl<T> {
    fn controls(&self, command: T) -> Ev3Result<()>;
}
