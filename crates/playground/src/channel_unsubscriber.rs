// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::playground::Playground;
use anyhow::{anyhow, Result};
use base::snp::upsetter_simple_client::UserUnsubscribeRequest;

impl Playground {
    pub(crate) async fn channel_unsubscribe(
        &mut self,
        client_name: &str,
        channel_name: &str,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_name);
        if client.is_none() {
            return Err(anyhow!("unknown client"));
        }

        let channel_bundle = self.channels.get(channel_name);
        if channel_bundle.is_none() {
            return Err(anyhow!("channel bundle not found"));
        }

        client
            .unwrap()
            .user_unsubscribe_from_status_updates(UserUnsubscribeRequest {
                channel_bundle: Some(channel_bundle.unwrap().clone()),
            })
            .await
            .map_err(|e| anyhow!(format!("error subscribing: {:?}", e)))?
            .into_inner();

        println!("🖖 unsubscribed from channel {}", channel_name);
        Ok(())
    }
}
