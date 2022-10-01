pub struct Interface {
    depoosit: bool,
    withdraw: bool,
    check_balance: bool,
}

impl Interface {
    fn prompt_for_amount() -> u32 {
        println!("Please enter the amount: ");
        let amount = 300;

        return amount;
    }

    fn show_balance() -> u32 {
        let balance = 0;

        println!("Your balance is {balance} ");

        return balance;
    }

    pub fn get_user_choice() {
        // Get user choice
        println!("Deposit: \nWithdraw: \nCheck Balance: ");
        let mut user_choice = "get ser choice".to_owned();
        match user_choice.trim() {
            "deposit" => prompt_for_amount(),
            "withdraw" => prompt_for_amount(),
            "Check balance" => show_balance(),
            _ => {}
        }
    }
}
