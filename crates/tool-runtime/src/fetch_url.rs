use monty::{MontyObject, PrintWriter, ResourceTracker, RunProgress, Snapshot};
use tracing::info;

pub async fn fetch_url(url: &str) -> anyhow::Result<String> {
    let parsed = reqwest::Url::parse(url)?;
    if !matches!(parsed.scheme(), "http" | "https") {
        anyhow::bail!("fetch_url() only allows http:// and https:// URLs");
    }

    let response = reqwest::get(parsed).await?;
    Ok(response.text().await?)
}

pub async fn handle_fetch_url_call<T: ResourceTracker>(
    args: &[MontyObject],
    kwargs: &[(MontyObject, MontyObject)],
    state: Snapshot<T>,
) -> anyhow::Result<RunProgress<T>> {
    if !kwargs.is_empty() {
        anyhow::bail!("fetch_url() does not accept keyword arguments");
    }

    let [MontyObject::String(url)] = args else {
        anyhow::bail!("fetch_url() expects exactly one string URL argument");
    };

    info!(url = %url, "python tool fetching url");
    let body = fetch_url(url).await?;
    info!(url = %url, bytes = body.len(), "python tool fetched url");

    let mut writer = PrintWriter::Stdout;
    Ok(state.run(MontyObject::String(body), &mut writer)?)
}
