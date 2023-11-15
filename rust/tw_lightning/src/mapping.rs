use super::Result;
use breez_sdk_core::{OpeningFeeParams, ReceivePaymentRequest};
use tw_proto::Lightning::Proto;

pub fn receive_payment_reuest_from_proto(
    proto: Proto::ReceivePaymentRequest,
) -> Result<ReceivePaymentRequest> {
    let req = proto.payment_request.unwrap();

    let preimage = if !req.preimage.is_empty() {
        Some(req.preimage.to_vec())
    } else {
        None
    };

    let opening_fee_params = if let Some(params) = req.opening_fee_params {
        Some(opening_fee_params_from_proto(params))
    } else {
        None
    };

    let r = ReceivePaymentRequest {
        amount_msat: req.amount_msat,
        description: req.description.to_string(),
        preimage,
        opening_fee_params,
        use_description_hash: Some(req.use_description_hash),
        // TODO: Should have a `use_expiry` flag.
        expiry: Some(req.expiry),
        // TODO: Should have a `use_expiry` flag.
        cltv: Some(req.cltv),
    };

    Ok(r)
}

pub fn opening_fee_params_from_proto(proto: Proto::OpeningFeeParams) -> OpeningFeeParams {
    OpeningFeeParams {
        min_msat: proto.min_msat,
        proportional: proto.proportional,
        valid_until: proto.valid_until.to_string(),
        max_idle_time: proto.max_idle_time,
        max_client_to_self_delay: proto.max_client_to_self_delay,
        promise: proto.promise.to_string(),
    }
}
