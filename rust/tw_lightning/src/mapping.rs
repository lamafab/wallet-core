use super::Result;
use breez_sdk_core::airgap::grpc_primitives::breez as breez_grpc;
use breez_sdk_core::airgap::receive_payment::PreparedInvoiceContext;
use breez_sdk_core::{
    LspInformation, OpeningFeeParams, OpeningFeeParamsMenu, ReceivePaymentRequest,
};
use tw_proto::Lightning::Proto;

pub fn lsp_information_from_proto(
    id: String,
    proto: breez_grpc::LspInformation,
) -> Result<LspInformation> {
    let l = LspInformation {
        id,
        name: proto.name,
        widget_url: proto.widget_url,
        pubkey: proto.pubkey,
        host: proto.host,
        channel_capacity: proto.channel_capacity,
        target_conf: proto.target_conf,
        base_fee_msat: proto.base_fee_msat,
        fee_rate: proto.fee_rate,
        time_lock_delta: proto.time_lock_delta,
        min_htlc_msat: proto.min_htlc_msat,
        lsp_pubkey: proto.lsp_pubkey,
        opening_fee_params_list: OpeningFeeParamsMenu {
            values: proto
                .opening_fee_params_list
                .into_iter()
                .map(|item| OpeningFeeParams {
                    min_msat: item.min_msat,
                    proportional: item.proportional,
                    valid_until: item.valid_until,
                    max_idle_time: item.max_idle_time,
                    max_client_to_self_delay: item.max_client_to_self_delay,
                    promise: item.promise,
                })
                .collect(),
        },
    };

    Ok(l)
}

pub fn receive_payment_request_from_proto(
    proto: Proto::ReceivePaymentParams,
) -> Result<ReceivePaymentRequest> {
    let preimage = if !proto.preimage.is_empty() {
        Some(proto.preimage.to_vec())
    } else {
        None
    };

    let opening_fee_params = if let Some(params) = proto.opening_fee_params {
        Some(opening_fee_params_from_proto(params))
    } else {
        None
    };

    let r = ReceivePaymentRequest {
        amount_msat: proto.amount_msat,
        description: proto.description.to_string(),
        preimage,
        opening_fee_params,
        use_description_hash: Some(proto.use_description_hash),
        // TODO: Should have a `use_expiry` flag.
        expiry: Some(proto.expiry),
        // TODO: Should have a `use_expiry` flag.
        cltv: Some(proto.cltv),
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

pub fn proto_receive_payment_context_from_native(
    native: PreparedInvoiceContext,
) -> Proto::ReceivePaymentContext<'static> {
    let channel_opening_fee_params = if let Some(p) = native.channel_opening_fee_params {
        Some(Proto::OpeningFeeParams {
            min_msat: p.min_msat,
            proportional: p.proportional,
            valid_until: p.valid_until.into(),
            max_idle_time: p.max_idle_time,
            max_client_to_self_delay: p.max_client_to_self_delay,
            promise: p.promise.into(),
        })
    } else {
        None
    };

    Proto::ReceivePaymentContext {
        short_channel_id: native.short_channel_id,
        destination_invoice_amount_msat: native.destination_invoice_amount_msat,
        channel_opening_fee_params,
        open_channel_needed: native.open_channel_needed,
        channel_fees_msat: native.channel_fees_msat.unwrap_or(0),
    }
}
