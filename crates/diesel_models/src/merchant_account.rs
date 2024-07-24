use common_utils::{encryption::Encryption, id_type, pii};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};

use crate::enums as storage_enums;
#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "merchant_account_v2")
))]
use crate::schema::merchant_account;
#[cfg(all(feature = "v2", feature = "merchant_account_v2"))]
use crate::schema_v2::merchant_account;

#[cfg(all(
    any(feature = "v1", feature = "v2"),
    not(feature = "merchant_account_v2")
))]
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    Identifiable,
    serde::Serialize,
    Queryable,
    Selectable,
    router_derive::DebugAsDisplay,
)]
#[diesel(table_name = merchant_account, primary_key(merchant_id), check_for_backend(diesel::pg::Pg))]
pub struct MerchantAccount {
    pub id: i32,
    pub merchant_id: id_type::MerchantId,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub merchant_name: Option<Encryption>,
    pub merchant_details: Option<Encryption>,
    pub webhook_details: Option<serde_json::Value>,
    pub sub_merchants_enabled: Option<bool>,
    pub parent_merchant_id: Option<id_type::MerchantId>,
    pub publishable_key: Option<String>,
    pub storage_scheme: storage_enums::MerchantStorageScheme,
    pub locker_id: Option<String>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub primary_business_details: serde_json::Value,
    pub intent_fulfillment_time: Option<i64>,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub organization_id: id_type::OrganizationId,
    pub is_recon_enabled: bool,
    pub default_profile: Option<String>,
    pub recon_status: storage_enums::ReconStatus,
    pub payment_link_config: Option<serde_json::Value>,
    pub pm_collect_link_config: Option<serde_json::Value>,
}

#[cfg(all(feature = "v2", feature = "merchant_account_v2"))]
#[derive(
    Clone,
    Debug,
    serde::Deserialize,
    Identifiable,
    serde::Serialize,
    Queryable,
    router_derive::DebugAsDisplay,
    Selectable,
)]
#[diesel(table_name = merchant_account, primary_key(merchant_id), check_for_backend(diesel::pg::Pg))]
pub struct MerchantAccount {
    pub merchant_id: id_type::MerchantId,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub merchant_name: Option<Encryption>,
    pub merchant_details: Option<Encryption>,
    pub webhook_details: Option<serde_json::Value>,
    pub sub_merchants_enabled: Option<bool>,
    pub parent_merchant_id: Option<common_utils::id_type::MerchantId>,
    pub publishable_key: Option<String>,
    pub storage_scheme: storage_enums::MerchantStorageScheme,
    pub locker_id: Option<String>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub primary_business_details: serde_json::Value,
    pub intent_fulfillment_time: Option<i64>,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub organization_id: id_type::OrganizationId,
    pub is_recon_enabled: bool,
    pub default_profile: Option<String>,
    pub recon_status: storage_enums::ReconStatus,
    pub payment_link_config: Option<serde_json::Value>,
    pub pm_collect_link_config: Option<serde_json::Value>,
}

impl MerchantAccount {
    pub fn get_id(&self) -> &id_type::MerchantId {
        &self.merchant_id
    }
}

#[derive(Clone, Debug, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = merchant_account)]
pub struct MerchantAccountNew {
    pub merchant_id: id_type::MerchantId,
    pub merchant_name: Option<Encryption>,
    pub merchant_details: Option<Encryption>,
    pub return_url: Option<String>,
    pub webhook_details: Option<serde_json::Value>,
    pub sub_merchants_enabled: Option<bool>,
    pub parent_merchant_id: Option<id_type::MerchantId>,
    pub enable_payment_response_hash: Option<bool>,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: Option<bool>,
    pub publishable_key: Option<String>,
    pub locker_id: Option<String>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub primary_business_details: serde_json::Value,
    pub intent_fulfillment_time: Option<i64>,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub organization_id: id_type::OrganizationId,
    pub is_recon_enabled: bool,
    pub default_profile: Option<String>,
    pub recon_status: storage_enums::ReconStatus,
    pub payment_link_config: Option<serde_json::Value>,
    pub pm_collect_link_config: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = merchant_account)]
pub struct MerchantAccountUpdateInternal {
    pub merchant_name: Option<Encryption>,
    pub merchant_details: Option<Encryption>,
    pub return_url: Option<String>,
    pub webhook_details: Option<serde_json::Value>,
    pub sub_merchants_enabled: Option<bool>,
    pub parent_merchant_id: Option<id_type::MerchantId>,
    pub enable_payment_response_hash: Option<bool>,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: Option<bool>,
    pub publishable_key: Option<String>,
    pub storage_scheme: Option<storage_enums::MerchantStorageScheme>,
    pub locker_id: Option<String>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub primary_business_details: Option<serde_json::Value>,
    pub modified_at: Option<time::PrimitiveDateTime>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub organization_id: Option<id_type::OrganizationId>,
    pub is_recon_enabled: bool,
    pub default_profile: Option<Option<String>>,
    pub recon_status: Option<storage_enums::ReconStatus>,
    pub payment_link_config: Option<serde_json::Value>,
    pub pm_collect_link_config: Option<serde_json::Value>,
}
