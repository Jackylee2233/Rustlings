// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought. No hints this time!
//
// No hints this time ;)


// Put your function here!
 fn calculate_price_of_apples(quantity:i32) -> i32 {
    let price = if quantity > 40 {
        quantity
    } else {
        quantity * 2
    };
    price
 }
// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}


/* 
Progress: [------------------------------------------------------------] 0/16 
Progress: [###>--------------------------------------------------------] 1/16 (0.0 %)
Progress: [#######>----------------------------------------------------] 2/16 (6.2 %)
Progress: [###########>------------------------------------------------] 3/16 (12.5 %)
Progress: [###############>--------------------------------------------] 4/16 (18.8 %)
Progress: [##################>-----------------------------------------] 5/16 (25.0 %)
Progress: [######################>-------------------------------------] 6/16 (31.2 %)
Progress: [##########################>---------------------------------] 7/16 (37.5 %)
Progress: [##############################>-----------------------------] 8/16 (43.8 %)
Progress: [#################################>--------------------------] 9/16 (50.0 %)
Progress: [#####################################>----------------------] 10/16 (56.2 %)
Progress: [#########################################>------------------] 11/16 (62.5 %)
Progress: [#############################################>--------------] 12/16 (68.8 %)
Progress: [################################################>-----------] 13/16 (75.0 %)
Progress: [####################################################>-------] 14/16 (81.2 %)
Progress: [########################################################>---] 15/16 (87.5 %)
 */