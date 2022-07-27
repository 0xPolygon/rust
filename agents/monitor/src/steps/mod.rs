pub(crate) mod between;
pub(crate) mod combine;
pub(crate) mod dispatch_wait;
pub(crate) mod e2e;
pub(crate) mod producer;
pub(crate) mod relay_wait;
pub(crate) mod terminal;
pub(crate) mod update_wait;

#[derive(Debug)]
pub(crate) enum TaskResult<T> {
    Recoverable {
        task: T,
        err: eyre::Report,
    },
    Unrecoverable {
        err: eyre::Report,
        worth_logging: bool,
    },
}
