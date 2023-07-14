mod sub;

fn main() {
    // outer is a direct child of the crate
    // and public is a public module
    println!("invoke outer::public::public() from main...");
    outer::public::public();

    // outer is a direct child of the crate
    // and proxy_public is public
    println!("invoke outer::proxy_public() from main...");
    outer::proxy_public();
    // outer::private is not visible
    // outer::public::_private is not visible

    // sub is added into scope, and its child mod "module1" is made
    // public in mod.rs
    println!("invoke sub::public::public() from main...");
    sub::public::public();
    // sub::private not visible
}

mod outer {
    // this module is not accessible to outer (parent)
    // because it is private
    mod private {
        fn _private() {
            // outer::private is visible because by default
            // parent (outer) is visible to child (public)
            println!("invoke super::private() from outer::private::_private()");
            super::private();
        }
    }

    // this module is accessible to outer (parent) because it is
    // public
    pub mod public {
        pub fn public() {
            println!("invoke super::private() from outer::public::public()");
            // super::private::private is not accessible because
            // private is not visible to outer (parent of public)

            // outer::private is visible because by default
            // parent (outer) is visible to child (public)
            super::private();
        }

        fn _private() {
            println!("outer::public::_private()");
        }
    }

    // this is not visible to main even though outer is public
    // and therefor visible to main
    fn private() {
        println!("outer::private()");
    }

    pub fn proxy_public() {
        println!("invoke public::public() from outer::proxy_public");
        public::public();
    }
}
