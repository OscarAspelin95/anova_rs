use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "hasValidSubscription")]
    pub has_valid_subscription: bool,
    #[serde(rename = "isLegacyAccount")]
    pub is_legacy_account: bool,
    #[serde(rename = "renewalPeriod")]
    pub renewal_period: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatePayload {
    #[serde(rename = "isConnectedToAlexa")]
    pub is_connected_to_alexa: bool,
    #[serde(rename = "isConnectedToGoogleHome")]
    pub is_connected_to_google_home: bool,
    #[serde(rename = "sousVideSubscription")]
    pub sous_vide_subscription: Subscription,
    #[serde(rename = "ovenSubscription")]
    pub oven_subscription: Subscription,
}
