use crate::database::*;
use crate::util::result::{Error, Result};

use rocket_contrib::json::JsonValue;

#[get("/<target>")]
pub async fn req(user: User, target: Ref) -> Result<JsonValue> {
    let target = target.fetch_channel().await?;

    let perm = permissions::channel::calculate(&user, &target).await;
    if !perm.get_view() {
        Err(Error::LabelMe)?
    }

    Ok(json!(target))
}