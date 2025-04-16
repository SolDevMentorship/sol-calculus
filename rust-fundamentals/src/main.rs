fn main() {
    println!("Welcome to Sol-calculus world!");

    struct Citizen {
        name: String,
        age: u8,
        address: String,
        citizenship: String,
        occupation: String,
        education: String,
        marital_status: String,
        income: f64,
        owns_property: bool,
        which_property: String,
        account_balance: i64,
        is_employee: bool,
        owns_business: bool,
        business_name: String,
    }

    let mut president = Citizen {
        name: "Sol Calculus".to_string(),
        age: 30,
        address: "123 Main St, Springfield".to_string(),
        citizenship: "Nigerian".to_string(),
        occupation: "Software Engineer".to_string(),
        education: "Bachelor's Degree".to_string(),
        marital_status: "Single".to_string(),
        income: 75000.0,
        owns_property: true,
        which_property: "100 Hectares of land".to_string(),
        account_balance: 1000000,
        is_employee: false,
        owns_business: true,
        business_name: "Sol Calculus Inc.".to_string(),
    };

    fn get_input(prompt: &str) -> Result<i64, String> {
        println!("{}", prompt);
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            return Err("Failed to read input".to_string());
        }
        match input.trim().parse::<i64>() {
            Ok(num) => Ok(num),
            Err(_) => Err("Please enter a valid number".to_string()),
        }
    }


    //atm scenario
    let atm_balance = match get_input("Enter the simulated ATM balance: ") {
        Ok(value) => value as u64,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    if president.account_balance > (0.1 * atm_balance as f64) as i64 {
        println!("You have sufficient funds for withdrawal.");
        let withdrawal_amount = match get_input("Enter the amount you want to withdraw: ") {
            Ok(value) => value as u64,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };
       
        if withdrawal_amount > atm_balance {
            println!("ATM does not have sufficient funds.");
        } else if withdrawal_amount as i64 > president.account_balance {
            println!("Account has no sufficient funds.");
        }
        president.account_balance -= withdrawal_amount as i64;
        println!("You just withdrew ${} from your account.", withdrawal_amount);
        println!("Withdrawal successful! New account balance: ${}", president.account_balance);
    } else {
        println!("Insufficient funds in your account.");
    }


    // Check if the citizen is an employee
    if president.is_employee {
        println!("{} is an employee.", president.name);
        println!("Occupation: {}", president.occupation);
    } else {
        println!("{} is not an employee.", president.name);
    }


    // Check if the citizen owns property
    if president.owns_property {
        println!("{} owns property: {}", president.name, president.which_property);
    } else {
        println!("{} does not own any property.", president.name);
    }


    // Check if the citizen owns a business
    if president.owns_business {
        println!("{} owns a business: {}", president.name, president.business_name);
        println!("----------------------------------------");
        println!("Enter amount of stock ");
        let mut stock = match get_input("Enter the stock amount: ") {
            Ok(value) => value as i64,
            Err(err) => {
            println!("Error: {}", err);
            return;
            }
        };

        println!("----------------------------------------");

        while stock > 10 || stock < 0 {
            if stock > 10 {
            println!("Stock is too much, please enter a valid amount less than or equal to 10: ");
            } else if stock < 0 {
            println!("Stock cannot be negative, please enter a valid amount: ");
            }

            println!("----------------------------------------");

            stock = match get_input("Enter stock amount: ") {
            Ok(value) => value as i64,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
            };
        }

        println!("Stock is valid, proceed with business process.");
        println!("----------------------------------------");
        println!("Business Process is .....");
        for _ in 0..200_000_000 {};

        while stock > 0 {
            stock -= 1;
            println!("Processing stock {} for business: {}", stock, president.business_name);
            for _ in 0..200_000_000 {} 
            println!("----------------------------------------");
            for _ in 0..100_000_000 {}
        }
        
        println!("All stock processed successfully.");
        println!("----------------------------------------");

    } else {
        println!("{} does not own any business.", president.name);
    }


    // Display the citizen's information
    println!("Citizen Information:");
    println!("President Name: {}", president.name);

}
