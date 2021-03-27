/* Any copyright is dedicated to the Public Domain.
http://creativecommons.org/publicdomain/zero/1.0/ */

use crate::auth::TestClient;
use crate::testing::TestGroup;
use anyhow::Result;
use autofill::{
    db::{
        store::Store as AutofillStore,
        models::credit_card::{CreditCard, UpdatableCreditCardFields},
    },
    error::Result as AutofillResult,
};

// pub fn sync_addresses(client: &mut TestClient) -> Result<()> {
//    let (init, key, _device_id) = client.data_for_sync()?;
//     client.autofill_store.sync(&init, &key, "addresses")?;
//     Ok(())
// }

pub fn sync_credit_cards(client: &mut TestClient) -> Result<()> {
   let (init, key, _device_id) = client.data_for_sync()?;
    client.autofill_store.sync(&init, &key, "creditcards", "autofill-sync-test-encryption-key")?;
    Ok(())
}

pub fn add_credit_card(s: &AutofillStore, c: UpdatableCreditCardFields) -> AutofillResult<CreditCard> {
    let id = s.add_credit_card(c)?.guid;
    Ok(s.get_credit_card(id).expect("Credit card we just added to exist"))
}

pub fn verify_credit_card(s: &AutofillStore, c: &CreditCard) {
    let equivalent = s
        .get_credit_card(c.guid.clone())
        .expect("get_credit_card() to succeed");
    assert_credit_cards_equiv(&equivalent, c);
}

pub fn assert_credit_cards_equiv(a: &CreditCard, b: &CreditCard) {
    assert_eq!(a.cc_name, b.cc_name, "cc_name mismatch");
    assert_eq!(a.cc_number_enc, b.cc_number_enc, "cc_number_enc mismatch");
    assert_eq!(a.cc_number_last_4, b.cc_number_last_4, "cc_number_last_4 mismatch");
    assert_eq!(a.cc_exp_month, b.cc_exp_month, "cc_exp_month mismatch");
    assert_eq!(a.cc_exp_year, b.cc_exp_year, "cc_exp_year mismatch");
    assert_eq!(a.cc_type, b.cc_type, "cc_type mismatch");
}

// Actual tests
fn test_autofill_credit_cards_general(c0: &mut TestClient, c1: &mut TestClient) {
    log::info!("Add some credit cards to client0");

    let cc1 = add_credit_card(
        &c0.autofill_store,
        UpdatableCreditCardFields {
            cc_name: "jane doe".to_string(),
            cc_number_enc: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
            cc_number_last_4: "1234".to_string(),
            cc_exp_month: 3,
            cc_exp_year: 2022,
            cc_type: "visa".to_string(),
        },
    )
    .expect("add cc1");

    let cc2 = add_credit_card(
        &c0.autofill_store,
        UpdatableCreditCardFields {
            cc_name: "john deer".to_string(),
            cc_number_enc: "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ".to_string(),
            cc_number_last_4: "6543".to_string(),
            cc_exp_month: 10,
            cc_exp_year: 2025,
            cc_type: "mastercard".to_string(),
        },
    )
    .expect("add cc2");


    log::info!("Syncing client0");
    sync_credit_cards(c0).expect("c0 sync to work");

    log::info!("Syncing client1");
    sync_credit_cards(c1).expect("c1 sync to work");

    log::info!("Check state");

    verify_credit_card(&c1.autofill_store, &cc1);
    verify_credit_card(&c1.autofill_store, &cc2);
}

pub fn get_test_group() -> TestGroup {
    TestGroup::new(
        "autofill",
        vec![("test_autofill_credit_cards_general", test_autofill_credit_cards_general)],
    )
}
