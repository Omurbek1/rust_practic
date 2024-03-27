
pub mod salam{
    pub mod adamAttar{
        pub mod turkish{
            pub fn hello(){
                println!("Merhaba");
            }
        }
        pub mod english{
            pub fn hello(){
                println!("Hello");
            }
        }
        pub mod spanish{
            pub fn hello(){
                println!("Hola");
            }
        }
        pub mod kyrgyz{
            pub fn hello(){
                println!("Salam");
            }
        }
    }
}
use salam::adamAttar::kyrgyz;
fn main() {
    kyrgyz::hello();
}
