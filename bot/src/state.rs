use crate::types::{ChitChat, HR, PurchaseKind};

pub async fn state_buy(chit: &ChitChat) -> HR {
    let state = chit.get_or_default().await?;

    Ok(())
}
