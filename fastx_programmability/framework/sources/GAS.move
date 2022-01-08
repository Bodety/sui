/// Coin<Gas> is the token used to pay for gas in FastX
module FastX::GAS {
    use FastX::Coin;
    use FastX::Transfer;
    use FastX::TxContext::{Self, TxContext};

    /// Name of the coin
    struct GAS has drop {}

    /// Register the token to acquire its `TreasuryCap`. Because
    /// this is a module initializer, it ensures the currency only gets
    /// registered once.
    // TODO(https://github.com/MystenLabs/fastnft/issues/90): implement module initializers
    fun init(ctx: &mut TxContext) {
        // Get a treasury cap for the coin and give it to the transaction sender
        let treasury_cap = Coin::create_currency(GAS{}, ctx);
        Transfer::transfer(treasury_cap, TxContext::get_signer_address(ctx))
    }
}