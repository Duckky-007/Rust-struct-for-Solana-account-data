#[derive(Debug)]
struct Account {
    lamports :u64,
    data :Vec<u8>,
    owner:[u8; 32],
    executable: bool

}

impl Account {
    fn new(lamports:u64,owner:[u8;32]) -> Self {
        Self { lamports, data:Vec::new(), owner, executable:false}
    }

    fn get_lamports (&self) -> u64 {
        return self.lamports;
    }

    fn deposit (&mut self,amount:u64) {
        self.lamports += amount ;
    }  
    fn withdraw (&mut self,amount:u64) -> Result<(), String> {
        if amount>self.lamports{
            return Err("not enough lamports".to_string());
        } else {
            self.lamports -= amount ;
            return Ok(())
        }
    }  
}

fn main(){
    let mut acc = Account::new(0, [0; 32]);
    println!("{:?}", acc);

    acc.deposit(500);
    acc.get_lamports();

    println!("{}", acc.get_lamports());

    if let Err(e) = acc.withdraw(200) {

        println!("error: {}", e);
    }
    println!("{}", acc.get_lamports());
    

}