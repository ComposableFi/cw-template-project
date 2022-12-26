#![cfg(test)]

use crate::msg::{ExecuteMsg, InstantiateMsg};
use cosmwasm_orchestrate::{
    block, env, info,
    vm::{Account, JunoAddressHandler},
    JunoApi, StateBuilder, WasmLoader,
};

fn build_wasm() -> Vec<u8> {
    let mut command = std::process::Command::new("cargo");
    let _ = command.args(["wasm"]);
    WasmLoader::new(env!("CARGO_PKG_NAME"))
        .command(command)
        .load()
        .unwrap()
}

#[test]
fn run() {
    let code = build_wasm();

    let mut state = StateBuilder::new().add_code(&code).build();
    let sender = Account::generate_from_seed::<JunoAddressHandler>("sender").unwrap();

    let (contract, _) = <JunoApi>::instantiate(
        &mut state,
        1,
        None,
        block(),
        None,
        info(&sender),
        100_000_000_000,
        InstantiateMsg {},
    )
    .unwrap();

    let (_, events) = <JunoApi>::execute(
        &mut state,
        env(&contract),
        info(&sender),
        100_000_000_000,
        ExecuteMsg::Echo {
            message: "Hello, World!".into(),
        },
    )
    .unwrap();

    assert_eq!(events[1].attributes[0].value, "Hello, World!");
}
