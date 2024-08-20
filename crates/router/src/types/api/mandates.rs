use api_models::mandates;
pub use api_models::mandates::{MandateId, MandateResponse, MandateRevokedResponse};
use common_utils::ext_traits::OptionExt;
use error_stack::ResultExt;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        errors::{self, RouterResult, StorageErrorExt},
        payment_methods,
    },
    newtype,
    routes::SessionState,
    types::{
        api, domain,
        storage::{self, enums as storage_enums},
    },
};

newtype!(
    pub MandateCardDetails = mandates::MandateCardDetails,
    derives = (Default, Debug, Deserialize, Serialize)
);

#[async_trait::async_trait]
pub(crate) trait MandateResponseExt: Sized {
    async fn from_db_mandate(
        state: &SessionState,
        key_store: domain::MerchantKeyStore,
        mandate: storage::Mandate,
        storage_scheme: storage_enums::MerchantStorageScheme,
    ) -> RouterResult<Self>;
}

#[async_trait::async_trait]
impl MandateResponseExt for MandateResponse {
    async fn from_db_mandate(
        state: &SessionState,
        key_store: domain::MerchantKeyStore,
        mandate: storage::Mandate,
        storage_scheme: storage_enums::MerchantStorageScheme,
    ) -> RouterResult<Self> {
        let db = &*state.store;
        let payment_method = db
            .find_payment_method(&mandate.payment_method_id, storage_scheme)
            .await
            .to_not_found_response(errors::ApiErrorResponse::PaymentMethodNotFound)?;

        let pm = payment_method
            .payment_method
            .get_required_value("payment_method")
            .change_context(errors::ApiErrorResponse::PaymentMethodNotFound)
            .attach_printable("payment_method not found")?;

        // let token_data = if payment_method.network_token_locker_id.is_some() && state.conf.locker.locker_enabled {
        //     let card = payment_methods::cards::get_card_from_locker(
        //         state,
        //         &payment_method.customer_id,
        //         &payment_method.merchant_id,
        //         payment_method
        //             .network_token_locker_id
        //             .as_ref()
        //             .unwrap_or(&payment_method.payment_method_id),
        //     )
        //     .await?;

        //     payment_methods::transformers::get_card_detail(&payment_method, card)
        //         .change_context(errors::ApiErrorResponse::InternalServerError)
        //         .attach_printable("Failed while getting card details")?
        // } else {
        //     payment_methods::cards::get_card_details_without_locker_fallback(
        //         &payment_method,
        //         state,
        //         &key_store,
        //     )
        //     .await?
        // };

        let card = if pm == storage_enums::PaymentMethod::Card {
            // if locker is disabled , decrypt the payment method data
            // let card_details = if state.conf.locker.locker_enabled {
            //     let card = payment_methods::cards::get_card_from_locker(
            //         // get network token details here? instead of that status check?
            //         state,
            //         &payment_method.customer_id,
            //         &payment_method.merchant_id,
            //         payment_method
            //             .locker_id
            //             .as_ref()
            //             .unwrap_or(&payment_method.payment_method_id),
            //     )
            //     .await?;

            //     payment_methods::transformers::get_card_detail(&payment_method, card)
            //         .change_context(errors::ApiErrorResponse::InternalServerError)
            //         .attach_printable("Failed while getting card details")?
            let token_data = if payment_method.network_token_locker_id.is_some()
                && state.conf.locker.locker_enabled
            {
                let card = payment_methods::cards::get_card_from_locker(
                    state,
                    &payment_method.customer_id,
                    &payment_method.merchant_id,
                    payment_method
                        .network_token_locker_id
                        .as_ref()
                        .unwrap_or(&payment_method.payment_method_id),
                )
                .await?;

                payment_methods::transformers::get_card_detail(&payment_method, card)
                    .change_context(errors::ApiErrorResponse::InternalServerError)
                    .attach_printable("Failed while getting card details")?
            } else {
                payment_methods::cards::get_card_details_without_locker_fallback(
                    &payment_method,
                    state,
                    &key_store,
                )
                .await?
            };

            Some(MandateCardDetails::from(token_data).into_inner())
        } else {
            None
        };
        let payment_method_type = payment_method
            .payment_method_type
            .map(|pmt| pmt.to_string());
        Ok(Self {
            mandate_id: mandate.mandate_id,
            customer_acceptance: Some(api::payments::CustomerAcceptance {
                acceptance_type: if mandate.customer_ip_address.is_some() {
                    api::payments::AcceptanceType::Online
                } else {
                    api::payments::AcceptanceType::Offline
                },
                accepted_at: mandate.customer_accepted_at,
                online: Some(api::payments::OnlineMandate {
                    ip_address: mandate.customer_ip_address,
                    user_agent: mandate.customer_user_agent.unwrap_or_default(),
                }),
            }),
            card,
            status: mandate.mandate_status,
            payment_method: pm.to_string(),
            payment_method_type,
            payment_method_id: mandate.payment_method_id,
        })
    }
}

impl From<api::payment_methods::CardDetailFromLocker> for MandateCardDetails {
    fn from(card_details_from_locker: api::payment_methods::CardDetailFromLocker) -> Self {
        mandates::MandateCardDetails {
            last4_digits: card_details_from_locker.last4_digits,
            card_exp_month: card_details_from_locker.expiry_month.clone(),
            card_exp_year: card_details_from_locker.expiry_year.clone(),
            card_holder_name: card_details_from_locker.card_holder_name,
            card_token: card_details_from_locker.card_token,
            scheme: card_details_from_locker.scheme,
            issuer_country: card_details_from_locker.issuer_country,
            card_fingerprint: card_details_from_locker.card_fingerprint,
            card_isin: card_details_from_locker.card_isin,
            card_issuer: card_details_from_locker.card_issuer,
            card_network: card_details_from_locker.card_network,
            card_type: card_details_from_locker.card_type,
            nick_name: card_details_from_locker.nick_name,
        }
        .into()
    }
}
