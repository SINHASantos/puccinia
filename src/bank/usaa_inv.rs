use bank::{Bank, BankAccount};
use ofx::Ofx;

pub struct UsaaInv {
    name: String,
    username: String,
    password: String,
    accounts: Option<Vec<BankAccount>>,
}

impl UsaaInv {
    pub fn new(name: String, username: String, password: String, accounts: Option<Vec<BankAccount>>) -> UsaaInv {
        UsaaInv {
            name: name,
            username: username,
            password: password,
            accounts: accounts,
        }
    }
}

impl Bank for UsaaInv {
    fn kind(&self) -> &str {
        "usaa_inv"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_ofx<'a>(&'a self) -> Option<&'a dyn Ofx> {
        Some(self as &dyn Ofx)
    }

    fn accounts(&self) -> Result<Vec<BankAccount>, String> {
        if let Some(ref accounts) = self.accounts {
            Ok(accounts.clone())
        } else {
            self.ofx_accounts()
        }
    }
}

impl Ofx for UsaaInv {
    fn url(&self) -> &str {
        "https://service2.usaa.com/ofx/OFXServlet"
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn fid(&self) -> &str {
        "24592"
    }

    fn fid_org(&self) -> &str {
        "USAA"
    }

    fn bank_id(&self) -> &str {
        "314074269"
    }

    fn broker_id(&self) -> &str {
        "USAA.COM"
    }
}
