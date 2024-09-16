use common_utils::events::{ApiEventMetric, ApiEventsType};
#[cfg(feature = "recon")]
use masking::PeekInterface;

#[cfg(feature = "dummy_connector")]
use crate::user::sample_data::SampleDataRequest;
#[cfg(feature = "recon")]
use crate::user::VerifyTokenResponse;
use crate::user::{
    dashboard_metadata::{
        GetMetaDataRequest, GetMetaDataResponse, GetMultipleMetaDataPayload, SetMetaDataRequest,
    },
    AcceptInviteFromEmailRequest, AuthSelectRequest, AuthorizeResponse, BeginTotpResponse,
    ChangePasswordRequest, ConnectAccountRequest, CreateInternalUserRequest,
    CreateUserAuthenticationMethodRequest, DashboardEntryResponse, ForgotPasswordRequest,
    GetSsoAuthUrlRequest, GetUserAuthenticationMethodsRequest, GetUserDetailsResponse,
    GetUserRoleDetailsRequest, GetUserRoleDetailsResponse, GetUserRoleDetailsResponseV2,
    InviteUserRequest, ListUsersResponse, ReInviteUserRequest, RecoveryCodes, ResetPasswordRequest,
    RotatePasswordRequest, SendEmailRequest, SendVerifyEmailRequest, SignUpRequest,
    SignUpWithMerchantIdRequest, SsoSignInRequest, SwitchMerchantRequest,
    SwitchOrganizationRequest, SwitchProfileRequest, TokenResponse, TwoFactorAuthStatusResponse,
    UpdateUserAccountDetailsRequest, UpdateUserAuthenticationMethodRequest, UserFromEmailRequest,
    UserMerchantCreate, VerifyEmailRequest, VerifyRecoveryCodeRequest, VerifyTotpRequest,
};

impl ApiEventMetric for DashboardEntryResponse {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::User {
            user_id: self.user_id.clone(),
        })
    }
}

#[cfg(feature = "recon")]
impl ApiEventMetric for VerifyTokenResponse {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::User {
            user_id: self.user_email.peek().to_string(),
        })
    }
}

common_utils::impl_api_event_type!(
    Miscellaneous,
    (
        SignUpRequest,
        SignUpWithMerchantIdRequest,
        ChangePasswordRequest,
        GetMultipleMetaDataPayload,
        GetMetaDataResponse,
        GetMetaDataRequest,
        SetMetaDataRequest,
        SwitchOrganizationRequest,
        SwitchMerchantRequest,
        SwitchProfileRequest,
        CreateInternalUserRequest,
        UserMerchantCreate,
        ListUsersResponse,
        AuthorizeResponse,
        ConnectAccountRequest,
        ForgotPasswordRequest,
        ResetPasswordRequest,
        RotatePasswordRequest,
        InviteUserRequest,
        ReInviteUserRequest,
        VerifyEmailRequest,
        SendVerifyEmailRequest,
        AcceptInviteFromEmailRequest,
        UpdateUserAccountDetailsRequest,
        GetUserDetailsResponse,
        GetUserRoleDetailsRequest,
        GetUserRoleDetailsResponse,
        GetUserRoleDetailsResponseV2,
        TokenResponse,
        TwoFactorAuthStatusResponse,
        UserFromEmailRequest,
        BeginTotpResponse,
        VerifyRecoveryCodeRequest,
        VerifyTotpRequest,
        RecoveryCodes,
        GetUserAuthenticationMethodsRequest,
        CreateUserAuthenticationMethodRequest,
        UpdateUserAuthenticationMethodRequest,
        GetSsoAuthUrlRequest,
        SsoSignInRequest,
        AuthSelectRequest,
        SendEmailRequest
    )
);

#[cfg(feature = "dummy_connector")]
common_utils::impl_api_event_type!(Miscellaneous, (SampleDataRequest));
