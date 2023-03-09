#![no_std]
use gstd::{msg, ActorId, prelude::*};

#[derive(Debug, PartialEq, Eq)]
enum EscrowState {
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

struct Escrow {
    seller: ActorId,
    buyer: ActorId,
    price: u128,
    state: EscrowState,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    Deposit,
    ConfirmDelivery,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
    DeliveryConfirmed,
}

impl Escrow {
    fn deposit(&mut self) {
        assert_eq!(
            self.state,
            EscrowState::AwaitingPayment,
            "State must be `AwaitingPayment"
        );

        assert_eq!(
            msg::source(),
            self.buyer,
            "The message sender must be a buyer"
        );

        assert_eq!(
            msg::value(),
            self.price,
            "The attached value must be equal to set price"
        );

        self.state = EscrowState::AwaitingDelivery;
        msg::reply(EscrowEvent::FundsDeposited, 0)
            .expect("Error in reply `EscrowEvent::FundsDeposited");
    }

    fn confirm_delivery(&mut self) {}
}

static mut ESCROW: Option<Escrow> = None;

#[no_mangle]
extern "C" fn handle () {
    let action: EscrowAction = msg::load()
              .expect("Unable to decode `EscrowAction`");
    let escrow = unsafe { ESCROW.as_mut().unwrap() };
    match action {
        EscrowAction::Deposit => escrow.deposit(),
        EscrowAction::ConfirmDelivery => escrow.confirm_delivery(),
    }
}

#[no_mangle]
extern "C" fn init () {
    let init_config: InitEscrow = msg::load()
           .expect("Error in decoding `InitEscrow`");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment,
        //..init_config
    };
    unsafe { ESCROW = Some(escrow) };

}
