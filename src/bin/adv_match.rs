enum Ticket {
    Backstage(String, f32),
    Vip(String, f32),
    Standard(f32),
}

fn main() {
    let backstage_tkt = Ticket::Backstage("John".to_string(), 10.0);
    let vip_tkt = Ticket::Vip("Paul".to_string(), 15.0);
    let std_tkt = Ticket::Standard(4.0);

    let tkt_vec = [backstage_tkt, vip_tkt, std_tkt];

    for tkt in tkt_vec {
        match tkt {
            Ticket::Backstage(name, price) => {
                println!(
                    "The backstage tkt   
              issued to {}, having price {}",
                    name, price
                )
            }
            Ticket::Vip(name, price) => {
                println!(
                    "The Vip tkt was issued to {}, 
                  having price {}",
                    name, price
                )
            }

            Ticket::Standard(price) => {
                println!("The standard tkt price is {}", price)
            }
        }
    }
}
