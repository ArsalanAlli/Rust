pub mod my_house {
        pub mod front_gate{
            pub fn lightstatus (status:String){
                if status == "ON" {
                    println!("Front Gate light is ON");
                }
                else if status == "OFF"{
                    println!("Front Gate light is OFF");
                }
		else {
		    println!("Front Gate light status is unkown");
			}	

            }
        }
}

