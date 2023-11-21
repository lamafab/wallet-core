use crate::mapping::receive_payment_context_from_proto;
use crate::{mapping, Result};
use breez_sdk_core::airgap::grpc_primitives::{breez as breez_grpc, greenlight};
use breez_sdk_core::airgap::receive_payment;
use breez_sdk_core::LspInformation;
use prost::Message;
use tw_proto::Lightning::Proto;

pub struct LightningEntry;

impl LightningEntry {
    #[inline]
    pub fn process_node_state() {
        todo!()
    }

    #[inline]
    pub fn prepare_payment_request_invoice(proto: Proto::ReceivePaymentRequest) {
        todo!()
    }

    pub fn prepare_payment_request_invoice_impl(
        proto: Proto::ReceivePaymentRequest,
    ) -> Result<Proto::ReceivePaymentContext> {
        let Proto::ReceivePaymentRequest {
            payment_request,
            blob_lsp_info,
            blob_node_peers,
            node_state_inbound_liquidity,
        } = proto;

        // Map protobuf types into the native type.
        let req = mapping::receive_payment_request_from_proto(payment_request.unwrap()).unwrap();

        // TODO: Id
        // Map the proto blob into the native type.
        let lsp_info = mapping::lsp_information_from_proto(
            String::from("id"),
            <breez_grpc::LspInformation as Message>::decode(blob_lsp_info.as_ref()).unwrap(),
        )
        .unwrap();

        // Map the proto blob into the native type.
        let proto_node_peers =
            <greenlight::ListpeersResponse as Message>::decode(blob_node_peers.as_ref()).unwrap();

        // Create and return prepared invoice context.
        let ctx = receive_payment::prepare_invoice(
            req,
            &lsp_info,
            proto_node_peers,
            node_state_inbound_liquidity,
        )
        .unwrap();

        // Map the native type into a protobuf message.
        Ok(mapping::proto_receive_payment_context_from_native(ctx))
    }

    pub fn check_lsp_hints_impl(
        proto: Proto::ReceivePaymentCheckLspHints,
    ) -> Result<Option<Proto::SignedInvoiceWithLspHint>> {
        let Proto::ReceivePaymentCheckLspHints {
            private_key,
            invoice,
            payment_request,
            context,
            // TODO: Replace prefix "proto_" with "blob_".
            proto_lsp_info,
        } = proto;

        // Map protobuf types into the native type.
        let req = mapping::receive_payment_request_from_proto(payment_request.unwrap()).unwrap();

        // Map protobuf types into the native type.
        let context = receive_payment_context_from_proto(context.unwrap());

        // TODO: Id
        // Map the proto blob into the native type.
        let lsp_info = mapping::lsp_information_from_proto(
            String::from("id"),
            <breez_grpc::LspInformation as Message>::decode(proto_lsp_info.as_ref()).unwrap(),
        )
        .unwrap();

        let try_raw_invoice =
            receive_payment::check_lsp_hints(invoice.to_string(), &req, &context, &lsp_info)
                .unwrap();

        if let Some(raw_invoice) = try_raw_invoice {
            if private_key.is_empty() {
                panic!()
            }

            let secret_key =
                bitcoin::secp256k1::SecretKey::from_slice(private_key.as_ref()).unwrap();

            let new_invoice = super::modules::sign_invoice::sign_invoice(raw_invoice, secret_key)
                .unwrap()
                .to_string();

            Ok(Some(Proto::SignedInvoiceWithLspHint {
                invoice: new_invoice.into(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn check_payment_registration_impl(
        proto: Proto::ReceivePaymentCheckRegistration,
    ) -> Result<Option<Proto::LspPaymentRegistrationParams>> {
        let Proto::ReceivePaymentCheckRegistration {
            invoice,
            payment_request,
            context,
        } = proto;

        // Map protobuf types into the native type.
        let req = mapping::receive_payment_request_from_proto(payment_request.unwrap()).unwrap();

        // Map protobuf types into the native type.
        let ctx = receive_payment_context_from_proto(context.unwrap());

        if let Some(Payment_info) =
            receive_payment::check_payment_registration(invoice.as_ref(), &req, &ctx).unwrap()
        {
            let params = mapping::proto_lsp_payment_registration_params_from_native(lsp_id, lsp_pubkey, payment_info);
        }

        todo!()
    }
}
