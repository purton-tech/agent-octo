use monty::{MontyObject, PrintWriter, ResourceTracker, RunProgress, Snapshot};
use tracing::info;

pub fn handle_bitcoin_price_call<T: ResourceTracker>(
    args: &[MontyObject],
    kwargs: &[(MontyObject, MontyObject)],
    state: Snapshot<T>,
) -> anyhow::Result<RunProgress<T>> {
    if !kwargs.is_empty() {
        anyhow::bail!("bitcoin_price() does not accept keyword arguments");
    }

    let [MontyObject::String(currency)] = args else {
        anyhow::bail!("bitcoin_price() expects exactly one string currency argument");
    };

    info!(currency = %currency, "python tool returning fake bitcoin price");

    let mut writer = PrintWriter::Stdout;
    Ok(state.run(MontyObject::Float(13.0), &mut writer)?)
}
