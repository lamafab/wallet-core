use crate::Result;
use breez_sdk_core::airgap::receive_payment;
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
        /*
        let context = receive_payment::prepare_invoice(
            req,
            lsp_info,
            node_peers,
            node_state_inbound_liquidity_msats
        );
        */

        todo!()
    }
}
